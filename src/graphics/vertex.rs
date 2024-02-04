use core::{fmt, slice};
use std::{ffi::{c_void, CString}, mem};
use gl::types::*;
use glam::Vec3;

use super::shader::ShaderProgram;

pub const VERTEX_SIZE: isize = unsafe {
    std::mem::size_of::<Vertex>() as isize
};

pub const FLOAT_SIZE: isize = unsafe {
    std::mem::size_of::<f32>() as isize
};

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub texture: Vec3,
    pub normals: Vec3,
    pub bone_ids: Vec3,
    pub bone_weights: Vec3,
    pub color: Vec3,
}

impl Vertex {
    pub fn new(position: &Vec3, texture: &Vec3, normals: &Vec3) -> Self {
        Vertex {
            position: position.clone(),
            texture: texture.clone(),
            normals: normals.clone(),
            bone_ids: Vec3::new(0.0, 0.0, 0.0),
            bone_weights: Vec3::new(0.0, 0.0, 0.0),
            color: Vec3::new(1.0, 1.0, 1.0),
        }
    }
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[x: {0}, y: {1}, z: {2}, u: {3}, v: {4}, i: {5}, nx: {6}, ny: {7}, nz: {8}]",
            self.position.x,
            self.position.y,
            self.position.z,
            self.texture.x,
            self.texture.y,
            self.texture.z,
            self.normals.x,
            self.normals.y,
            self.normals.z,
        )
    }
}

#[derive(Clone)]
pub struct VAO {
    pub id: GLuint,
    pub component_count: isize,
}

impl VAO {
    pub fn new() -> Self {
        let mut id: GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        VAO { 
            id: id,
            component_count: 0,
        }
    }

    pub fn bind(&self, bind: bool) {
        match bind {
            true => {
                unsafe {
                    gl::BindVertexArray(self.id);
                }
            }
            false => {
                unsafe {
                    gl::BindVertexArray(0);
                }
            }
        }
    }

    pub fn set_vertex_attribute(&mut self, shader_program: ShaderProgram, attribute_name: &str, attribute_component_count: isize) {
        let c_str = CString::new(attribute_name.as_bytes()).unwrap();
    
        unsafe {
            let attrib_loc: i32 = gl::GetAttribLocation(shader_program.program_id, c_str.as_ptr());
            gl::EnableVertexAttribArray(attrib_loc as u32);
            gl::VertexAttribPointer(attrib_loc as u32, 3, gl::FLOAT, gl::FALSE, i32::try_from(VERTEX_SIZE).unwrap(), (self.component_count*FLOAT_SIZE) as *const c_void);
        }

        self.component_count += attribute_component_count;
    }

    pub fn render(&self, index_count: usize) {
        self.bind(true);

        unsafe {
            gl::DrawElements(gl::TRIANGLES, index_count as i32, gl::UNSIGNED_INT, std::ptr::null());
        }

        self.bind(false);
    }

    pub fn get_data(&self) -> (Vec<Vertex>, Vec<u32>) {
        unsafe {
            self.bind(true);

            let mut vertex_size: GLint = 0;
            gl::GetBufferParameteriv(gl::ARRAY_BUFFER, gl::BUFFER_SIZE, &mut vertex_size);

            let data_ptr = gl::MapBufferRange(gl::ARRAY_BUFFER, 0, vertex_size as isize, gl::MAP_READ_BIT) as *const Vertex;
            let data_slice = slice::from_raw_parts(data_ptr, vertex_size as usize / mem::size_of::<Vertex>());
            let vertex_data: Vec<Vertex> = data_slice.to_vec();

            gl::UnmapBuffer(gl::ARRAY_BUFFER);

            let mut index_size: GLint = 0;
            gl::GetBufferParameteriv(gl::ELEMENT_ARRAY_BUFFER, gl::BUFFER_SIZE, &mut index_size);

            let data_ptr = gl::MapBufferRange(gl::ELEMENT_ARRAY_BUFFER, 0, index_size as isize, gl::MAP_READ_BIT) as *const u32;
            let data_slice = slice::from_raw_parts(data_ptr, index_size as usize / mem::size_of::<u32>());
            let index_data: Vec<u32> = data_slice.to_vec();

            gl::UnmapBuffer(gl::ELEMENT_ARRAY_BUFFER);

            self.bind(false);

            (vertex_data, index_data)
        }
    }

    pub fn delete(&self) -> (Vec<Vertex>, Vec<u32>) {
        let (vertex_data, index_data) = self.get_data();

        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }

        (vertex_data, index_data)
    }

    pub fn build_from_data(vertex_array: &Vec<Vertex>, index_array: &Vec<u32>, shader_program: &ShaderProgram) -> Self {
        let vbo = VBO::new();
        vbo.bind(true);
        vbo.add_data(vertex_array.clone());
        vbo.bind(false);

        let ibo = IBO::new();
        ibo.bind(true);
        ibo.add_data(index_array.clone());
        ibo.bind(false);

        let mut vao = VAO::new();

        vao.bind(true);
        vbo.bind(true);
        ibo.bind(true);

        vao.set_vertex_attribute(shader_program.clone(), "in_position", 3);
        vao.set_vertex_attribute(shader_program.clone(), "in_tex_coords", 3);
        vao.set_vertex_attribute(shader_program.clone(), "in_normal", 3);
        vao.set_vertex_attribute(shader_program.clone(), "in_bone_ids", 3);
        vao.set_vertex_attribute(shader_program.clone(), "in_bone_weights", 3);
        vao.set_vertex_attribute(shader_program.clone(), "in_color", 3);

        shader_program.use_program(true);

        shader_program.set_uniform_vec_i32("sampler_objs", &vec![
            00, 01, 02, 03, 04, 05, 06, 07,
            08, 09, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ]);

        shader_program.use_program(false);
        vao.bind(false);

        vao
    }
}

#[derive(Clone)]
pub struct VBO {
    pub id: GLuint,
}

impl VBO {
    pub fn new() -> Self {
        let mut id: GLuint = 0;

        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        VBO { id: id }
    }

    pub fn bind(&self, bind: bool) {
        match bind {
            true => {
                unsafe {
                    gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
                }
            }
            false => {
                unsafe {
                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                }
            }
        }
    }

    pub fn add_data(&self, vertex_data: Vec<Vertex>) {
        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER, isize::from(vertex_data.len() as i16)*(VERTEX_SIZE), vertex_data.as_ptr() as *const c_void, gl::STATIC_DRAW);
        }
    }

    pub fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

#[derive(Clone)]
pub struct IBO {
    pub id: GLuint,
}

impl IBO {
    pub fn new() -> Self {
        let mut id: GLuint = 0;

        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        IBO { id: id }
    }

    pub fn bind(&self, bind: bool) {
        match bind {
            true => {
                unsafe {
                    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
                }
            }
            false => {
                unsafe {
                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                }
            }
        }
    }

    pub fn add_data(&self, index_data: Vec<u32>) {
        unsafe {
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, isize::from(index_data.len() as i16)*FLOAT_SIZE, index_data.as_ptr() as *const c_void, gl::STATIC_DRAW);
        }
    }

    pub fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}