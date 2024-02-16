pub mod graphics;
pub mod util;

use std::time::Duration;

use glam::{Vec2, Vec3};
use util::{entity::Entity, event::EventQueue};
use graphics::{animation::SpriteAnimation, color::Color, mesh::{AnimatedMesh, AnimatedMeshData, Mesh}, renderable::{RenderableMesh, RenderableSprite}, shader::{ShaderBuilder, ShaderFunction, ShaderProgram}, view::{GraphicsLayer, View, View2D, View3D}, window::{self, Window}};
use util::event::Input;

use crate::graphics::{shader::{ShaderSource, *}, texture::Texture};
use crate::graphics::shader::ShaderType;


fn main() {
    let mut application = Entity::new()
        .with_init(|entity| {
            let vertex_shader = ShaderBuilderTemplate::basic_vertex_shader("#version 450 core")
                .build(&ShaderType::VERTEX)
                .compile()
                .unwrap()
            ;

            let fragment_shader = ShaderBuilderTemplate::texture_fragment_shader("#version 450 core")
                .build(&ShaderType::FRAGMENT)
                .compile()
                .unwrap()
            ;

            let mut shader_program = ShaderProgram::new();
            shader_program.attach_shader(&vertex_shader);
            shader_program.attach_shader(&fragment_shader);

            shader_program.build();

            let animation_frames = vec![
                Texture::from_file("./res/Idle/Idle1.png"),
                Texture::from_file("./res/Idle/Idle2.png"),
                Texture::from_file("./res/Idle/Idle3.png"),
                Texture::from_file("./res/Idle/Idle4.png"),
                Texture::from_file("./res/Idle/Idle5.png"),
                Texture::from_file("./res/Idle/Idle6.png"),
            ];

            let sprite = RenderableSprite::new(
                &Vec2::new(800.0, 800.0),
                &shader_program,
            )
                .with_animation(&SpriteAnimation::new(&animation_frames))
                .with_position(&Vec3::new(-5.0, -200.0, 0.0))
            ;

            entity.variables.insert("sprite", sprite);
        })
        .with_render(|entity, graphics| {
            let mut sprite = entity.variables.take_out::<RenderableSprite>("sprite");

            graphics.render_object(&mut sprite);

            entity.variables.insert("sprite", sprite);
        })
        .with_update(|entity, event_queue, input, delta| {
            let mut sprite = entity.variables.take_out::<RenderableSprite>("sprite");

            sprite.animate(delta, &Duration::from_secs_f32(0.5));

            entity.variables.insert("sprite", sprite);
        })
    ;

    let view = View::View2D(
        View2D::new(Vec2::new(1920.0, 1080.0))
    );
    let graphics = GraphicsLayer::new(&view);

    let mut window = Window::new("Rustler", &graphics).unwrap();

    window.run(&mut application, 20, 0);
}