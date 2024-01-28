use std::f32::consts::PI;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::Add;

use glm::Vec2;
use glm::{Mat4, Vec3, Vec4};

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

#[derive(Clone, Copy)]
pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Quaternion {
            x,
            y,
            z,
            w,
        }
    }

    pub fn identity() -> Self {
        Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub fn from_euler_angles(x: f32, y: f32, z: f32) -> Self {
        let x = glm::radians(x);
        let y = glm::radians(y);
        let z = glm::radians(z);

        let c1 = glm::cos(y / 2.0);
        let c2 = glm::cos(z / 2.0);
        let c3 = glm::cos(x / 2.0);
        let s1 = glm::sin(y / 2.0);
        let s2 = glm::sin(z / 2.0);
        let s3 = glm::sin(x / 2.0);

        Quaternion {
            x: s1 * s2 * c3 + c1 * c2 * s3,
            y: s1 * c2 * c3 + c1 * s2 * s3,
            z: c1 * s2 * c3 - s1 * c2 * s3,
            w: c1 * c2 * c3 - s1 * s2 * s3,
        }
    }

    pub fn to_euler_angles(&self) -> Vec3 {
        let mut angles = Vec3::new(0.0, 0.0, 0.0);

        let sinr_cosp = 2.0 * (self.w * self.x + self.y * self.z);
        let cosr_cosp = 1.0 - 2.0 * (self.x * self.x + self.y * self.y);
        angles.z = glm::atan(sinr_cosp / cosr_cosp);

        let sinp = 2.0 * (self.w * self.y - self.z * self.x);
        if glm::abs(sinp) >= 1.0 {
            angles.x = glm::sign(sinp) * PI / 2.0;
        } else {
            angles.x = glm::asin(sinp);
        }

        let siny_cosp = 2.0 * (self.w * self.z + self.x * self.y);
        let cosy_cosp = 1.0 - 2.0 * (self.y * self.y + self.z * self.z);
        angles.y = glm::atan(siny_cosp / cosy_cosp);

        angles
    }

    pub fn normalize(&mut self) {
        let length = glm::sqrt(self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w);
        self.x /= length;
        self.y /= length;
        self.z /= length;
        self.w /= length;
    }

    pub fn conjugate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    pub fn inverse(&mut self) {
        self.conjugate();
        self.normalize();
    }

    pub fn dot(&self, rhs: &Quaternion) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn lerp(&self, rhs: &Quaternion, t: f32) -> Quaternion {
        let mut clone = self.clone();
        let mut rhs_clone = rhs.clone();

        let dot = clone.dot(&rhs_clone);

        if dot < 0.0 {
            rhs_clone.conjugate();
            clone.x = -clone.x;
            clone.y = -clone.y;
            clone.z = -clone.z;
            clone.w = -clone.w;
        }

        let mut result = clone + (rhs_clone - clone) * t;
        result.normalize();
        result
    }
}

impl Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Quaternion {
        Quaternion {
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y + self.y * rhs.w + self.z * rhs.x - self.x * rhs.z,
            z: self.w * rhs.z + self.z * rhs.w + self.x * rhs.y - self.y * rhs.x,
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        }
    }
}

impl Mul<f32> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: f32) -> Quaternion {
        Quaternion {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Sub for Quaternion {
    type Output = Quaternion;

    fn sub(self, rhs: Quaternion) -> Quaternion {
        Quaternion {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Add for Quaternion {
    type Output = Quaternion;

    fn add(self, rhs: Quaternion) -> Quaternion {
        Quaternion {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Neg for Quaternion {
    type Output = Quaternion;

    fn neg(self) -> Quaternion {
        let mut clone = self.clone();
        clone.inverse();
        clone
    }
}