use std::mem;
use std::slice;
use crate::graphics::vertex::*;
use crate::graphics::shader::*;
use gl::types::GLint;
use glm::Vec2;
use glm::Vec3;
use blend::Blend;
use libflate::gzip::Decoder;
use std::{
    env,
    fs::File,
    io::{self, BufWriter, Read, Write},
    path::{self, PathBuf},
};

#[derive(Clone)]
pub struct MeshData {
    pub vertex_array: Vec<Vertex>,
    pub index_array: Vec<u32>,
}

impl MeshData {
    pub fn new(vertex_array: &Vec<Vertex>, index_array: &Vec<u32>) -> MeshData {
        MeshData {
            vertex_array: vertex_array.clone(),
            index_array: index_array.clone(),
        }
    }

    pub fn new_by_value(vertex_array: Vec<Vertex>, index_array: Vec<u32>) -> MeshData {
        MeshData {
            vertex_array: vertex_array,
            index_array: index_array,
        }
    }

    pub fn build_mesh(self, shader_program: &ShaderProgram) -> Mesh {
        Mesh::new(&self, shader_program)
    }

    pub fn load_blender_mesh(path: &str, compressed: bool) -> MeshData {
        let mut blend = Blend::from_path(path);

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        if compressed {
            let mut file = File::open(path).unwrap();
            let mut data = Vec::new();
            file.read_to_end(&mut data).unwrap();

            if data[0..7] != *b"BLENDER" {
                let mut decoder = Decoder::new(&data[..]).unwrap();
                let mut gzip_data = Vec::new();
                decoder.read_to_end(&mut gzip_data).unwrap();
            
                data = gzip_data;
            }

            blend = Blend::new(&data[..]);
        }

        let unwrapped_blend = blend.unwrap();

        for obj in unwrapped_blend.instances_with_code(*b"OB") {
            if obj.is_valid("data") && obj.get("data").code()[0..=1] == *b"ME" {
                let mesh = obj.get("data");
                let faces = mesh.get_iter("mpoly").collect::<Vec<_>>();
                let loops = mesh.get_iter("mloop").collect::<Vec<_>>();
                let uvs = mesh.get_iter("mloopuv").collect::<Vec<_>>();
                let verts = mesh.get_iter("mvert").collect::<Vec<_>>();
                //let materials = mesh.get_iter("material").collect::<Vec<_>>();

                // Iterate through faces and get indices
                for face in faces {
                    let start = face.get_u32("loopstart") as usize;
                    let count = face.get_u32("totloop") as usize;

                    if count > 0 {
                        // Extract face indices
                        let face_indices = &loops[start..start + count];
                        indices.extend(face_indices.iter().map(|index| index.get_u32("v")));
                    }
                }

                // Now "indices" contains the face indices for the current mesh
                println!("Face Indices: {:?}", indices);

                for i in 0..loops.len() {
                    let loop_data = &loops[i];
                    let uv_data = &uvs[i];
                    let vert_data = &verts[loop_data.get_u32("v") as usize];
    
                    let position = Vec3 {
                        x: vert_data.get_f32_vec("co")[0],
                        y: vert_data.get_f32_vec("co")[1],
                        z: vert_data.get_f32_vec("co")[2],
                    };
                    
                    //let material_index = loop_data.get_u32("mat_nr") as usize;
                    //let texture_id = materials
                    //    .get(material_index)
                    //    .map(|material| material.get_u32("mtex") as u32)
                    //    .unwrap();

                    //let texture_uv = Vec3 {
                    //    x: uv_data.get_f32("uv"),
                    //    y: uv_data.get_f32("uv"),
                    //    z: texture_id as f32,
                    //};

                    let texture_uv = Vec3 {
                        x: uv_data.get_f32_vec("uv")[0],
                        y: uv_data.get_f32_vec("uv")[1],
                        z: 0.0,
                    };
    
                    let normals = Vec3 {
                        x: vert_data.get_i16_vec("no")[0] as f32,
                        y: vert_data.get_i16_vec("no")[1] as f32,
                        z: vert_data.get_i16_vec("no")[2] as f32,
                    };

                    //TODO: Load bone information for the mesh from the blend file.

                    vertices.push(Vertex::new(position, texture_uv, normals));
                }
            }
        }

        MeshData::new(&vertices, &indices)
    }

    pub fn generate_triangle_data() -> MeshData {
        let vertices = [
            Vertex::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.5, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
		    Vertex::new(Vec3::new(-1.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
		    Vertex::new(Vec3::new(1.0, -1.0, 0.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
        ];

        let indices = [0, 1, 2];

        MeshData::new(&vertices.to_vec(), &indices.to_vec())
    }

    pub fn generate_plane_data() -> MeshData {
        let vertices = [
            Vertex::new(Vec3::new(-1.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
            Vertex::new(Vec3::new(-1.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
            Vertex::new(Vec3::new(1.0, -1.0, 0.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
            Vertex::new(Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
        ];
    
        let indices = [0, 1, 2, 2, 3, 0];

        MeshData::new(&vertices.to_vec(), &indices.to_vec())
    }

    pub fn generate_cube_data() -> MeshData {
        let vertices = [
            // Front face
            Vertex::new(Vec3::new(-1.0, -1.0, 1.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)), // 0
            Vertex::new(Vec3::new(1.0, -1.0, 1.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.25, 0.0, 0.0)), // 1
            Vertex::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.25, 0.25, 0.0)), // 2
            Vertex::new(Vec3::new(-1.0, 1.0, 1.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.25, 0.0)), // 3

            // Back face
            Vertex::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.5, 0.0, 0.0)), // 4
            Vertex::new(Vec3::new(1.0, -1.0, -1.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.75, 0.0, 0.0)), // 5
            Vertex::new(Vec3::new(1.0, 1.0, -1.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.75, 0.25, 0.0)), // 6
            Vertex::new(Vec3::new(-1.0, 1.0, -1.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.5, 0.25, 0.0)), // 7
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

        MeshData::new(&vertices.to_vec(), &indices.to_vec())
    }

    pub fn generate_triangle_pyramid_data() -> MeshData {
        let vertices = [
            // Base vertices (same as the triangle)
            Vertex::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.5, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
            Vertex::new(Vec3::new(-1.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
            Vertex::new(Vec3::new(1.0, -1.0, 0.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
            // Apex vertex
            Vertex::new(Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.5, 0.5, 0.0), Vec3::new(0.0, 0.0, 0.0)),
        ];

        // Define the indices for the pyramid
        let indices = [
            // Base triangle (same as the triangle)
            0, 1, 2,
            // Front triangle
            0, 2, 3,
            // Left triangle
            0, 3, 1,
            // Right triangle
            2, 1, 3,
        ];

        MeshData::new(&vertices.to_vec(), &indices.to_vec())
    }

    pub fn generate_square_pyramid_data() -> MeshData {
        let vertices = [
            // Base vertices
            Vertex::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
            Vertex::new(Vec3::new(1.0, -1.0, -1.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
            Vertex::new(Vec3::new(1.0, -1.0, 1.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
            Vertex::new(Vec3::new(-1.0, -1.0, 1.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
            // Apex vertex
            Vertex::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.5, 0.5, 0.0), Vec3::new(0.0, 0.0, 0.0)),
        ];

        // Define the indices for the pyramid
        let indices = [
            // Base square
            0, 1, 2,
            2, 3, 0,
            // Front triangle
            0, 1, 4,
            // Right triangle
            1, 2, 4,
            // Back triangle
            2, 3, 4,
            // Left triangle
            3, 0, 4,
        ];

        MeshData::new(&vertices.to_vec(), &indices.to_vec())
    }

    pub fn calculate_cylindrical_projection(x: f32, y: f32, z: f32, r: f32) -> Vec2 {
        // Convert the Cartesian coordinates to cylindrical coordinates
        let rho = (x.powi(2) + y.powi(2)).sqrt();
        let phi = y.atan2(x);
        let zeta = z;
    
        // Scale and translate the angular coordinate to the range [0, 1]
        let u = (phi + std::f32::consts::PI) / (2.0 * std::f32::consts::PI);
    
        // Scale and translate the height coordinate to the range [0, 1]
        let v = (zeta + r) / (2.0 * r);
    
        // Return the uv coordinates
        Vec2::new(u, v)
    }

    pub fn generate_polygon_data(n: usize, radius: f32) -> MeshData {
        // Initialize an empty vector to store the vertices
        let mut vertices = Vec::new();
        // Initialize an empty vector to store the indices
        let mut indices = Vec::new();
    
        // Calculate the angle increment for each side
        let angle_increment = 2.0 * std::f32::consts::PI / n as f32;
    
        // Loop over the n sides of the polygon
        for i in 0..n {
            // Calculate the angle for the current vertex
            let angle = i as f32 * angle_increment;
    
            // Calculate the x, y, and z coordinates for the current vertex
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            let z = 0.0;
            
            // Calculate the u and v coordinates for the current vertex
            let uv_vec2 = MeshData::calculate_cylindrical_projection(x, y, z, radius);
            let u = uv_vec2.x;
            let v = uv_vec2.y;
    
            // Create a new vertex and push it to the vector
            let vertex = Vertex::new(Vec3::new(x, y, z), Vec3::new(u, v, 0.0), Vec3::new(0.0, 0.0, 0.0));
            vertices.push(vertex);
        }
    
        // Loop over the triangles of the polygon
        for i in 1..n - 1 {
            // Push the indices of each triangle to the vector
            indices.push(0);
            indices.push(i as u32);
            indices.push((i + 1) as u32);
        }
    
        // Return a tuple of two vectors, one with the vertices and one with the indices
        MeshData::new(&vertices, &indices)
    }
}

#[derive(Clone)]
pub struct Mesh {
    pub vao: VAO,
    pub shader_program: ShaderProgram,
    pub index_count: u32,
}

impl Mesh {
    pub fn new(mesh_data: &MeshData, shader_program: &ShaderProgram) -> Self {
        let vertex_array = &mesh_data.vertex_array;
        let index_array = &mesh_data.index_array;

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

        shader_program.use_program(true);

        shader_program.set_uniform_vec_i32("sampler_objs", &vec![
            00, 01, 02, 03, 04, 05, 06, 07,
            08, 09, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ]);

        shader_program.use_program(false);
        vao.bind(false);

        Mesh {
            vao: vao,
            shader_program: shader_program.clone(),
            index_count: index_array.len() as u32,
        }
    }

    pub fn render(&self) {
        self.shader_program.use_program(true);
        self.vao.bind(true);

        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.index_count as i32, gl::UNSIGNED_INT, std::ptr::null());
        }

        self.vao.bind(false);
        self.shader_program.use_program(false);
    }

    pub fn get_mesh_data(&self) -> MeshData {
        unsafe {
            self.vao.bind(true);

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

            self.vao.bind(false);

            MeshData::new_by_value(vertex_data, index_data)
        }
    }

    pub fn delete(&self) -> MeshData {
        let result_data = self.get_mesh_data();

        self.vao.delete();
        
        result_data.clone()
    }

    pub fn new_triangle() -> Self {
        MeshData::generate_triangle_data().build_mesh(&ShaderProgram::default_shader_program())
    }

    pub fn new_plane() -> Self {
        MeshData::generate_plane_data().build_mesh(&ShaderProgram::default_shader_program())
    }

    pub fn new_polygon(sides: usize) -> Self {
        MeshData::generate_polygon_data(sides, 1.0).build_mesh(&ShaderProgram::default_shader_program())
    }

    pub fn new_cube() -> Self {
        MeshData::generate_cube_data().build_mesh(&ShaderProgram::default_shader_program())
    }

    pub fn new_triangle_pyramid() -> Self {
        MeshData::generate_triangle_pyramid_data().build_mesh(&ShaderProgram::default_shader_program())
    }

    pub fn new_square_pyramid() -> Self {
        MeshData::generate_square_pyramid_data().build_mesh(&ShaderProgram::default_shader_program())
    }
}