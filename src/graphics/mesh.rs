use dae_parser::*;
use glam::Mat4;
use crate::graphics::vertex::*;
use crate::graphics::shader::*;
use crate::graphics::animation::*;
use crate::graphics::collada::ColladaLoader;

#[derive(Clone)]
pub enum MeshData {
    StaticMeshData(StaticMeshData),
    AnimatedMeshData(AnimatedMeshData),
}

#[derive(Clone)]
pub struct StaticMeshData {
    pub vertex_array: Vec<Vertex>,
    pub index_array: Vec<u32>,
}

impl StaticMeshData {
    pub fn new(vertex_array: &Vec<Vertex>, index_array: &Vec<u32>) -> Self {
        Self {
            vertex_array: vertex_array.clone(),
            index_array: index_array.clone(),
        }
    }
    
    pub fn from_collada(path: &str) -> Self {
        let doc = Document::from_file(path).unwrap();
        let (mut vertices, indices) = ColladaLoader::load_collada_mesh_data(&doc);

        Self::new(&vertices, &indices)
    }

    pub fn build(self, shader_program: &ShaderProgram) -> StaticMesh {
        StaticMesh::new(&self, shader_program)
    }

    pub fn break_down(self) -> (Vec<Vertex>, Vec<u32>) {
        (self.vertex_array, self.index_array)
    }
}

#[derive(Clone)]
pub struct AnimatedMeshData {
    pub vertex_array: Vec<Vertex>,
    pub index_array: Vec<u32>,
    pub skeleton: Joint,
    pub animation: AnimationData,
}

impl AnimatedMeshData {
    pub fn new(vertex_array: &Vec<Vertex>, index_array: &Vec<u32>, skeleton: &Joint, animation: &AnimationData) -> Self {
        Self {
            vertex_array: vertex_array.clone(),
            index_array: index_array.clone(),
            skeleton: skeleton.clone(),
            animation: animation.clone(),
        }
    }

    pub fn from_collada(path: &str) -> AnimatedMeshData {
        let doc = Document::from_file(path).unwrap();
        let (mut vertices, indices) = ColladaLoader::load_collada_mesh_data(&doc);
        let (mut root_joint, joints) = ColladaLoader::load_collada_skeleton(&doc, &mut vertices);
        let animation = ColladaLoader::load_collada_animations(&doc, &joints);
        
        root_joint.calculate_inverse_bind_transform(&Mat4::IDENTITY);

        AnimatedMeshData::new(&vertices, &indices, &root_joint, &animation)
    }

    pub fn build(self, shader_program: &ShaderProgram) -> AnimatedMesh {
        AnimatedMesh::new(&self, shader_program)
    }

    pub fn break_down(self) -> (Vec<Vertex>, Vec<u32>, Joint, AnimationData) {
        (self.vertex_array, self.index_array, self.skeleton, self.animation)
    }
}

#[derive(Clone)]
pub enum Mesh {
    StaticMesh(StaticMesh),
    AnimatedMesh(AnimatedMesh),
}

#[derive(Clone)]
pub struct StaticMesh {
    pub vao: VAO,
    pub shader_program: ShaderProgram,
    pub index_count: usize,
}

impl StaticMesh {
    pub fn new(mesh_data: &StaticMeshData, shader_program: &ShaderProgram) -> Self {
        let vao = VAO::build_from_data(&mesh_data.vertex_array, &mesh_data.index_array, shader_program);

        Self {
            vao: vao,
            shader_program: shader_program.clone(),
            index_count: mesh_data.index_array.len(),
        }
    }

    pub fn render(&self) {
        self.shader_program.use_program(true);
        self.vao.render(self.index_count);
        self.shader_program.use_program(false);
    }

    pub fn get_mesh_data(&self) -> StaticMeshData {
        let (vertices, indices) = self.vao.get_data();
        StaticMeshData::new(&vertices, &indices)
    }

    pub fn delete(&self) -> StaticMeshData {
        let (vertices, indices) = self.vao.delete();

        StaticMeshData::new(&vertices, &indices)
    }
}

#[derive(Clone)]
pub struct AnimatedMesh {
    pub vao: VAO,
    pub shader_program: ShaderProgram,
    pub skeleton: Joint,
    pub animation: AnimationData,
    pub index_count: usize,
}

impl AnimatedMesh {
    pub fn new(mesh_data: &AnimatedMeshData, shader_program: &ShaderProgram) -> Self {
        let vao = VAO::build_from_data(&mesh_data.vertex_array, &mesh_data.index_array, shader_program);

        Self {
            vao: vao,
            shader_program: shader_program.clone(),
            skeleton: mesh_data.skeleton.clone(),
            animation: mesh_data.animation.clone(),
            index_count: mesh_data.index_array.len(),
        }
    }

    pub fn render(&self) {
        self.shader_program.use_program(true);
        self.vao.render(self.index_count);
        self.shader_program.use_program(false);
    }

    pub fn get_mesh_data(&self) -> AnimatedMeshData {
        let (vertices, indices) = self.vao.get_data();
        AnimatedMeshData::new(&vertices, &indices, &self.skeleton, &self.animation)
    }

    pub fn delete(&self) -> AnimatedMeshData {
        let (vertices, indices) = self.vao.delete();

        AnimatedMeshData::new(&vertices, &indices, &self.skeleton, &self.animation)
    }

}