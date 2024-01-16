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

mod graphics;
mod util;

struct Trinagle {
    model: RenderableObject,
    layer: GraphicsLayer,
    rotation: f32,
}

impl Entity for Trinagle {
    fn init(&mut self) {
        self.model.texture_array.push(Texture::from_file("./crate.png"));   
    }

    fn render(&mut self) {
        self.layer.clear_screen(Color::from_hex(0xff0000ff));
        self.model.mesh.shader_program.set_uniform3f("color", Vec3::new(1.0, 1.0, 1.0));
        self.model.rotation.y = self.rotation;
        self.layer.render_object(&self.model);

        self.rotation += 0.0005;
        if self.rotation >= 360.0 {
            self.rotation = 0.0;
        }
    }

    fn update(&mut self, event_queue: &mut EventQueue, input: &mut Input) {
        
    }

    fn exit(&mut self) {
        
    }
}

struct Application;

impl EventLoopHandler for Application {
    fn init(&self, entity_manager: &mut Box<EntityManager>) {
        let view = View::new(Vec2::new(1280.0, 720.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 0.0), 45.0);
        let layer = GraphicsLayer::default_graphics_layer(view);
        let model = RenderableObject::new(Mesh::new_cube(), Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 45.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
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
    window.run(&app)
}
