use gl::types::*;
use glam::IVec2;
use image::{io::Reader, RgbaImage};

use crate::ColorBuffer;

#[derive(Clone, Copy)]
pub struct Texture {
    pub texture_id: GLuint,
}

impl Texture {
    pub fn from_vec(size: &IVec2, image_data: &Vec<u8>) -> Self {
        let mut texture: GLuint = 0;

        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::ActiveTexture(gl::TEXTURE0);
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

    pub fn from_color_buffer(color_buffer: &ColorBuffer) -> Texture {
        Texture::from_vec(&IVec2::new(color_buffer.width as i32, color_buffer.height as i32), &color_buffer.to_byte_vec())
    }

    pub fn from_image(image: &RgbaImage) -> Self {
        let data_array = image.as_raw();
        let size = IVec2::new(image.width() as i32, image.height() as i32);
        
        Self::from_vec(&size, data_array)
    }

    pub fn from_file(file: &str) -> Self {
        Self::from_image(&Reader::open(file).unwrap().decode().unwrap().into_rgba8())
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

    pub fn get_width(&self) -> u32 {
        self.bind(0, true);
        let mut width: GLint = 0;

        unsafe {
            gl::GetTexLevelParameteriv(gl::TEXTURE_2D, 0, gl::TEXTURE_WIDTH, &mut width)
        }

        self.bind(0, false);

        width as u32
    }

    pub fn get_height(&self) -> u32 {
        self.bind(0, true);
        let mut height: GLint = 0;

        unsafe {
            gl::GetTexLevelParameteriv(gl::TEXTURE_2D, 0, gl::TEXTURE_HEIGHT, &mut height)
        }

        self.bind(0, false);

        height as u32
    }

    pub fn get_sub_texture(&self, position: &IVec2, size: &IVec2) -> Self {
        let binding = Vec::<u8>::new();

        unsafe {
            gl::TexSubImage2D(self.texture_id, 0, position.x, position.y, size.x, size.y, gl::RGBA, gl::UNSIGNED_BYTE, binding.as_ptr() as *const std::ffi::c_void);
        }

        Texture::from_vec(&size, &binding)
    }

    pub fn get_color_buffer(&self) -> ColorBuffer {
        self.bind(0, true);

        let width = self.get_width();
        let height = self.get_height();

        let mut data: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);

        unsafe { 
            gl::GetTexImage(gl::TEXTURE_2D, 0, gl::RGBA, gl::UNSIGNED_BYTE, data.as_mut_ptr() as *mut std::ffi::c_void);
            data.set_len((width * height * 4) as usize);
        }

        ColorBuffer::from_byte_vec(width, height, &data)
    }

    pub fn delete(&self) -> ColorBuffer {
        let color_buffer = self.get_color_buffer();

        unsafe {
            gl::DeleteTextures(1, &self.texture_id);
        }

        color_buffer.clone()
    }

}