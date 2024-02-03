use glam::{Vec2, Vec3, Mat4};
use super::color::Color;
use super::renderable::Renderable;
use super::math::Deg;

#[derive(Clone)]
pub enum View {
    View2D(View2D),
    View3D(View3D),
}

#[derive(Clone)]
pub struct View3D {
    pub size: Vec2,
    pub position: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub fov: Deg,
}

impl View3D {
    pub fn new(size: Vec2) -> Self {
        View3D {
            size: size,
            position: Vec3::new(0.0, 0.0, 0.0),
            front: Vec3::new(0.0, 0.0, 1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            fov: Deg(70.0),
        }
    }

    pub fn with_position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    pub fn with_front(mut self, front: Vec3) -> Self {
        self.front = front;
        self
    }

    pub fn with_up(mut self, up: Vec3) -> Self {
        self.up = up;
        self
    }

    pub fn with_fov(mut self, fov: Deg) -> Self {
        self.fov = fov;
        self
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        Mat4::perspective_lh(
            self.fov.to_radians().as_float(), 
            self.size.x / self.size.y, 0.1, 100.0
        ) * Mat4::look_at_lh(
            self.position, 
            self.front, 
            self.up
        )
    }
}

#[derive(Clone)]
pub struct View2D {
    pub size: Vec2,
    pub position: Vec3,
}

impl View2D {
    pub fn new(size: Vec2) -> Self {
        View2D {
            size: size,
            position: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn with_position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        Mat4::orthographic_lh(
            0.0, self.size.x, self.size.y, 0.0, 0.1, 100.0
        ) * Mat4::from_translation(self.position)
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
    pub fn new(view: &View) -> Self {
        GraphicsLayer {
            view: view.clone(),
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            parent: None,
        }
    }

    pub fn with_position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    pub fn with_rotation(mut self, rotation: Vec3) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        self
    }

    pub fn get_graphics_layer_matrix(&self) -> Mat4 {
        let translation = Mat4::from_translation(self.position);

        let rotation_x = Mat4::from_rotation_x(Deg(self.rotation.x).to_radians().as_float());
        let rotation_y = Mat4::from_rotation_y(Deg(self.rotation.y).to_radians().as_float());
        let rotation_z = Mat4::from_rotation_z(Deg(self.rotation.z).to_radians().as_float());

        let rotation = rotation_x * rotation_y * rotation_z;

        let scale = Mat4::from_scale(self.scale);

        if self.parent.as_ref().is_some() {
            let parent_matrix = self.parent.as_ref().unwrap().get_graphics_layer_matrix();
            return parent_matrix * translation * rotation * scale;
        }

        translation * rotation * scale
    }

    pub fn add_child(self, child: &mut GraphicsLayer) {
        child.parent = Some(Box::new(self));
    }

    pub fn render_object(&self, obj: &mut dyn Renderable) {
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