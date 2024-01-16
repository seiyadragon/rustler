use crate::graphics::vertex::*;
use crate::graphics::shader::*;
use glm::Vec3;

pub struct Mesh {
    pub vao: VAO,
    pub vertex_array: Vec<Vertex>,
    pub index_array: Vec<u32>,
    pub shader_program: ShaderProgram,
}

impl Mesh {
    pub fn new(vertex_array: Vec<Vertex>, index_array: Vec<u32>, shader_program: ShaderProgram) -> Self {
        let mut vao = VAO::new();
        
        let vbo = VBO::new();
        let ibo = IBO::new();
        
        vbo.add_data(vertex_array.clone());
        ibo.add_data(index_array.clone());
        
        vao.bind(true);
        vbo.bind(true);
        ibo.bind(true);

        vao.set_vertex_attribute(shader_program.clone(), "in_position", 3);
        vao.set_vertex_attribute(shader_program.clone(), "in_tex_coords", 3);
        vao.set_vertex_attribute(shader_program.clone(), "in_normal", 3);
        vao.set_vertex_attribute(shader_program.clone(), "in_bone_ids", 3);
        vao.set_vertex_attribute(shader_program.clone(), "in_bone_weights", 3);

        vao.bind(false);

        Mesh {
            vao: vao,
            vertex_array: vertex_array.clone(),
            index_array: index_array.clone(),
            shader_program: shader_program,
        }
    }

    pub fn render(&self) {
        self.shader_program.use_program(true);
        self.vao.bind(true);

        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.index_array.len() as i32, gl::UNSIGNED_INT, std::ptr::null());
        }

        self.vao.bind(false);
        self.shader_program.use_program(false);
    }

    pub fn new_triangle() -> Self {
        let vertices = [
            Vertex::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.5, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
		    Vertex::new(Vec3::new(-1.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
		    Vertex::new(Vec3::new(1.0, -1.0, 0.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
        ];

        let indices = [0, 1, 2];
        let shader_program = ShaderProgram::default_shader_program();

        Mesh::new(vertices.to_vec(), indices.to_vec(), shader_program)
    }

    pub fn new_plane() -> Self {
        let vertices = [
            Vertex::new(Vec3::new(-1.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
            Vertex::new(Vec3::new(-1.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
            Vertex::new(Vec3::new(1.0, -1.0, 0.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
            Vertex::new(Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
        ];
    
        let indices = [0, 1, 2, 2, 3, 0];
        let shader_program = ShaderProgram::default_shader_program();

        Mesh::new(vertices.to_vec(), indices.to_vec(), shader_program)
    }

    pub fn new_cube() -> Self {
        let vertices = [
            // Front face
            Vertex::new(Vec3::new(-1.0, -1.0, 1.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)), // 0
            Vertex::new(Vec3::new(1.0, -1.0, 1.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.25, 0.0, 0.0)), // 1
            Vertex::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.25, 0.25, 0.0)), // 2
            Vertex::new(Vec3::new(-1.0, 1.0, 1.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.25, 0.0)), // 3

            // Back face
            Vertex::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.5, 0.0, 0.0)), // 4
            Vertex::new(Vec3::new(1.0, -1.0, -1.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.75, 0.0, 0.0)), // 5
            Vertex::new(Vec3::new(1.0, 1.0, -1.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.75, 0.25, 0.0)), // 6
            Vertex::new(Vec3::new(-1.0, 1.0, -1.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.5, 0.25, 0.0)), // 7
        ];

        let indices = [
            // Front face
            0, 1, 2, // First triangle
            2, 3, 0, // Second triangle
            // Back face
            4, 5, 6, // First triangle
            6, 7, 4, // Second triangle
            // Right face
            1, 5, 6, // First triangle
            6, 2, 1, // Second triangle
            // Left face
            0, 4, 7, // First triangle
            7, 3, 0, // Second triangle
            // Top face
            3, 7, 6, // First triangle
            6, 2, 3, // Second triangle
            // Bottom face
            0, 1, 5, // First triangle
            5, 4, 0, // Second triangle
        ];

        let shader_program = ShaderProgram::default_shader_program();

        Mesh::new(vertices.to_vec(), indices.to_vec(), shader_program)
    }
}