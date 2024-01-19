use winit::{event::*, keyboard::*};
use glm::*;
use graphics::color::*;
use graphics::renderable::*;
use graphics::view::*;
use graphics::window::*;
use graphics::shader::*;
use graphics::texture::*;
use graphics::vertex::*;
use util::entity::*;
use graphics::mesh::*;
use image::io::Reader;

use crate::graphics::mesh;

mod graphics;
mod util;

struct Trinagle {
    model: RenderableObject,
    layer: GraphicsLayer,
    rotation: f32,
}

impl Entity for Trinagle {
    fn init(&mut self) {
        self.model.texture_array.push(Texture::from_file("./hexagon.jpg"));
        self.model.texture_array.push(Texture::from_file("./crate.png"));
    }

    fn render(&mut self) {
        self.layer.clear_screen(Color::from_hex(0xff0000ff));
        self.model.mesh.shader_program.set_uniform_vec3_f32("color", Color::from_hex(0xffffffff).to_vec3());
        self.model.rotation = Vec3::new(self.rotation, self.rotation, self.rotation);
        self.layer.render_object(&self.model);
    }

    fn update(&mut self, event_queue: &mut EventQueue, input: &mut Input) {
        self.rotation += 0.01;
        if self.rotation >= 360.0 {
            self.rotation = 0.0;
        }
    }

    fn exit(&mut self) {
        
    }
}

struct Application;

impl EventLoopHandler for Application {
    fn init(&self, entity_manager: &mut Box<EntityManager>) {
        let view = View::new(Vec2::new(1280.0/2.0, 720.0/2.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 0.0), 45.0);
        let layer = GraphicsLayer::default_graphics_layer(view);

        let mut built_mesh = Mesh::new_pyramid();

        let mesh_data_2 = built_mesh.delete();
        let mesh_data_2_clone = mesh_data_2.clone();
        let shader = ShaderProgram::default_shader_program();

        let built_mesh_2 = mesh_data_2.build_mesh(&shader);

        let model = RenderableObject::new(built_mesh, &Vec3::new(0.0, 0.0, 5.0), &Vec3::new(0.0, 45.0, 0.0), &Vec3::new(1.0, 1.0, 1.0));
        let rot_tracker = 0.0;

        let triangle = Trinagle {
            layer: layer,
            model: model,
            rotation: rot_tracker,
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
    let color = Color::from_hex(0x00ff00ff);
    println!("{color}");
    println!("{:#08x}", color.to_hex());

    let app = Application{};
    let window = Window::new("Rustler", 1280/2, 720/2).unwrap();
    window.run_at_20_ticks_with_frames(&app, 20);
}
