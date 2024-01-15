use std::{ptr::{null, self}, ffi::CString};

use glm::{Vec3, Vec2};
use graphics::{window::{Window, EventLoopHandler, EventQueue, Input}, mesh::Mesh, vertex::{self, VAO, VBO, IBO, Vertex}, shader, view::{GraphicsLayer, self, View}, renderable::Model};
use winit::{event::*, keyboard::*};
use util::entity::*;
use graphics::shader::*;

mod graphics;
mod util;

struct Trinagle {
    model: Model,
    layer: GraphicsLayer,
}

impl Entity for Trinagle {
    fn init(&mut self) {
        //let shader_program = ShaderProgram::default_shader_program();   
    }

    fn render(&mut self) {
        self.model.model_mesh.shader_program.set_uniform3f("color", Vec3::new(0.0, 1.0, 0.0));
        self.layer.render_object(&self.model);
    }

    fn update(&mut self, event_queue: &mut EventQueue, input: &mut Input) {
        
    }

    fn exit(&mut self) {
        
    }
}

struct Application;

impl EventLoopHandler for Application {
    fn init(&self, entity_manager: &mut Box<EntityManager>) {
        let view = View::new(Vec2::new(1280.0/2.0, 720.0/2.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 0.0), 45.0);
        let layer = GraphicsLayer::default_graphics_layer(view);
        let model = Model::new(Mesh::new_cube(), Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 45.0, 0.0), Vec3::new(1.0, 1.0, 1.0));

        let triangle = Trinagle {
            layer: layer,
            model: model,
        };

        entity_manager.push(Box::from(triangle));
    }

    fn render(&self, entity_manager: &mut Box<EntityManager>) {
        
    }

    fn update(&self, entity_manager: &mut Box<EntityManager>, event_queue: &mut EventQueue, input: &mut Input) {
        event_queue.process_events(move|event| {
            match event {
                _ => {},
            }
        });

        if input.was_key_just_pressed(KeyCode::Space) {
            println!("Space was just pressed.");
        }

        if input.was_key_just_released(KeyCode::Space) {
            println!("Space was just released.");
        }

        if input.is_key_being_held_down(KeyCode::KeyE) {
            println!("E is being held down.");
        }

        if input.was_button_just_pressed(MouseButton::Left) {
            println!("Left mouse button just pressed");
        }

        if input.was_button_just_released(MouseButton::Left) {
            println!("Left mouse button just released");
        }

        if input.is_button_being_held(MouseButton::Right) {
            println!("Right mouse button is held");
        }
    }

    fn exit(&self, entity_manager: &mut Box<EntityManager>) {
        
    }
}

fn main() {
    let app = Application{};
    let window = Window::new("Rustler", 1280/2, 720/2).unwrap();
    window.run(&app)
}
