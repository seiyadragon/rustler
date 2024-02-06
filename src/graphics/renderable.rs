use crate::{AnimatedMesh, ShaderProgram, StaticMesh, StaticMeshData, Texture, View};
use super::shader;
use super::{math::Deg, vertex::Vertex};
use super::mesh::Mesh;
use super::view::GraphicsLayer;
use glam::{Mat4, Vec2, Vec3};

pub trait Renderable {
    fn render(&self, layer: &GraphicsLayer);
    fn get_model_matrix(&self) -> Mat4;
}

pub struct RenderableMesh {
    pub mesh: Mesh,
    pub texture_array: Vec<Texture>,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl RenderableMesh {
    pub fn new(mesh: Mesh) -> Self {
        RenderableMesh {
            mesh: mesh,
            texture_array: Vec::new(),
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn with_position(mut self, position: &Vec3) -> Self {
        self.position = *position;
        self
    }

    pub fn with_rotation(mut self, rotation: &Vec3) -> Self {
        self.rotation = *rotation;
        self
    }

    pub fn with_scale(mut self, scale: &Vec3) -> Self {
        self.scale = *scale;
        self
    }

    pub fn push_texture(&mut self, texture: &Texture) {
        self.texture_array.push(*texture);
    }

    pub fn pop_texture(&mut self) -> Texture {
        self.texture_array.pop().unwrap()
    }

    pub fn get_static_mesh(&mut self) -> Option<&mut StaticMesh> {
        match &mut self.mesh {
            Mesh::StaticMesh(mesh) => Some(mesh),
            Mesh::AnimatedMesh(_) => None,
        }
    }

    pub fn get_animated_mesh(&mut self) -> Option<&mut AnimatedMesh> {
        match &mut self.mesh {
            Mesh::StaticMesh(_) => None,
            Mesh::AnimatedMesh(mesh) => Some(mesh),
        }
    }

    pub fn move_to(&mut self, position: &Vec3) {
        self.position = *position;
    }

    pub fn move_by(&mut self, position: &Vec3) {
        self.position += *position;
    }

    pub fn rotate_to(&mut self, rotation: &Vec3) {
        self.rotation = *rotation;

        if self.rotation.x > 360.0 || self.rotation.x < -360.0 {
            self.rotation.x = 0.0;
        }

        if self.rotation.y > 360.0 || self.rotation.y < -360.0 {
            self.rotation.y = 0.0;
        }

        if self.rotation.z > 360.0 || self.rotation.z < -360.0 {
            self.rotation.z = 0.0;
        }
    }

    pub fn rotate_by(&mut self, rotation: &Vec3) {
        self.rotation += *rotation;

        if self.rotation.x > 360.0 || self.rotation.x < -360.0 {
            self.rotation.x = 0.0;
        }

        if self.rotation.y > 360.0 || self.rotation.y < -360.0 {
            self.rotation.y = 0.0;
        }

        if self.rotation.z > 360.0 || self.rotation.z < -360.0 {
            self.rotation.z = 0.0;
        }
    }

    pub fn scale_to(&mut self, scale: &Vec3) {
        self.scale = *scale;
    }

    pub fn scale_by(&mut self, scale: &Vec3) {
        self.scale += *scale;
    }
}

impl Renderable for RenderableMesh {
    fn get_model_matrix(&self) -> Mat4 {
        let translation = Mat4::from_translation(self.position);

        let rotation_x = Mat4::from_rotation_x(Deg(self.rotation.x).to_radians().as_float());
        let rotation_y = Mat4::from_rotation_y(Deg(self.rotation.y).to_radians().as_float());
        let rotation_z = Mat4::from_rotation_z(Deg(self.rotation.z).to_radians().as_float());

        let rotation = rotation_x * rotation_y * rotation_z;

        let scale = Mat4::from_scale(self.scale);

        translation * rotation * scale
    }

    fn render(&self, layer: &GraphicsLayer) {
        let shader_program = match &self.mesh {
            Mesh::StaticMesh(mesh) => &mesh.shader_program,
            Mesh::AnimatedMesh(mesh) => &mesh.shader_program,
        };

        let y_up = match &self.mesh {
            Mesh::StaticMesh(mesh) => mesh.y_up,
            Mesh::AnimatedMesh(mesh) => mesh.y_up,
        };

        let mut model_matrix = self.get_model_matrix();

        if !y_up {
            model_matrix = model_matrix * Mat4::from_rotation_x(Deg(-90.0).to_radians().as_float());
        }

        let view_matrix = match &layer.view {
            View::View2D(view) => view.get_view_matrix(),
            View::View3D(view) => view.get_view_matrix(),
        };

        shader_program.use_program(true);
        let mvp = view_matrix * layer.get_graphics_layer_matrix() * model_matrix;
        shader_program.set_uniform_mat4_f32("mvp", &mvp);

        for i in 0..self.texture_array.len() {
            self.texture_array[i].bind(i as u32, true)
        }

        if self.texture_array.len() == 0 {
            shader_program.set_uniform_bool("should_sample_texture", false);
        } else {
            shader_program.set_uniform_bool("should_sample_texture", true);
        }

        match &self.mesh {
            Mesh::StaticMesh(mesh) => {
                let bone_matrices = [Mat4::IDENTITY; 100];
                shader_program.set_uniform_vec_mat4_f32("joint_transforms", &bone_matrices.to_vec());

                mesh.render()
            },
            Mesh::AnimatedMesh(mesh) => {
                shader_program.set_uniform_vec_mat4_f32("joint_transforms", &mesh.animation_player.skeleton.get_global_transform_matrices());
                mesh.render();
            },
        }

        for i in 0..self.texture_array.len() {
            self.texture_array[i].bind(i as u32, false)
        }

        shader_program.use_program(false);
    }
}

impl Drop for RenderableMesh {
    fn drop(&mut self) {
        match &self.mesh {
            Mesh::StaticMesh(mesh) => {
                mesh.delete();
            },
            Mesh::AnimatedMesh(mesh) => {
                mesh.delete();
            },
        }

        for texture in &self.texture_array {
            texture.delete();
        }
    }
}

pub struct RenderableSprite {
    pub mesh: StaticMesh,
    pub texture_array: Vec<Texture>,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl RenderableSprite {
    pub fn new(size: &Vec2) -> Self {
        let w = size.x;
        let h = size.y;

        let vertices = vec![
            Vertex::new(&Vec3::new(-0.5, -0.5, 0.0), &Vec3::new(0.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, 1.0)),
            Vertex::new(&Vec3::new(0.5, -0.5, 0.0), &Vec3::new(1.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, 1.0)),
            Vertex::new(&Vec3::new(0.5, 0.5, 0.0), &Vec3::new(1.0, 1.0, 0.0), &Vec3::new(0.0, 0.0, 1.0)),
            Vertex::new(&Vec3::new(-0.5, 0.5, 0.0), &Vec3::new(0.0, 1.0, 0.0), &Vec3::new(0.0, 0.0, 1.0)),
        ];

        let indices: Vec<u32> = vec![
            0, 1, 2,
            2, 3, 0,
        ];

        let mesh = StaticMeshData::new(&vertices, &indices).build(&ShaderProgram::default_shader_program());

        RenderableSprite {
            mesh: mesh,
            texture_array: Vec::new(),
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(w, h, 1.0),
        }
    }

    pub fn with_position(mut self, position: &Vec3) -> Self {
        self.position = *position;
        self
    }

    pub fn with_rotation(mut self, rotation: &Vec3) -> Self {
        self.rotation = *rotation;
        self
    }

    pub fn with_scale(mut self, scale: &Vec3) -> Self {
        self.scale = *scale;
        self
    }

    pub fn push_texture(&mut self, texture: &Texture) {
        self.texture_array.push(*texture);
    }

    pub fn pop_texture(&mut self) -> Texture {
        self.texture_array.pop().unwrap()
    }

    pub fn move_to(&mut self, position: &Vec3) {
        self.position = *position;
    }

    pub fn move_by(&mut self, position: &Vec3) {
        self.position += *position;
    }

    pub fn rotate_to(&mut self, rotation: &Vec3) {
        self.rotation = *rotation;

        if self.rotation.x > 360.0 || self.rotation.x < -360.0 {
            self.rotation.x = 0.0;
        }

        if self.rotation.y > 360.0 || self.rotation.y < -360.0 {
            self.rotation.y = 0.0;
        }

        if self.rotation.z > 360.0 || self.rotation.z < -360.0 {
            self.rotation.z = 0.0;
        }
    }

    pub fn rotate_by(&mut self, rotation: &Vec3) {
        self.rotation += *rotation;

        if self.rotation.x > 360.0 || self.rotation.x < -360.0 {
            self.rotation.x = 0.0;
        }

        if self.rotation.y > 360.0 || self.rotation.y < -360.0 {
            self.rotation.y = 0.0;
        }

        if self.rotation.z > 360.0 || self.rotation.z < -360.0 {
            self.rotation.z = 0.0;
        }
    }

    pub fn scale_to(&mut self, scale: &Vec3) {
        self.scale = *scale;
    }

    pub fn scale_by(&mut self, scale: &Vec3) {
        self.scale += *scale;
    }
}

impl Renderable for RenderableSprite {
    fn get_model_matrix(&self) -> Mat4 {
        let translation = Mat4::from_translation(self.position);

        let rotation_x = Mat4::from_rotation_x(Deg(self.rotation.x).to_radians().as_float());
        let rotation_y = Mat4::from_rotation_y(Deg(self.rotation.y).to_radians().as_float());
        let rotation_z = Mat4::from_rotation_z(Deg(self.rotation.z).to_radians().as_float());

        let rotation = rotation_x * rotation_y * rotation_z;

        let scale = Mat4::from_scale(self.scale);

        translation * rotation * scale
    }

    fn render(&self, layer: &GraphicsLayer) {
        let shader_program = &self.mesh.shader_program;
        let y_up = self.mesh.y_up;
        let mut model_matrix = self.get_model_matrix();

        if !y_up {
            model_matrix = model_matrix * Mat4::from_rotation_x(Deg(-90.0).to_radians().as_float());
        }

        let view_matrix = match &layer.view {
            View::View2D(view) => view.get_view_matrix(),
            View::View3D(view) => view.get_view_matrix(),
        };

        shader_program.use_program(true);
        let mvp = view_matrix * layer.get_graphics_layer_matrix() * model_matrix;
        shader_program.set_uniform_mat4_f32("mvp", &mvp);

        for i in 0..self.texture_array.len() {
            self.texture_array[i].bind(i as u32, true)
        }

        if self.texture_array.len() == 0 {
            shader_program.set_uniform_bool("should_sample_texture", false);
        } else {
            shader_program.set_uniform_bool("should_sample_texture", true);
        }

        let bone_matrices = [Mat4::IDENTITY; 100];
        shader_program.set_uniform_vec_mat4_f32("joint_transforms", &bone_matrices.to_vec());

        self.mesh.render();            

        for i in 0..self.texture_array.len() {
            self.texture_array[i].bind(i as u32, false)
        }

        shader_program.use_program(false);
    }
}

impl Drop for RenderableSprite {
    fn drop(&mut self) {
        self.mesh.delete();

        for texture in &self.texture_array {
            texture.delete();
        }
    }
}