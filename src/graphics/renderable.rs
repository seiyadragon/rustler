use crate::Texture;

use super::mesh::Mesh;
use super::view::GraphicsLayer;
use glm::Vec2;
use glm::Vec3;
use glm::Vec4;
use glm::Mat4;

use glm::Vector2;

pub struct MatrixBuilder;
impl MatrixBuilder {
    pub fn identity(identity: f32) -> Mat4 {
        Mat4::new(
            Vec4::new(identity, 0.0, 0.0, 0.0),
            Vec4::new(0.0, identity, 0.0, 0.0),
            Vec4::new(0.0, 0.0, identity, 0.0),
            Vec4::new(0.0, 0.0, 0.0, identity),
        )
    }

    pub fn translation(position: Vec3) -> Mat4 {
        glm::ext::translate(&Self::identity(1.0), position)
    }

    pub fn rotation(rotation: Vec3) -> Mat4 {
        let rotation_x = glm::ext::rotate(&Self::identity(1.0), rotation.x, Vec3::new(1.0, 0.0, 0.0));
        let rotation_y = glm::ext::rotate(&Self::identity(1.0), rotation.y, Vec3::new(0.0, 1.0, 0.0));
        let rotation_z = glm::ext::rotate(&Self::identity(1.0), rotation.z, Vec3::new(0.0, 0.0, 1.0));

        rotation_x * rotation_y * rotation_z
    }

    pub fn scale(scale: Vec3) -> Mat4 {
        glm::ext::scale(&Self::identity(1.0), scale)
    }

    pub fn perspective(fov_degs: f32, width_height: Vec2, near_far: Vec2) -> Mat4 {
        glm::ext::perspective(glm::radians(fov_degs), width_height.x / width_height.y, near_far.x, near_far.y)
    }

    pub fn look_at(position: Vec3, front: Vec3, up: Vec3) -> Mat4 {
        glm::ext::look_at(position, position + front, up)
    }
}

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
    pub fn new(mesh: Mesh, position: Vec3, rotation: Vec3, scale: Vec3) -> Self {
        RenderableObject {
            mesh: mesh,
            texture_array: Vec::new(),
            position: position,
            rotation: rotation,
            scale: scale,
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
        
        self.mesh.shader_program.set_uniform_mat4f("mvp", mvp);
        self.mesh.shader_program.set_uniform1i("sampler_obj", 0);

        for i in 0..self.texture_array.len() {
            self.texture_array[i].bind(i as u32, true)
        }

        self.mesh.render();

        for i in 0..self.texture_array.len() {
            self.texture_array[i].bind(i as u32, false)
        }
    }
}