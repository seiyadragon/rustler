use gl::types::*;
use glm::Vector2;
use image::{io::Reader, DynamicImage, EncodableLayout, RgbaImage};

use super::color::Color;

pub struct Texture {
    pub texture_id: GLuint,
}

impl Texture {
    pub fn from_vec(size: Vector2<i32>, image_data: &Vec<u8>) -> Self {
        let mut texture: GLuint = 0;

        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as i32);
	        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as i32);
	        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
	        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);

	        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, size.x, size.y, 0, gl::RGBA, gl::UNSIGNED_BYTE, image_data.as_ptr() as *const std::ffi::c_void);
	        gl::GenerateMipmap(gl::TEXTURE_2D);

	        gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Texture {
            texture_id: texture,
        }
    }

    pub fn from_image(image: RgbaImage) -> Self {
        let data_array = image.as_raw();
        let size = Vector2::<i32>::new(image.width() as i32, image.height() as i32);
        
        Self::from_vec(size, data_array)
    }

    pub fn from_file(file: &str) -> Self {
        Self::from_image(Reader::open(file).unwrap().decode().unwrap().into_rgba8())
    }

    pub fn bind(&self, unit_slot: u32, should_bind: bool) {
        match should_bind {
            true => {
                unsafe {
                    gl::ActiveTexture(gl::TEXTURE0 + unit_slot);
                    gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
                }
            }
            false => {
                unsafe {
                    gl::ActiveTexture(gl::TEXTURE0 + unit_slot);
                    gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
                }
            }
        }
    }

    pub fn get_sub_texture(&self, position: Vector2<i32>, size: Vector2<i32>) -> Self {
        let binding = Vec::<u8>::new();

        unsafe {
            gl::TexSubImage2D(self.texture_id, 0, position.x, position.y, size.x, size.y, gl::RGBA, gl::UNSIGNED_BYTE, binding.as_ptr() as *const std::ffi::c_void);
        }

        Texture::from_vec(size, &binding)
    }

}