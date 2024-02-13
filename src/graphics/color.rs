
use glam::{Vec3, Vec4};
use image::imageops::flip_vertical;
use image::RgbaImage;
use image::io::Reader;

use crate::graphics::texture::Texture;

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

    pub fn from_vec4(vec: &Vec4) -> Self {
        Color {
            r: (vec.x * 256.0) as u8,
            g: (vec.y * 256.0) as u8,
            b: (vec.z * 256.0) as u8,
            a: (vec.w * 256.0) as u8,
        }
    }

    pub fn from_vec3(vec: &Vec3) -> Self {
        Color {
            r: (vec.x * 256.0) as u8,
            g: (vec.y * 256.0) as u8,
            b: (vec.z * 256.0) as u8,
            a: 255,
        }
    }

    pub fn from_hex(hex: u32) -> Self {
        let (a, r, g, b) = ((hex >> 24) as u8, (hex >> 16) as u8, (hex >> 8) as u8, hex as u8);

        Color::new(r, g, b, a)
    }

    pub fn to_vec4(&self) -> Vec4 {
        Vec4::new(self.r as f32 / 256.0, self.g as f32 / 256.0, self.b as f32 / 256.0, self.a as f32 / 256.0)
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.r as f32 / 256.0, self.g as f32 / 256.0, self.b as f32 / 256.0)
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

#[derive(Clone)]
pub struct ColorBuffer {
    pub buffer: Vec<Color>,
    pub width: u32,
    pub height: u32,
}

impl ColorBuffer {
    pub fn new(width: u32, height: u32, color: &Color) -> ColorBuffer {
        let mut color_vec = Vec::<Color>::new();

        for i in 0..width*height {
            color_vec.push(color.clone());
        }

        ColorBuffer {
            buffer: color_vec,
            width: width,
            height: height,
        }
    }

    pub fn from_byte_vec(width: u32, height: u32, byte_vec: &Vec<u8>) -> ColorBuffer {
        let mut color_vec = Vec::<Color>::new();

        for i in 0..width*height {
            color_vec.push(Color::new(byte_vec[(i * 4 + 0) as usize], byte_vec[(i * 4 + 1) as usize], byte_vec[(i * 4 + 2) as usize], byte_vec[(i * 4 + 3) as usize]));
        }

        ColorBuffer {
            buffer: color_vec,
            width: width,
            height: height,
        }
    }

    pub fn from_image(image: &RgbaImage) -> Self {
        let flipped_image = flip_vertical(image);
        let data_array = flipped_image.as_raw();
        
        Self::from_byte_vec(image.width(), image.height(), &data_array.clone())
    }

    pub fn from_file(file: &str) -> Self {
        Self::from_image(&Reader::open(file).unwrap().decode().unwrap().into_rgba8())
    }

    pub fn to_byte_vec(&self) -> Vec<u8> {
        let mut result_vec = Vec::<u8>::new();

        for color in &self.buffer {
            result_vec.push(color.r);
            result_vec.push(color.g);
            result_vec.push(color.b);
            result_vec.push(color.a);
        }

        result_vec
    }

    pub fn get_color_at_pixel(&self, x: u32, y: u32) -> Color {
        self.buffer[x as usize + y as usize * 4]
    }

    pub fn set_color_at_pixel(&mut self, x: u32, y: u32, color: &Color) {
        self.buffer[x as usize + y as usize * 4] = color.clone();
    }

    pub fn build_texture(&self) -> Texture {
        Texture::from_color_buffer(self)
    }
}