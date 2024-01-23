use winit::{event::*, keyboard::*};
use glm::*;
use graphics::color::*;
use graphics::renderable::*;
use graphics::view::*;
use graphics::window::*;
use graphics::shader::*;
use graphics::texture::*;
use util::entity::*;
use graphics::mesh::*;

use crate::graphics::mesh;

mod graphics;
mod util;

struct Trinagle {
    model: RenderableObject,
    layer: GraphicsLayer,
    rotation: f32,
    model2: RenderableObject,
}

impl Entity for Trinagle {
    fn init(&mut self) {
        //self.model.texture_array.push(Texture::from_file("./hexagon.jpg"));
        //self.model.texture_array.push(Texture::from_file("./crate.png"));

        let color_buffer = ColorBuffer::new(512, 512, &Color::from_hex(0xffff00ff));
        let purp = color_buffer.build_texture();

        let crate_color_buffer = ColorBuffer::from_file("./hexagon.jpg");
        let crate_tex = Texture::from_color_buffer(&crate_color_buffer);

        let crate_buffer = crate_tex.delete();
        let crate_tex_2 = crate_buffer.build_texture();

        self.model.texture_array.push(crate_tex_2);
    }

    fn render(&mut self) {
        self.layer.clear_screen(Color::from_hex(0xff0000ff));
        self.model.mesh.shader_program.set_uniform_vec3_f32("color", Color::from_hex(0xffffffff).to_vec3());
        self.layer.render_object(&self.model);
        self.layer.render_object(&self.model2);
    }

    fn update(&mut self, event_queue: &mut EventQueue, input: &mut Input) {
        self.rotation += 0.01;
        if self.rotation >= 360.0 {
            self.rotation = 0.0;
        }
        self.model.rotation = Vec3::new(self.rotation, self.rotation, self.rotation);
        self.model2.rotation = Vec3::new(self.rotation, self.rotation, self.rotation);
    }

    fn exit(&mut self) {
        
    }
}

struct Application;

impl EventLoopHandler for Application {
    fn init(&self, entity_manager: &mut Box<EntityManager>) {
        let view = View::new(Vec2::new(1280.0/2.0, 720.0/2.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 0.0), 45.0);
        let layer = GraphicsLayer::default_graphics_layer(view);

        let mut built_mesh = MeshData::generate_from_collada("./cube.dae").build_mesh(&ShaderProgram::default_shader_program());

        let model = RenderableObject::new(built_mesh, &Vec3::new(0.0, 0.0, 10.0), &Vec3::new(0.0, 0.0, 0.0), &Vec3::new(1.0, 1.0, 1.0));
        let rot_tracker = 0.0;

        let model2 = RenderableObject::new(Mesh::new_cube(), &Vec3::new(3.0, 0.0, 10.0), &Vec3::new(0.0, 0.0, 0.0), &Vec3::new(1.0, 1.0, 1.0));

        let triangle = Trinagle {
            layer: layer,
            model: model,
            model2: model2,
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
    //window.run_at_20_ticks_with_frames(&app, 2000);
}
