use gl::types::*;
use glam::{Vec2, Vec3, Vec4, IVec2, IVec3, IVec4, Mat4};
use core::fmt;
use std::{error::Error, ptr::{null, null_mut}};
use std::ffi::CString;
use std::str;

pub const DEFAULT_VERTEX_SHADER: &str = "
    #version 450 core

    const int MAX_BONES = 100;
    const int MAX_WEIGHTS = 3;

    layout (location = 0) in vec3 in_position;
    layout (location = 1) in vec3 in_tex_coords;
    layout (location = 2) in vec3 in_normal;
    layout (location = 3) in vec3 in_bone_ids;
    layout (location = 4) in vec3 in_bone_weights;
    layout (location = 5) in vec3 in_color;

    out vec3 tex_coords;
    out vec3 vertex_color;

    uniform mat4 mvp;
    uniform mat4 joint_transforms[MAX_BONES];

    void main() {
        bool should_animate = false;

        if (in_bone_ids.x != 0.0 || in_bone_ids.y != 0.0 || in_bone_ids.z != 0.0) {
            should_animate = true;
        }

        if (should_animate) {
            vec4 total_local_pos = vec4(0.0);
            vec4 total_normal = vec4(0.0);
            
            for(int i = 0; i < MAX_WEIGHTS; i++) {
                mat4 joint_transform = joint_transforms[highp int(in_bone_ids[i])];
                vec4 pos_position = joint_transform * vec4(in_position, 1.0);
                total_local_pos += pos_position * in_bone_weights[i];
            
                vec4 world_normal = joint_transform * vec4(in_normal, 0.0);
                total_normal += world_normal * in_bone_weights[i];
            }

            gl_Position = mvp * total_local_pos;
        } else {
            gl_Position = mvp * vec4(in_position, 1.0);
        }

        tex_coords = in_tex_coords;
        vertex_color = in_color;
    }
";

pub const DEFAULT_FRAGMENT_SHADER: &str = "
    #version 450 core

    in vec3 tex_coords;
    in vec3 vertex_color;

    out vec4 output_color;

    uniform bool should_sample_texture;
    uniform sampler2D sampler_objs[32];

    void main() {
        highp int sampler_index = int(tex_coords.z);

        if (should_sample_texture) {
            output_color = mix(texture(sampler_objs[sampler_index], tex_coords.xy), vec4(vertex_color, 1.0), 0.5);
        } else {
            output_color = vec4(vertex_color, 1.0);
        }
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
pub struct ShaderSource {
    pub shader_type: ShaderType,
    pub shader_src: String,
}

impl ShaderSource {
    pub fn from_source_string(shader_type: &ShaderType, shader_source: &str) -> ShaderSource {
        ShaderSource { 
            shader_type: shader_type.clone(), 
            shader_src: String::from(shader_source),
        }
    }

    pub fn new(shader_type: &ShaderType) -> ShaderSource {
        ShaderSource { 
            shader_type: shader_type.clone(), 
            shader_src: String::from("#version 450 core\n"),
        }
    }

    pub fn default_shader_source(shader_type: ShaderType) -> ShaderSource {
        match shader_type {
            ShaderType::VERTEX => {
                ShaderSource::from_source_string(&ShaderType::VERTEX, DEFAULT_VERTEX_SHADER)
            }
            ShaderType::FRAGMENT => {
                ShaderSource::from_source_string(&ShaderType::FRAGMENT, DEFAULT_FRAGMENT_SHADER)
            }
            ShaderType::GEOMETRY => {
                todo!();
            }
        }
    }

    pub fn compile(&self) -> Result<CompiledShader, ShaderError> {
        CompiledShader::new(self)
    }
}
#[derive(Debug, Clone)]
pub struct CompiledShader {
    pub shader_id: GLuint,
    pub shader_type: ShaderType,
    pub shader_src: String,
}

impl CompiledShader {
    pub fn new(shader_source: &ShaderSource) -> Result<CompiledShader, ShaderError> {
        let shader_type = shader_source.shader_type;
        let shader_source = shader_source.shader_src.clone();

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

    pub fn delete_shader(&self) -> ShaderSource {
        unsafe {
            gl::DeleteShader(self.shader_id);
        }

        ShaderSource::from_source_string(&self.shader_type, &self.shader_src.as_str())
    }

    pub fn default_vertex_shader() -> Result<CompiledShader, ShaderError> {
        let result = Self::new(&ShaderSource::default_shader_source(ShaderType::VERTEX));

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
        let result = Self::new(&ShaderSource::default_shader_source(ShaderType::FRAGMENT));

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

impl Drop for CompiledShader {
    fn drop(&mut self) {
        self.delete_shader();
    }
}

#[derive(Debug, Clone)]
pub struct ShaderProgram {
    pub program_id: GLuint,
    pub shader_list: Vec<ShaderSource>,
    compiled_shader_list: Vec<CompiledShader>,
}

impl ShaderProgram {
    pub fn new() -> ShaderProgram {
        unsafe {
            let program = gl::CreateProgram();

            ShaderProgram {
                program_id: program,
                shader_list: Vec::new(),
                compiled_shader_list: Vec::new(),
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

    pub fn attach_shader(&mut self, shader: &CompiledShader) {
        self.compiled_shader_list.push(shader.clone());
    }

    pub fn build(&mut self) {
        self.use_program(true);

        unsafe {
            for shader in &self.compiled_shader_list {
                gl::AttachShader(self.program_id, shader.shader_id);
            }
    
            gl::LinkProgram(self.program_id);
        }

        for s in &self.compiled_shader_list {
            self.shader_list.push(s.delete_shader());
        }

        self.compiled_shader_list = Vec::new();

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
            }
        }

        self.use_program(false);
    }

    pub fn default_shader_program() -> Self {
        let mut program = Self::new();
        let vertex_shader = CompiledShader::default_vertex_shader().unwrap();
        let fragment_shader = CompiledShader::default_fragment_shader().unwrap();

        program.attach_shader(&vertex_shader);
        program.attach_shader(&fragment_shader);

        program.build();

        program
    }

    pub fn set_uniform_f32(&self, name: &str, value: f32) {
        let c_str = CString::new(name.as_bytes()).unwrap();

        self.use_program(true);

        unsafe {
            gl::Uniform1f(gl::GetUniformLocation(self.program_id, c_str.as_ptr() as *const i8), value);
        }

        self.use_program(false);
    }

    pub fn set_uniform_vec2_f32(&self, name: &str, value: &Vec2) {
        let c_str = CString::new(name.as_bytes()).unwrap();

        self.use_program(true);

        unsafe {
            gl::Uniform2f(gl::GetUniformLocation(self.program_id, c_str.as_ptr() as *const i8), value.x, value.y);
        }

        self.use_program(false);
    }

    pub fn set_uniform_vec3_f32(&self, name: &str, value: &Vec3) {
        let c_str = CString::new(name.as_bytes()).unwrap();

        self.use_program(true);

        unsafe {
            gl::Uniform3f(gl::GetUniformLocation(self.program_id, c_str.as_ptr() as *const i8), value.x, value.y, value.z);
        }

        self.use_program(false);
    }

    pub fn set_uniform_vec4_f32(&self, name: &str, value: &Vec4) {
        let c_str = CString::new(name.as_bytes()).unwrap();

        self.use_program(true);

        unsafe {
            gl::Uniform4f(gl::GetUniformLocation(self.program_id, c_str.as_ptr() as *const i8), value.x, value.y, value.z, value.w);
        }

        self.use_program(false);
    }

    pub fn set_uniform_mat4_f32(&self, name: &str, value: &Mat4) {
        let c_str = CString::new(name.as_bytes()).unwrap();

        self.use_program(true);

        unsafe {
            gl::UniformMatrix4fv(gl::GetUniformLocation(self.program_id, c_str.as_ptr()), 1, gl::FALSE, &value.to_cols_array()[0]);
        }

        self.use_program(false);
    }

    pub fn set_uniform_vec_f32(&self, name: &str, vec: &Vec<f32>) {
        let c_str = CString::new(name.as_bytes()).unwrap();

        self.use_program(true);

        unsafe {
            gl::Uniform1fv(gl::GetUniformLocation(self.program_id, c_str.as_ptr()), vec.len() as i32, vec.as_ptr())
        }

        self.use_program(false);
    }

    pub fn set_uniform_vec_mat4_f32(&self, name: &str, vec: &Vec<Mat4>) {
        let c_str = CString::new(name.as_bytes()).unwrap();

        self.use_program(true);

        unsafe {
            gl::UniformMatrix4fv(gl::GetUniformLocation(self.program_id, c_str.as_ptr()), vec.len() as i32, gl::FALSE, vec.as_ptr() as *const f32);
        }

        self.use_program(false);
    }

    pub fn set_uniform_i32(&self, name: &str, value: i32) {
        let c_str = CString::new(name.as_bytes()).unwrap();

        self.use_program(true);

        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self.program_id, c_str.as_ptr() as *const i8), value);
        }

        self.use_program(false);
    }

    pub fn set_uniform_vec2_i32(&self, name: &str, value: &IVec2) {
        let c_str = CString::new(name.as_bytes()).unwrap();

        self.use_program(true);

        unsafe {
            gl::Uniform2i(gl::GetUniformLocation(self.program_id, c_str.as_ptr() as *const i8), value.x, value.y);
        }

        self.use_program(false);
    }

    pub fn set_uniform_vec3_i32(&self, name: &str, value: &IVec3) {
        let c_str = CString::new(name.as_bytes()).unwrap();

        self.use_program(true);

        unsafe {
            gl::Uniform3i(gl::GetUniformLocation(self.program_id, c_str.as_ptr() as *const i8), value.x, value.y, value.z);
        }

        self.use_program(false);
    }

    pub fn set_uniform_vec4_i32(&self, name: &str, value: &IVec4) {
        let c_str = CString::new(name.as_bytes()).unwrap();

        self.use_program(true);

        unsafe {
            gl::Uniform4i(gl::GetUniformLocation(self.program_id, c_str.as_ptr() as *const i8), value.x, value.y, value.z, value.w);
        }

        self.use_program(false);
    }

    pub fn set_uniform_vec_i32(&self, name: &str, vec: &Vec<i32>) {
        let c_str = CString::new(name.as_bytes()).unwrap();

        self.use_program(true);

        unsafe {
            gl::Uniform1iv(gl::GetUniformLocation(self.program_id, c_str.as_ptr()), vec.len() as i32, vec.as_ptr())
        }

        self.use_program(false);
    }

    pub fn set_uniform_bool(&self, name: &str, value: bool) {
        self.set_uniform_i32(name, value as i32);
    }

    pub fn set_uniform_vec_bool(&self, name: &str, vec: &Vec<bool>) {
        let vec_i32: Vec<i32> = vec.iter().map(|&x| x as i32).collect();

        self.set_uniform_vec_i32(name, &vec_i32)
    }
}