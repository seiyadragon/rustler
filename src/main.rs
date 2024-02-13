pub mod graphics;
pub mod util;

use std::time::Duration;

use glam::{Vec2, Vec3};
use util::{entity::Entity, event::EventQueue};
use graphics::{mesh::{AnimatedMesh, AnimatedMeshData, Mesh}, renderable::RenderableMesh, shader::{ShaderBuilder, ShaderFunction, ShaderProgram}, view::{GraphicsLayer, View, View2D, View3D}, window::{self, Window}};
use util::event::Input;

use crate::graphics::{shader::{ShaderSource, *}, texture::Texture};
use crate::graphics::shader::ShaderType;


fn main() {
    let mut entry = Entity::new()
        .with_init(|entity: &mut Entity| {
            let obj = Entity::new()
                .with_init(|entity: &mut Entity| {       
                    let vertex_builder = ShaderBuilderTemplate::animated_vertex_shader("#version 450 core");
                    println!("{}", vertex_builder);

                    let fragment_builder = ShaderBuilderTemplate::lighting_fragment_shader("#version 450 core");
                    println!("{}", fragment_builder);

                    let built_vertex = vertex_builder.build(&ShaderType::VERTEX);
                    let compiled_vertex = built_vertex.compile().unwrap();

                    let build_fragment = fragment_builder.build(&ShaderType::FRAGMENT);
                    let compiled_fragment = build_fragment.compile().unwrap();

                    let mut program = ShaderProgram::new();
                    program.attach_shader(&compiled_vertex);
                    program.attach_shader(&compiled_fragment);
                    program.build();

                    let mesh = RenderableMesh::new(
                        Mesh::AnimatedMesh(
                            AnimatedMeshData::from_collada("./res/model.dae")
                            .build(&program)
                        )
                    )
                        .with_position(&Vec3::new(0.0, -10.0, 15.0))
                        .with_texture(&Texture::from_file("./res/model_texture.png"))
                    ;

                    entity.variables.insert("mesh", mesh);
                })
                .with_render(|entity: &mut Entity, graphics: &mut GraphicsLayer| {
                    let mut mesh = entity.variables.take_out::<RenderableMesh>("mesh");

                    mesh.rotate_by(&Vec3::new(0.0, 1.0, 0.0));

                    graphics.render_object(&mut mesh);

                    entity.variables.insert("mesh", mesh);
                })
                .with_update(|entity: &mut Entity, event_queue: &mut EventQueue, input: &mut Input, delta: &Duration| {
                    let mut mesh = entity.variables.take_out::<RenderableMesh>("mesh");

                    mesh.get_animated_mesh().unwrap().animation_player.animate(delta, &Duration::from_secs_f32(1.0));

                    entity.variables.insert("mesh", mesh);
                })
            ;

            entity.push(obj);
        })
    ;

    let view = View::View3D(View3D::new(Vec2::new(1920.0, 1080.0)));
    let graphics = GraphicsLayer::new(&view);
    let window = Window::new("Rustler", &graphics);

    window.unwrap().run(&mut entry, 20, 0);
}