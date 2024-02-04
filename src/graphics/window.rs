use glutin::config::Config;
use glutin::surface::WindowSurface;
use winit::dpi::{LogicalSize, PhysicalSize};
use raw_window_handle::HasRawWindowHandle;
use winit::event::ElementState;
pub use winit::event::{Event, KeyEvent, WindowEvent};
use winit::platform::scancode::PhysicalKeyExtScancode;
use winit::window::WindowBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, Version, PossiblyCurrentContext};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin_winit::{self, GlWindow};
use std::error::Error;
use glutin::context::NotCurrentContext;
use std::ffi::CString;
use glutin::surface::Surface;
use crate::util::event::{EventQueue, Input};
use crate::{Entity, GraphicsLayer, View, View2D, View3D};
use std::time::{Duration, Instant};

use super::color::Color;
use super::view;

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
    pub current_frames_per_second: u64,
    pub current_ticks_per_second: u64,
    pub default_graphics_layer: GraphicsLayer,
}

impl Window {
    pub fn new(title: &str, default_graphics_layer: &GraphicsLayer) -> Result<Self, Box<dyn Error>> {
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        let width = match &default_graphics_layer.view {
            View::View2D(view) => view.size.x as u32,
            View::View3D(view) => view.size.x as u32,
        };

        let height = match &default_graphics_layer.view {
            View::View2D(view) => view.size.y as u32,
            View::View3D(view) => view.size.y as u32,
        };

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
            current_frames_per_second: 0,
            current_ticks_per_second: 0,
            default_graphics_layer: default_graphics_layer.clone(),
        })
    }

    pub fn run(mut self, loop_handler: &mut Entity, target_ticks_per_second: u64, target_frames_per_second: u64) {
        let mut last_tick_time = Instant::now();
        let mut last_frame_time = Instant::now();
        let mut tick_lag: f64 = 0.0;
        let mut frame_lag: f64 = 0.0;
        
        let mut frames: u64 = 0;
        let mut ticks: u64 = 0;

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
                    Window::clear_screen(Color::new(165, 93, 63, 255));

                    self.internal_window = Some(window);
                    self.is_context_current = true;

                    unsafe {
                        gl::Enable(gl::DEPTH_TEST);
                        gl::DepthFunc(gl::LESS);

                        gl::Enable(gl::CULL_FACE);
                        gl::CullFace(gl::BACK);
                    }

                    loop_handler.init();
                },
                winit::event::Event::AboutToWait => {
                    let tick_elapsed = last_tick_time.elapsed();
                    last_tick_time = Instant::now();
                    tick_lag += tick_elapsed.as_millis() as f64;

                    let frame_elapsed = last_frame_time.elapsed();
                    last_frame_time = Instant::now();
                    frame_lag += frame_elapsed.as_millis() as f64;

                    if ticks >= target_ticks_per_second {
                        self.current_frames_per_second = frames;
                        self.current_ticks_per_second = ticks;

                        println!("FPS: {0} | TPS: {1}", self.current_frames_per_second, self.current_ticks_per_second);
                
                        frames = 0;
                        ticks = 0;
                    }

                    while tick_lag >= 1000.0 / (target_ticks_per_second as f64) {
                        loop_handler.update(&mut self.event_queue, &mut self.input, &Duration::from_millis(tick_lag as u64));
                        self.input.update();

                        ticks += 1;
                        tick_lag -= 1000.0 / (target_ticks_per_second as f64);
                    }

                    if target_frames_per_second > 0 && target_frames_per_second < 320 {
                        while frame_lag >= 1000.0 / (target_frames_per_second as f64) {
                            self.internal_window.as_ref().unwrap().request_redraw();

                            frames += 1;
                            frame_lag -= 1000.0 / (target_frames_per_second as f64);
                        }
                    } else {
                        self.internal_window.as_ref().unwrap().request_redraw();

                        frames += 1;
                        frame_lag -= 1000.0 / (target_frames_per_second as f64);
                    }
                },
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::RedrawRequested => {
                        Window::clear_screen(Color::new(165, 93, 63, 255));
                        loop_handler.render(&mut self.default_graphics_layer);
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
                        loop_handler.exit();
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

    pub fn clear_screen(color: Color) {
        let color_vec = color.to_vec4();

        unsafe {
            gl::ClearColor(color_vec.x, color_vec.y, color_vec.z, color_vec.w);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
}