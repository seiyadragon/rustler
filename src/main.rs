
use std::time::Duration;

use graphics::renderable;
use graphics::vertex::Vertex;
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
                    /*entity.variables.insert("renderable", RenderableMesh::new(
                        Mesh::StaticMesh(
                            StaticMeshData::new(
                                &vec![
                                    Vertex::new(&Vec3::new(-0.5, -0.5, 0.0), &Vec3::new(0.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, 1.0)),
                                    Vertex::new(&Vec3::new(0.5, -0.5, 0.0), &Vec3::new(1.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, 1.0)),
                                    Vertex::new(&Vec3::new(0.5, 0.5, 0.0), &Vec3::new(1.0, 1.0, 0.0), &Vec3::new(0.0, 0.0, 1.0)),
                                    Vertex::new(&Vec3::new(-0.5, 0.5, 0.0), &Vec3::new(0.0, 1.0, 0.0), &Vec3::new(0.0, 0.0, 1.0)),
                                ],
                                &vec![
                                    0, 1, 2,
                                    2, 3, 0,
                                ]
                            ).build(&ShaderProgram::default_shader_program())
                        ),
                    )
                        .with_position(&Vec3::new(0.0, 0.0, 0.0))
                        .with_scale(&Vec3::new(100.0, 100.0, 1.0))
                    );*/
                    entity.variables.insert("renderable", RenderableSprite::new(&Vec2::new(200.0, 200.0)));
                    entity.variables.insert("velocity", Vec2::new(0.0, 0.0));
                    entity.variables.insert("jumping", false);
                })
                .with_render(|entity, graphics| {
                    //let mut renderable = entity.variables.take_out::<RenderableMesh>("renderable");  
                    let mut renderable = entity.variables.take_out::<RenderableSprite>("renderable");                    

                    graphics.render_object(&mut renderable);

                    entity.variables.insert("renderable", renderable);
                })
                .with_update(|entity, event_queue, input, delta| {
                    //let mut renderable = entity.variables.take_out::<RenderableMesh>("renderable");  
                    let mut renderable = entity.variables.take_out::<RenderableSprite>("renderable");
                    let mut velocity = entity.variables.take_out::<Vec2>("velocity");
                    let mut jumping = entity.variables.take_out::<bool>("jumping");

                    if !jumping && velocity.y > -9.8 {
                        velocity.y -= 0.1;
                    }

                    if input.was_key_just_pressed(KeyCode::KeyW) && !jumping {
                        velocity.y = 3.0;
                        jumping = true;
                    }

                    if renderable.position.y <= -490.0 {
                        renderable.position.y = -490.0;
                        jumping = false;
                    }

                    if input.was_key_just_pressed(KeyCode::KeyA) {
                        velocity.x = -5.0;
                    } else if input.was_key_just_released(KeyCode::KeyA) {
                        velocity.x = 0.0;
                    }

                    if input.was_key_just_pressed(KeyCode::KeyD) {
                        velocity.x = 5.0;
                    } else if input.was_key_just_released(KeyCode::KeyD) {
                        velocity.x = 0.0;
                    }

                    renderable.position.x += velocity.x;
                    renderable.position.y += velocity.y;

                    entity.variables.insert("renderable", renderable);
                    entity.variables.insert("velocity", velocity);
                    entity.variables.insert("jumping", jumping);

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
            Vec2::new(1920.0, 1080.0)
        )
    );
    let graphics = GraphicsLayer::new(&view);
    let window = Window::new("Rustler", &graphics).unwrap();

    window.run(&mut application, 60, 0);
}