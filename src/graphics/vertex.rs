use core::fmt;
use std::ffi::{c_void, CString};
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

    pub fn delete(&self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
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