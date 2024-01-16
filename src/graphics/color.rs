
use glm::Vec4;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }

    pub fn from_vec4(vec: Vec4) -> Self {
        Color {
            r: (vec.x * 256.0) as u8,
            g: (vec.y * 256.0) as u8,
            b: (vec.z * 256.0) as u8,
            a: (vec.w * 256.0) as u8,
        }
    }

    pub fn from_hex(hex: u32) -> Self {
        let (a, r, g, b) = ((hex >> 24) as u8, (hex >> 16) as u8, (hex >> 8) as u8, hex as u8);

        Color::new(r, g, b, a)
    }

    pub fn to_vec4(&self) -> Vec4 {
        Vec4::new(self.r as f32 / 256.0, self.g as f32 / 256.0, self.b as f32 / 256.0, self.a as f32 / 256.0)
    }

    pub fn to_hex(&self) -> u32 {
        let color = u32::from(self.a) << 24 | u32::from(self.r) << 16 | u32::from(self.g) << 8 | u32::from(self.b);

        color
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Color: [r: {0}, g: {1}, b: {2}, a: {3}]", self.r, self.g, self.b, self.a)
    }
}