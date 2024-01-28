use std::ptr::null_mut;

use crate::Color;
use crate::ColorBuffer;
use crate::Texture;

use super::color;
use super::mesh::Mesh;
use super::view::GraphicsLayer;
use glm::Vec2;
use glm::Vec3;
use glm::Vec4;
use glm::Mat4;
use super::math::MatrixBuilder;

use glm::Vector2;

pub trait Renderable {
    fn render(&self, layer: &GraphicsLayer);
    fn get_model_matrix(&self) -> Mat4;
}

pub struct RenderableObject {
    pub mesh: Mesh,
    pub texture_array: Vec<Texture>,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl RenderableObject {
    pub fn new(mesh: Mesh, position: &Vec3, rotation: &Vec3, scale: &Vec3) -> Self {
        RenderableObject {
            mesh: mesh,
            texture_array: Vec::new(),
            position: position.clone(),
            rotation: rotation.clone(),
            scale: scale.clone(),
        }
    }
}

impl Renderable for RenderableObject {
    fn get_model_matrix(&self) -> Mat4 {
        let translation = MatrixBuilder::translation(self.position);
        let rotation = MatrixBuilder::rotation(self.rotation);
        let scale = MatrixBuilder::scale(self.scale);

        translation * rotation * scale
    }

    fn render(&self, layer: &GraphicsLayer) {
        self.mesh.shader_program.use_program(true);
        let mvp = layer.view.get_view_matrix() * layer.get_graphics_layer_matrix() * self.get_model_matrix();
        
        self.mesh.shader_program.set_uniform_mat4_f32("mvp", mvp);

        for i in 0..self.texture_array.len() {
            self.texture_array[i].bind(i as u32, true)
        }

        let mut temp_texture: Option<Texture> = None;

        if self.texture_array.len() == 0 {
            let color_buffer = ColorBuffer::new(64, 64, &Color::from_hex(0xffffffff));
            temp_texture = Some(color_buffer.build_texture());
        }

        if temp_texture.is_some() {
            temp_texture.as_ref().unwrap().bind(0, true);
        }

        self.mesh.render();

        if temp_texture.is_some() {
            temp_texture.as_ref().unwrap().bind(0, false);
            temp_texture.as_ref().unwrap().delete();
        }

        for i in 0..self.texture_array.len() {
            self.texture_array[i].bind(i as u32, false)
        }

        self.mesh.shader_program.use_program(false);
    }
}