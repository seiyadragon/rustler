use graphics::math::Deg;
use graphics::shader;
use winit::{event::*, keyboard::*};
use glam::*;
use graphics::color::*;
use graphics::renderable::*;
use graphics::view::*;
use graphics::window::*;
use graphics::shader::*;
use graphics::texture::*;
use util::entity::*;
use graphics::mesh::*;
use rand::Rng;

mod graphics;
mod util;

struct Model {
    renderable: RenderableObject,
}

impl Entity for Model {
    fn init(&mut self) {
        //self.renderable.texture_array.push(Texture::from_file("./res/crate.png"));
    }

    fn render(&mut self, graphics: &mut GraphicsLayer) {
        graphics.render_object(&mut self.renderable);
    }

    fn update(&mut self, event_queue: &mut EventQueue, input: &mut Input) {
        self.renderable.rotation.x += 1.0;
    }

    fn exit(&mut self) {
        
    }
}

struct Application;

impl EventLoopHandler for Application {
    fn init(&self, entity_manager: &mut Box<EntityManager>) {
        let shader_program = ShaderProgram::default_shader_program();
        let animated_mesh_data = AnimatedMeshData::from_collada("./res/model.dae");
        let mut animated_mesh = animated_mesh_data.build(&shader_program);

        animated_mesh.skeleton.apply_keyframe(&animated_mesh.animations[0].key_frames[3]);

        let model = Model {
            renderable: RenderableObject::new(
                Mesh::AnimatedMesh(
                    animated_mesh
                ),
                &Vec3::new(0.0, 0.0, 0.0), 
                &Vec3::new(0.0, 0.0, 0.0), 
                &Vec3::new(1.0, 1.0, 1.0)
            )
        };

        entity_manager.push(Box::new(model));
    }

    fn render(&self, entity_manager: &mut Box<EntityManager>, graphics: &mut GraphicsLayer) {
        //graphics.clear_screen(Color::from_hex(0x00000000));
    }

    fn update(&self, entity_manager: &mut Box<EntityManager>, event_queue: &mut EventQueue, input: &mut Input) {
        event_queue.ignore_events();
    }

    fn exit(&self, entity_manager: &mut Box<EntityManager>) {
        
    }
}

fn main() {
    let graphics = GraphicsLayer::default_graphics_layer(View::new(
        Vec2::new(1280.0/2.0, 720.0/2.0), 
        Vec3::new(0.0, 0.0, -20.0), 
        Vec3::new(0.0, 0.0, 1.0), 
        Vec3::new(0.0, 1.0, 0.0), 
        45.0,
    ));

    let app = Application{};
    let window = Window::new("Rustler", 1280/2, 720/2, &graphics).unwrap();
    window.run_at_20_ticks_with_frames(&app, 20);
    //window.run_at_20_ticks_with_frames(&app, 2000);
}
