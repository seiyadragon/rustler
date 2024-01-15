use glutin::config::Config;
use glutin::surface::WindowSurface;
use winit::dpi::{LogicalSize, PhysicalSize};
use raw_window_handle::HasRawWindowHandle;
use winit::event::{ElementState, MouseButton};
pub use winit::event::{Event, KeyEvent, WindowEvent};
pub use winit::keyboard::{KeyCode, PhysicalKey};
use winit::platform::scancode::PhysicalKeyExtScancode;
use winit::window::WindowBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, Version, PossiblyCurrentContext};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin_winit::{self, GlWindow};
use std::collections::VecDeque;
use std::error::Error;
use glutin::context::NotCurrentContext;
use std::ffi::CString;
use glutin::surface::Surface;
use crate::util::entity::EntityManager;

extern crate gl;

pub struct Window {
    pub event_loop: winit::event_loop::EventLoop<()>,
    pub internal_window: Option<winit::window::Window>,
    pub gl_config: Config,
    pub not_current_context: Option<NotCurrentContext>,
    pub current_context: Option<PossiblyCurrentContext>,
    pub is_context_current: bool,
    pub context_surface: Option<Surface<WindowSurface>>,
    pub title: String,
    pub size: PhysicalSize<u32>,
    pub event_queue: EventQueue,
    pub input: Input,
    pub entity_manager: Box<EntityManager>,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, Box<dyn Error>> {
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        let window_builder = winit::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(width, height))
            .with_active(true)
            .with_visible(true);

        let template = glutin::config::ConfigTemplateBuilder::new().with_alpha_size(8).with_transparency(cfg!(cgl_backend));
        let display_builder = glutin_winit::DisplayBuilder::new().with_window_builder(Some(window_builder));

        let (window, gl_config) = display_builder.build(&event_loop, template, |configs| {
            configs.reduce(|accum, config| {
                let transparency_check = config.supports_transparency().unwrap_or(false)
                    & !accum.supports_transparency().unwrap_or(false);

                if transparency_check || config.num_samples() > accum.num_samples() {
                    config
                } else {
                    accum
                }
            }).unwrap()
        })?;

        let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());
        let gl_display = gl_config.display();
        let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(raw_window_handle);

        let legacy_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
            .build(raw_window_handle);

        let not_current_gl_context = Some(unsafe {
            gl_display.create_context(&gl_config, &context_attributes).unwrap_or_else(|_| {
                gl_display.create_context(&gl_config, &fallback_context_attributes).unwrap_or_else(
                    |_| {
                        gl_display
                            .create_context(&gl_config, &legacy_context_attributes)
                            .expect("failed to create context")
                    },
                )
            })
        });

        Ok(Window {
            event_loop: event_loop,
            internal_window: window,
            gl_config: gl_config,
            not_current_context: not_current_gl_context,
            current_context: None,
            is_context_current: false,
            title: String::from(title),
            size: PhysicalSize::new(width, height),
            context_surface: None,
            event_queue: EventQueue::new(),
            input: Input::new(),
            entity_manager: Box::from(EntityManager::new()),
        })
    }

    pub fn run(mut self, loop_handler: &dyn EventLoopHandler) {
        self.event_loop.run(move |event, elwt| {
            self.event_queue.push(event.clone());

            match event {
                Event::Resumed => {
                    let window = self.internal_window.take().unwrap_or_else(|| {
                        let window_builder = WindowBuilder::new()
                            .with_title(self.title.clone())
                            .with_inner_size(self.size)
                            .with_active(true)
                            .with_visible(true);
                        glutin_winit::finalize_window(elwt, window_builder, &self.gl_config)
                            .unwrap()
                    });

                    let attrs = window.build_surface_attributes(Default::default());
                    let gl_surface = unsafe {
                        self.gl_config.display().create_window_surface(&self.gl_config, &attrs).unwrap()
                    };

                    self.context_surface = Some(gl_surface);
                    
                    let gl_context = self.not_current_context.take().unwrap().make_current(self.context_surface.as_ref().unwrap()).unwrap();
                    self.current_context = Some(gl_context);
                
                    gl::load_with(|s| self.gl_config.display().get_proc_address(CString::new(s).unwrap().as_c_str()) as *const _);
                    Window::clear_screen(165, 93, 63);

                    self.internal_window = Some(window);
                    self.is_context_current = true;

                    loop_handler.init(&mut self.entity_manager);
                },
                winit::event::Event::AboutToWait => {
                    loop_handler.update(&mut self.entity_manager, &mut self.event_queue, &mut self.input);
                    self.input.update();
                    self.entity_manager.update(&mut self.event_queue, &mut self.input);
                    self.internal_window.as_ref().unwrap().request_redraw();
                },
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::RedrawRequested => {
                        Self::clear_screen(165, 93, 63);
                        loop_handler.render(&mut self.entity_manager);
                        self.entity_manager.render();
                        self.context_surface.as_ref().unwrap().swap_buffers(self.current_context.as_ref().unwrap()).unwrap();
                    },
                    WindowEvent::Resized(size) => {
                        self.size = size;
                        unsafe {
                            gl::Viewport(0, 0, self.size.width as i32, self.size.height as i32);
                        }
                    },
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                        loop_handler.exit(&mut self.entity_manager);
                    },
                    WindowEvent::KeyboardInput { event, .. } => match event {
                        KeyEvent { physical_key, logical_key: _, text: _, location: _, state, repeat, .. } => {
                            match state {
                                ElementState::Pressed => {
                                    if repeat == false {
                                        self.input.set_key_pressed(physical_key.to_scancode().unwrap());
                                    }
                                },
                                ElementState::Released => {
                                    if repeat == false {
                                        self.input.set_key_unpressed(physical_key.to_scancode().unwrap());
                                    }
                                }
                            }
                        }
                    },
                    WindowEvent::MouseInput { device_id: _, state, button } => {
                        match state {
                            ElementState::Pressed => {
                                self.input.set_button_pressed(Input::mouse_button_to_index(button));
                            },
                            ElementState::Released => {
                                self.input.set_button_unpressed(Input::mouse_button_to_index(button));
                            }
                        }
                    },
                    _ => ()
                },
                _ => ()
            }
        }).unwrap();
    }

    pub fn clear_screen(r: u16, g: u16, b: u16) {
        unsafe {
            gl::ClearColor(r as f32 / 256.0, g as f32 / 256.0, b as f32 / 256.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
}

pub trait EventLoopHandler {
    fn init(&self, entity_manager: &mut Box<EntityManager>);
    fn render(&self, entity_manager: &mut Box<EntityManager>);
    fn update(&self, entity_manager: &mut Box<EntityManager>, event_queue: &mut EventQueue, input: &mut Input);
    fn exit(&self, entity_manager: &mut Box<EntityManager>);
}

pub struct EventQueue {
    pub internal_queue: VecDeque<Event<()>>,
}

impl EventQueue {
    pub fn new() -> Self {
        EventQueue {
            internal_queue: VecDeque::new(),
        }
    }

    pub fn process_events(&mut self, func: fn(event: Event<()>)) {
        for event in self.internal_queue.clone() {
            func(event);

            self.internal_queue.pop_front();
        }

        self.internal_queue = VecDeque::<Event<()>>::new();
    }

    pub fn push(&mut self, event: Event<()>) {
        self.internal_queue.push_back(event);
    }

    pub fn len(&self) -> usize {
        self.internal_queue.len()
    }
}

pub struct Input {
    pub keys: [bool; 255],
    pub keys_last: [bool; 255],
    pub buttons: [bool; 32],
    pub buttons_last: [bool; 32],
}

impl Input {
    pub fn new() -> Self {
        let keys: [bool; 255] = [false; 255];
        let keys_last: [bool; 255] = [false; 255];

        let buttons: [bool; 32] = [false; 32];
        let buttons_last: [bool; 32] = [false; 32];

        Input {
            keys: keys,
            keys_last: keys_last,
            buttons: buttons,
            buttons_last: buttons_last,
        }
    }

    pub fn set_key_pressed(&mut self, scancode: u32) {
        self.keys[scancode as usize] = true;
    }

    pub fn set_key_unpressed(&mut self, scancode: u32) {
        self.keys[scancode as usize] = false;
    }

    pub fn set_button_pressed(&mut self, button: u32) {
        self.buttons[button as usize] = true;
    }

    pub fn set_button_unpressed(&mut self, button: u32) {
        self.buttons[button as usize] = false;
    }

    pub fn update(&mut self) {
        for i in 0..self.keys.len() {
            self.keys_last[i] = self.keys[i];
        }

        for i in 0..self.buttons.len() {
            self.buttons_last[i] = self.buttons[i];
        }
    }

    pub fn key_to_scancode(key: KeyCode) -> u32 {
        PhysicalKey::Code(key).to_scancode().unwrap()
    }

    pub fn mouse_button_to_index(button: MouseButton) -> u32 {
        match button {
            MouseButton::Left => { 0 },
            MouseButton::Right => { 1 },
            MouseButton::Middle => { 2 },
            MouseButton::Back => { 3 },
            MouseButton::Forward => { 4 },
            MouseButton::Other(num) => { num as u32 },
        }
    }

    pub fn was_key_just_pressed(&self, key: KeyCode) -> bool {
        self.keys[Self::key_to_scancode(key) as usize] && !self.keys_last[Self::key_to_scancode(key) as usize]
    }

    pub fn was_key_just_released(&self, key: KeyCode) -> bool {
        !self.keys[Self::key_to_scancode(key) as usize] && self.keys_last[Self::key_to_scancode(key) as usize]
    }

    pub fn is_key_being_held_down(&self, key: KeyCode) -> bool {
        self.keys[Self::key_to_scancode(key) as usize]
    }

    pub fn was_button_just_pressed(&self, button: MouseButton) -> bool {
        self.buttons[Self::mouse_button_to_index(button) as usize] && !self.buttons_last[Self::mouse_button_to_index(button) as usize]
    }

    pub fn was_button_just_released(&self, button: MouseButton) -> bool {
        !self.buttons[Self::mouse_button_to_index(button) as usize] && self.buttons_last[Self::mouse_button_to_index(button) as usize]
    }

    pub fn is_button_being_held(&self, button: MouseButton) -> bool {
        self.buttons[Self::mouse_button_to_index(button) as usize]
    }
}

