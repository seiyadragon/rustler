
use std::time::Duration;

use graphics::renderable;
use graphics::view;
use util::entity;
use util::event::EventQueue;
use util::event::Input;
use glam::*;
use graphics::color::*;
use graphics::renderable::*;
use graphics::view::*;
use graphics::window::*;
use graphics::shader::*;
use graphics::texture::*;
use util::entity::*;
use graphics::mesh::*;
use winit::keyboard::KeyCode;
mod graphics;
mod util;


fn main() {
    let mut application = Entity::new()
        .with_init(|entity| {
            let model = Entity::new()
                .with_init(|entity| {
                    let renderable = entity.variables.declare("renderable", RenderableMesh::new(
                        Mesh::AnimatedMesh(AnimatedMeshData::from_collada("./res/model.dae").build(&ShaderProgram::default_shader_program()))
                    )
                    .with_position(&Vec3::new(0.0, -5.0, 15.0))
                    .with_rotation(&Vec3::new(0.0, 90.0, 0.0)));

                    //renderable.push_texture(&Texture::from_file("./res/model_texture.png"));
                })
                .with_render(|entity, graphics| {
                    let renderable = entity.variables.get::<RenderableMesh>("renderable");
                    renderable.rotate_by(&Vec3::new(0.0, 0.05, 0.0));

                    graphics.render_object(renderable);
                })
                .with_update(|entity, event_queue, input, delta| {
                    let renderable = entity.variables.get::<RenderableMesh>("renderable");

                    renderable.get_animated_mesh().unwrap().animation_player.animate(delta, &Duration::from_secs_f32(2.5));

                    if input.was_key_just_pressed(KeyCode::Space) {
                        renderable.get_animated_mesh().unwrap().animation_player.pause_to_pose(&Duration::from_secs_f32(0.0));
                    }

                    if input.was_key_just_released(KeyCode::Space) {
                        renderable.get_animated_mesh().unwrap().animation_player.toggle_pause();
                    }
                })
                .with_exit(|entity| {
                    
                })
            ;

            entity.push(model);
        })
        .with_render(|entity, graphics| {
            
        })
        .with_update(|entity, event_queue, input, delta| {
            
        })
        .with_exit(|entity| {
            
        })
    ;

    let view = View::View2D(
        View2D::new(
            Vec2::new(800.0, 600.0)
        )
    );
    let graphics = GraphicsLayer::new(&view);
    let window = Window::new("Rustler", &graphics).unwrap();

    window.run(&mut application, 60, 0);
}