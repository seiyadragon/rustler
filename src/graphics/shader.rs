use gl::types::*;
use glm::{Vec2, Vec3, Vec4};
use core::fmt;
use std::{error::Error, ptr::{null, null_mut}};
use std::ffi::CString;
use std::str;
use glm::Mat4;

pub const DEFAULT_VERTEX_SHADER: &str = "
    #version 430 core

    layout (location = 0) in vec3 in_position;
    layout (location = 1) in vec3 in_tex_coords;
    layout (location = 2) in vec3 in_normal;
    out vec3 tex_coords;

    uniform mat4 mvp;

    void main() {
        gl_Position = mvp * vec4(in_position, 1.0);
        tex_coords = in_tex_coords;
    }
";

pub const DEFAULT_FRAGMENT_SHADER: &str = "
    #version 430 core

    in vec3 tex_coords;
    out vec4 output_color;

    uniform sampler2D sampler_obj;
    uniform vec3 color;

    void main() {
        output_color = texture(sampler_obj, tex_coords.xy) * vec4(color, 1.0);
        //output_color = vec4(color, 1.0);
    }
";

#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    VERTEX = gl::VERTEX_SHADER as isize,
    FRAGMENT = gl::FRAGMENT_SHADER as isize,
    GEOMETRY = gl::GEOMETRY_SHADER as isize,
}

#[derive(Debug, Clone)]
pub struct ShaderError {
    pub error_log: String,
}

impl std::fmt::Display for ShaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error_log.as_str())
    }
}

impl Error for ShaderError {}

impl ShaderError {
    pub fn new(string: String) -> Self {
        ShaderError {
            error_log: string,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompiledShader {
    pub shader_id: GLuint,
    pub shader_type: ShaderType,
    pub shader_src: String,
}

impl CompiledShader {
    pub fn new(shader_type: ShaderType, shader_source: &str) -> Result<CompiledShader, ShaderError> {
        
        unsafe {
            let shader = gl::CreateShader(shader_type as GLuint);
            let c_str = CString::new(shader_source.as_bytes()).unwrap();

            gl::ShaderSource(shader, 1, &c_str.as_ptr(), null());
            gl::CompileShader(shader);

            let mut status = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            if status == gl::FALSE as i32 {
                let mut log_len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_len);

                let mut log = Vec::with_capacity(log_len as usize);
                log.set_len((log_len as usize) - 1);

                gl::GetShaderInfoLog(shader, log_len, null_mut(), log.as_mut_ptr() as *mut GLchar);

                let final_error_log = str::from_utf8(&log).unwrap();

                return Err(ShaderError::new(String::from(final_error_log)));
            }

            Ok(CompiledShader {
                shader_id: shader,
                shader_src: String::from(shader_source),
                shader_type: shader_type.clone(),
            })
        }
    }

    pub fn delete_shader(&self) {
        unsafe {
            gl::DeleteShader(self.shader_id);
        }
    }

    pub fn default_vertex_shader() -> Result<CompiledShader, ShaderError> {
        let result = Self::new(ShaderType::VERTEX, DEFAULT_VERTEX_SHADER);

        match result {
            Ok(compiled_shader) => {
                return Ok(compiled_shader);
            },
            Err(shader_error) => {
                println!("{shader_error}");
                return Err(shader_error);
            }
        }
    }

    pub fn default_fragment_shader() -> Result<CompiledShader, ShaderError> {
        let result = Self::new(ShaderType::FRAGMENT, DEFAULT_FRAGMENT_SHADER);

        match result {
            Ok(compiled_shader) => {
                return Ok(compiled_shader);
            },
            Err(shader_error) => {
                println!("{shader_error}");
                return Err(shader_error);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShaderProgram {
    pub program_id: GLuint,
    pub shader_list: Vec<CompiledShader>,
}

impl ShaderProgram {
    pub fn new() -> ShaderProgram {
        unsafe {
            let program = gl::CreateProgram();

            ShaderProgram {
                program_id: program,
                shader_list: Vec::new(),
            }
        }
    }

    pub fn use_program(&self, should_use: bool) {
        unsafe {
            if should_use {
                gl::UseProgram(self.program_id);
            } else {
                gl::UseProgram(0);
            }
        }
    }

    pub fn attach_shader(&mut self, shader: CompiledShader) {
        self.shader_list.push(shader);
    }

    pub fn build(&self) {
        self.use_program(true);

        unsafe {
            for shader in &self.shader_list {
                gl::AttachShader(self.program_id, shader.shader_id);
            }
    
            gl::LinkProgram(self.program_id);
        }

        for s in &self.shader_list {
            s.delete_shader();
        }

        unsafe {
            let mut status = 0;
            gl::GetProgramiv(self.program_id, gl::LINK_STATUS, &mut status);

            if status == gl::FALSE as i32 {
                let mut log_len = 0;
                gl::GetProgramiv(self.program_id, gl::INFO_LOG_LENGTH, &mut log_len);

                let mut log = Vec::with_capacity(log_len as usize);
                log.set_len((log_len as usize) - 1);

                gl::GetProgramInfoLog(self.program_id, log_len, null_mut(), log.as_mut_ptr() as *mut GLchar);

                let final_error_log = str::from_utf8(&log).unwrap();

                println!("Shader Program failed to build: {final_error_log}");
            }
        }

        self.use_program(false);
    }

    pub fn default_shader_program() -> Self {
        let mut program = Self::new();
        let vertex_shader = CompiledShader::default_vertex_shader().unwrap();
        let fragment_shader = CompiledShader::default_fragment_shader().unwrap();

        program.attach_shader(vertex_shader);
        program.attach_shader(fragment_shader);

        program.build();

        program
    }

    pub fn set_uniform1f(&self, name: &str, value: f32) {
        let c_str = CString::new(name.as_bytes()).unwrap();

        self.use_program(true);

        unsafe {
            gl::Uniform1f(gl::GetUniformLocation(self.program_id, c_str.as_ptr() as *const i8), value);
        }

        self.use_program(false);
    }

    pub fn set_uniform1i(&self, name: &str, value: i32) {
        let c_str = CString::new(name.as_bytes()).unwrap();

        self.use_program(true);

        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self.program_id, c_str.as_ptr() as *const i8), value);
        }

        self.use_program(false);
    }

    pub fn set_uniform2f(&self, name: &str, value: Vec2) {
        let c_str = CString::new(name.as_bytes()).unwrap();

        self.use_program(true);

        unsafe {
            gl::Uniform2f(gl::GetUniformLocation(self.program_id, c_str.as_ptr() as *const i8), value.x, value.y);
        }

        self.use_program(false);
    }

    pub fn set_uniform3f(&self, name: &str, value: Vec3) {
        let c_str = CString::new(name.as_bytes()).unwrap();

        self.use_program(true);

        unsafe {
            gl::Uniform3f(gl::GetUniformLocation(self.program_id, c_str.as_ptr() as *const i8), value.x, value.y, value.z);
        }

        self.use_program(false);
    }

    pub fn set_uniform4f(&self, name: &str, value: Vec4) {
        let c_str = CString::new(name.as_bytes()).unwrap();

        self.use_program(true);

        unsafe {
            gl::Uniform4f(gl::GetUniformLocation(self.program_id, c_str.as_ptr() as *const i8), value.x, value.y, value.z, value.w);
        }

        self.use_program(false);
    }

    pub fn set_uniform_mat4f(&self, name: &str, value: Mat4) {
        let c_str = CString::new(name.as_bytes()).unwrap();

        self.use_program(true);

        unsafe {
            let array: [[f32; 4]; 4] = [
                value.c0.as_array().clone(),
                value.c1.as_array().clone(),
                value.c2.as_array().clone(),
                value.c3.as_array().clone(),
            ];
            let array_ptr: *const f32 = std::mem::transmute(&array);

            gl::UniformMatrix4fv(gl::GetUniformLocation(self.program_id, c_str.as_ptr()), 1, gl::FALSE, array_ptr);
        }

        self.use_program(false);
    }
}