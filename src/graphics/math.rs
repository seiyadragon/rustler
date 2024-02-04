use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Deg(pub f32);

#[derive(Debug, Clone, Copy)]
pub struct Rad(pub f32);

impl Deg {
    pub fn to_radians(&self) -> Rad {
        Rad(self.0 * PI / 180.0)
    }

    pub fn as_float(&self) -> f32 {
        self.0
    }
}

impl Rad {
    pub fn to_degrees(&self) -> Deg {
        Deg(self.0 * 180.0 / PI)
    }

    pub fn as_float(&self) -> f32 {
        self.0
    }
}