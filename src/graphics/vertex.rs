use std::ffi::{c_void, CString};
use gl::types::*;
use super::shader::ShaderProgram;

pub const VERTEX_DATA_SIZE: isize = 4;
pub const VERTEX_DATA_FLOATS: isize = 15;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: glm::Vec3,
    pub texture: glm::Vec3,
    pub normals: glm::Vec3,
    pub bone_ids: glm::Vec3,
    pub bone_weights: glm::Vec3,
}

impl Vertex {
    pub fn new(position: glm::Vec3, texture: glm::Vec3, normals: glm::Vec3) -> Self {
        Vertex {
            position: position,
            texture: texture,
            normals: normals,
            bone_ids: glm::Vec3::new(0.0, 0.0, 0.0),
            bone_weights: glm::Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

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
        self.bind(true);
        let c_str = CString::new(attribute_name.as_bytes()).unwrap();
    
        unsafe {
            let attrib_loc: i32 = gl::GetAttribLocation(shader_program.program_id, c_str.as_ptr());
            gl::EnableVertexAttribArray(attrib_loc as u32);
            gl::VertexAttribPointer(attrib_loc as u32, 3, gl::FLOAT, gl::FALSE, i32::try_from(VERTEX_DATA_FLOATS*VERTEX_DATA_SIZE).unwrap(), (self.component_count*VERTEX_DATA_SIZE) as *const c_void);
        }

        self.component_count += attribute_component_count;
        self.bind(false);
    }
}

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
        self.bind(true);

        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER, isize::from(vertex_data.len() as i16)*(VERTEX_DATA_FLOATS*VERTEX_DATA_SIZE), vertex_data.as_ptr() as *const c_void, gl::STATIC_DRAW);
        }

        self.bind(false);
    }
}

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
        self.bind(true);

        unsafe {
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, isize::from(index_data.len() as i16)*VERTEX_DATA_SIZE, index_data.as_ptr() as *const c_void, gl::STATIC_DRAW);
        }

        self.bind(false);
    }
}