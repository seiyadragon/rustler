use std::any::Any;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem;
use std::ops::Deref;
use std::ops::DerefMut;
use std::path;
use std::rc::Rc;
use std::slice;
use crate::graphics::vertex::*;
use crate::graphics::shader::*;
use super::math::MatrixBuilder;
use gl::types::GLint;
use glm::Vec2;
use glm::Vec3;
use dae_parser::*;
use glm::Vec4;
use glm::Vector3;
use crate::graphics::animation::*;

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

    pub fn generate_from_collada(path: &str) -> MeshData {
        let doc = Document::from_file(path).unwrap();
        
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut normal_array: Vec<Vec3> = Vec::new();
        let mut texture_array: Vec<Vec2> = Vec::new();
        let mut color_array: Vec<Vec3> = Vec::new();

        let mut indices: Vec<u32> = Vec::new();
        let mut normal_indices: Vec<u32> = Vec::new();
        let mut texture_indices: Vec<u32> = Vec::new();
        let mut color_indices: Vec<u32> = Vec::new();

        let mut skin_weights: Vec<f32> = Vec::new();
        let mut joints: Vec<Joint> = Vec::new();

        for geometry in doc.iter::<Geometry>() {
            let mesh = geometry.element.as_mesh().unwrap();
            
            for source in &mesh.sources {
                if source.id.clone().unwrap().contains("position") {
                    let positions = source.array.clone().unwrap();

                    match positions {
                        ArrayElement::Float(positions) => {
                            for i in 0..positions.len() / 3 {
                                let x = positions[i * 3];
                                let y = positions[i * 3 + 1];
                                let z = positions[i * 3 + 2];

                                vertices.push(Vertex::new(Vec3::new(x, y, z), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)));
                            }
                        },
                        _ => {},
                    }
                }

                if source.id.clone().unwrap().contains("normal") {
                    let normals = source.array.clone().unwrap();

                    match normals {
                        ArrayElement::Float(normals) => {
                            for i in 0..normals.len() / 3 {
                                let x = normals[i * 3];
                                let y = normals[i * 3 + 1];
                                let z = normals[i * 3 + 2];

                                normal_array.push(Vec3::new(x, y, z));
                            }
                        },
                        _ => {},
                    }
                }

                if source.id.clone().unwrap().contains("map") {
                    let tex_coords = source.array.clone().unwrap();

                    match tex_coords {
                        ArrayElement::Float(tex_coords) => {
                            for i in 0..tex_coords.len() / 2 {
                                let u = tex_coords[i * 2];
                                let v = tex_coords[i * 2 + 1];

                                texture_array.push(Vec2::new(u, v));
                            }
                        },
                        _ => {},
                    }
                }

                if source.id.clone().unwrap().contains("color") {
                    let colors = source.array.clone().unwrap();

                    match colors {
                        ArrayElement::Float(colors) => {
                            for i in 0..colors.len() / 4 {
                                let r = colors[i * 4];
                                let g = colors[i * 4 + 1];
                                let b = colors[i * 4 + 2];

                                color_array.push(Vec3::new(r, g, b));
                            }
                        },
                        _ => {},
                    }
                }
            }

            for element in &mesh.elements {
                let poly_list_opt = element.as_polylist();
                let triangles_opt = element.as_triangles();

                let mut primitives: Box<[u32]> = Box::new([]);
                let mut inputs = InputList::new(vec![]);

                if poly_list_opt.is_some() {
                    let poly_list = poly_list_opt.unwrap();

                    primitives = poly_list.data.clone().prim;
                    inputs = poly_list.inputs.clone();
                }

                else if triangles_opt.is_some() {
                    let triangles = triangles_opt.unwrap();

                    primitives = triangles.data.clone().prim.unwrap();
                    inputs = triangles.inputs.clone();
                }

                let prim_vec = primitives.to_vec();
                let mut stride = 3 as usize;
                let mut normal_offset = 1 as usize;
                let mut texture_offset = 2 as usize;
                let mut color_offset = 3 as usize;
                let mut found_semantics: Vector3<bool> = Vector3::new(false, false, false);

                let mut max_offset = stride - 1;
                for input in inputs.iter() {
                    let offset = input.offset as usize;
                    let semantic = input.semantic.clone().to_string();

                    if semantic == "NORMAL" {
                        normal_offset = offset;
                        found_semantics.x = true;
                    }

                    if semantic == "TEXCOORD" {
                        texture_offset = offset;
                        found_semantics.y = true;
                    }

                    if semantic == "COLOR" {
                        color_offset = offset;
                        found_semantics.z = true;
                    }

                    if offset > max_offset {
                        max_offset = offset;
                    }
                }

                stride = max_offset + 1;
                    
                for i in (0..prim_vec.len()).step_by(stride) {
                    indices.push(prim_vec[i]);

                    if found_semantics.x {
                        normal_indices.push(prim_vec[i + normal_offset]);
                    }

                    if found_semantics.y {
                        texture_indices.push(prim_vec[i + texture_offset]);
                    }

                    if found_semantics.z {
                        color_indices.push(prim_vec[i + color_offset]);
                    }
                }
                
            }
        }

        for i in 0..indices.len() {
            let vertex_index = indices[i] as usize;
            let mut normal_index = 0 as usize;
            let mut texture_index = 0 as usize;
            let mut color_index = 0 as usize;

            if normal_indices.len() > 0 {
                normal_index = normal_indices[i] as usize;
            }

            if texture_indices.len() > 0 {
                texture_index = texture_indices[i] as usize;
            }

            if color_indices.len() > 0 {
                color_index = color_indices[i] as usize;
            }

            let mut vertex = vertices[vertex_index].clone();

            let mut normal = Vec3::new(0.0, 0.0, 0.0);
            let mut texture = Vec2::new(0.0, 0.0);
            let mut color = Vec3::new(1.0, 1.0, 1.0);

            if normal_array.len() > 0 && normal_index < normal_array.len() {
                normal = normal_array[normal_index].clone();
            }

            if texture_array.len() > 0 && texture_index < texture_array.len() {
                texture = texture_array[texture_index].clone();
            }

            if color_array.len() > 0 && color_index < color_array.len() {
                color = color_array[color_index].clone();
            }

            vertex.normals = normal;
            vertex.texture = Vec3::new(texture.x, texture.y, 0.0);
            vertex.color = color;

            vertices[vertex_index] = vertex.clone();
        }

        for controller in doc.iter::<Controller>() {
            match &controller.element {
                ControlElement::Skin(skin) => {
                    for source in &skin.sources {
                        if source.id.clone().unwrap().contains("weights") {
                            let weights = source.array.clone().unwrap();

                            match weights {
                                ArrayElement::Float(weights) => {
                                    for i in 0..weights.len() {
                                        skin_weights.push(weights[i]);
                                    }
                                },
                                _ => {},
                            }
                        }

                        if source.id.clone().unwrap().contains("joints") {
                            let joints_array = source.array.clone().unwrap();

                            match joints_array {
                                ArrayElement::Name(joints_array) => {
                                    for i in 0..joints_array.len() {
                                        let joint_name = joints_array[i].clone();
                                        let joint = Joint::new(i.try_into().unwrap(), joint_name);

                                        joints.push(joint);
                                    }
                                },
                                _ => {},
                            }
                        }
                    }

                    let vertex_weights = skin.weights.clone();

                    let vcount = vertex_weights.vcount.clone().to_vec();
                    let prim = vertex_weights.prim.clone().to_vec();

                    let mut last_vertex_weight_index = 0;

                    for i in 0..vcount.len() {
                        let mut bone_ids: Vec<f32> = Vec::new();
                        let mut bone_weights_indices: Vec<f32> = Vec::new();

                        for j in 0..vcount[i] {
                            bone_ids.push(prim[last_vertex_weight_index + j as usize] as f32);
                            bone_weights_indices.push(prim[last_vertex_weight_index + j as usize + 1] as f32);
                        }

                        
                        let mut real_bone_weights: Vec<f32> = Vec::new();
                        for i in 0..bone_weights_indices.len() {
                            real_bone_weights.push(skin_weights[bone_weights_indices[i] as usize]);
                        }

                        let mut final_bone_ids = Vec3::new(0.0, 0.0, 0.0);
                        let mut final_bone_weights = Vec3::new(0.0, 0.0, 0.0);

                        if vcount[i] < 4 {
                            for j in 0..vcount[i] {
                                final_bone_ids[j as usize] = bone_ids[j as usize];
                                final_bone_weights[j as usize] = real_bone_weights[j as usize];
                            }
                        }

                        else if vcount[i] >= 4 {
                            let real_bone_weights_copy = real_bone_weights.clone();
                            
                            let mut max_weights: Vec3 = Vec3::new(0.0, 0.0, 0.0);
                            let mut max_weights_ids: Vec3 = Vec3::new(0.0, 0.0, 0.0);

                            for index in 0..3 {
                                let mut max_weight = 0.0;
                                let mut max_weight_index: usize = 0;

                                for j in 0..real_bone_weights_copy.len() {
                                    if real_bone_weights_copy[j] > max_weight {
                                        max_weight = real_bone_weights_copy[j];
                                        max_weight_index = j;
                                    }
                                }

                                max_weights[index] = max_weight;
                                max_weights_ids[index] = bone_ids[max_weight_index];
                            }

                            final_bone_weights = glm::normalize(max_weights);
                            final_bone_ids = max_weights_ids;
                        }

                        vertices[i].bone_ids = final_bone_ids;
                        vertices[i].bone_weights = final_bone_weights;

                        last_vertex_weight_index += vcount[i] as usize;
                    }
                }
                ControlElement::Morph(morph) => {}
            }
        }

        fn post_order_traversal(child: &Node, parent_joint: &mut Joint, joint_list: &Vec<Joint>) {
            if child.ty == NodeType::Joint {
                let name = child.id.clone().unwrap();
                let mut joint = Joint {
                    id: 0,
                    name: name.clone(),
                    local_bind_transform: MatrixBuilder::identity(1.0), // initialize with identity matrix
                    children: Vec::new(),
                    inverse_bind_transform: MatrixBuilder::identity(1.0), // initialize with identity matrix
                };

                for i in 0..joint_list.len() {
                    if joint_list[i].name == name {
                        joint.id = joint_list[i].id;
                        break;
                    }
                }

                for transform in &child.transforms {
                    if let Transform::Matrix(matrix) = transform {
                        let matrix = matrix.clone();
                        let matrix_data = (*matrix.0).clone();
                        let mut matrix_data_vecs: [Vec4; 4] = [Vec4::new(0.0, 0.0, 0.0, 0.0); 4];

                        for x in 0..4 {
                            for y in 0..4 {
                                matrix_data_vecs[x][y] = matrix_data[x + y * 4];
                            }
                        }

                        joint.local_bind_transform = glm::Mat4::from_array(&matrix_data_vecs).clone();
                    }
                }

                for c in &child.children {
                    post_order_traversal(c, &mut joint, joint_list);
                }

                parent_joint.children.push(Box::new(joint));
            }
        }

        let mut final_root_joint = Joint {
            id: 0,
            name: "".to_string(),
            local_bind_transform: MatrixBuilder::identity(1.0),
            children: Vec::new(),
            inverse_bind_transform: MatrixBuilder::identity(1.0),
        };

        // Call the post-order traversal function
        for visual_scene in doc.iter::<VisualScene>() {
            for node in visual_scene.clone().nodes {
                if node.id.clone().unwrap().contains("Armature") {
                    for child in node.children {
                        let mut root_joint = Joint {
                            id: 0,
                            name: "".to_string(),
                            local_bind_transform: MatrixBuilder::identity(1.0),
                            children: Vec::new(),
                            inverse_bind_transform: MatrixBuilder::identity(1.0),
                        };

                        post_order_traversal(&child, &mut root_joint, &joints);
                        final_root_joint = *root_joint.children[0].clone();
                    }
                }
            }
        }

        final_root_joint.calculate_inverse_bind_transform(final_root_joint.local_bind_transform.clone());

        println!("Joints: {}", &joints.len());
        for joint in &joints {
            println!("Joint: {}", joint.name);
        }
        println!("---------------------------------");
        println!("---------------------------------");
        final_root_joint.print_joint("-");
        
        //todo: Load animation section data.
        let animation_times: Vec<f32> = Vec::new();
        for animation in doc.iter::<Animation>() {
            for source in &animation.sources() {
                if source.id.clone().unwrap().contains("input") {
                    
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
            Vertex::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)), // 0
            Vertex::new(Vec3::new(1.0, 1.0, -1.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.25, 0.0, 0.0)), // 1
            Vertex::new(Vec3::new(1.0, -1.0, 1.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.25, 0.25, 0.0)), // 2
            Vertex::new(Vec3::new(1.0, -1.0, -1.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.25, 0.0)), // 3

            // Back face
            Vertex::new(Vec3::new(-1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.5, 0.0, 0.0)), // 4
            Vertex::new(Vec3::new(-1.0, 1.0, -1.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.75, 0.0, 0.0)), // 5
            Vertex::new(Vec3::new(-1.0, -1.0, 1.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.75, 0.25, 0.0)), // 6
            Vertex::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.5, 0.25, 0.0)), // 7
        ];

        let indices = [
            // Front face
            4, 2, 0, // First triangle
            2, 7, 3, // Second triangle
            // Back face
            6, 5, 7, // First triangle
            1, 7, 5, // Second triangle
            // Right face
            0, 3, 1, // First triangle
            4, 1, 5, // Second triangle
            // Left face
            4, 6, 2, // First triangle
            2, 6, 7, // Second triangle
            // Top face
            6, 4, 5, // First triangle
            1, 3, 7, // Second triangle
            // Bottom face
            0, 2, 3, // First triangle
            4, 0, 1, // Second triangle
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

    pub fn new_collada(path: &str) -> Self {
        MeshData::generate_from_collada(path).build_mesh(&ShaderProgram::default_shader_program())
    }
}