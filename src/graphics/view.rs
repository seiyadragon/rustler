use glm::Vec2;
use glm::Vec3;
use glm::Mat4;

use super::color::Color;
use super::renderable::Renderable;
use super::math::MatrixBuilder;

#[derive(Clone)]
pub struct View {
    pub size: Vec2,
    pub position: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub fov: f32,
}

impl View {
    pub fn new(size: Vec2, position: Vec3, front: Vec3, up: Vec3, fov: f32) -> Self {
        View {
            size: size,
            position: position,
            front: front,
            up: up,
            fov: fov,
        }
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        MatrixBuilder::perspective(self.fov, self.size, Vec2::new(0.1, 100.0)) * MatrixBuilder::look_at(self.position, self.front, self.up)
    }
}

#[derive(Clone)]
pub struct GraphicsLayer {
    pub view: View,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub parent: Option<Box<GraphicsLayer>>,
}

impl GraphicsLayer {
    pub fn new(view: View, position: Vec3, rotation: Vec3, scale: Vec3) -> Self {
        GraphicsLayer {
            view: view,
            position: position,
            rotation: rotation,
            scale: scale,
            parent: None,
        }
    }

    pub fn default_graphics_layer(view: View) -> Self {
        GraphicsLayer::new(view, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0))
    }

    pub fn get_graphics_layer_matrix(&self) -> Mat4 {
        let translation = MatrixBuilder::translation(self.position);
        let rotation = MatrixBuilder::rotation(self.rotation);
        let scale = MatrixBuilder::scale(self.scale);

        if self.parent.as_ref().is_some() {
            let parent_matrix = self.parent.as_ref().unwrap().get_graphics_layer_matrix();
            return parent_matrix * translation * rotation * scale;
        }

        translation * rotation * scale
    }

    pub fn add_child(self, child: &mut GraphicsLayer) {
        child.parent = Some(Box::new(self));
    }

    pub fn render_object(&self, obj: &dyn Renderable) {
        obj.render(self);
    }

    pub fn clear_screen(&self, color: Color) {
        let color_vec = color.to_vec4();

        unsafe {
            gl::ClearColor(color_vec.x, color_vec.y, color_vec.z, color_vec.w);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
}