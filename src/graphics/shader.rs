use gl::types::*;
use glam::{Vec2, Vec3, Vec4, IVec2, IVec3, IVec4, Mat4};
use core::{f32, fmt};
use std::{collections::HashMap, error::Error, fmt::Display, ptr::{null, null_mut}};
use std::ffi::CString;
use std::str;

#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    VERTEX = gl::VERTEX_SHADER as isize,
    FRAGMENT = gl::FRAGMENT_SHADER as isize,
    GEOMETRY = gl::GEOMETRY_SHADER as isize,
}

#[derive(Debug, Clone)]
pub struct ShaderClosure {
    pub constants: Vec<String>,
    pub variables: Vec<String>,
    pub instructions: Vec<String>,
    pub closures: Vec<ShaderClosure>,
}

impl ShaderClosure {
    pub fn new() -> ShaderClosure {
        ShaderClosure {
            constants: Vec::new(),
            variables: Vec::new(),
            instructions: Vec::new(),
            closures: Vec::new(),
        }
    }

    pub fn dec_const(&mut self, type_name: &str, name: &str, value: &str) {
        self.constants.push(format!("const {} {} = {};", type_name, name, value).replace(";", "") + ";");
    }

    pub fn with_dec_const(mut self, type_name: &str, name: &str, value: &str) -> ShaderClosure {
        self.dec_const(type_name, name, value);
        self
    }

    pub fn dec_var(&mut self, type_name: &str, name: &str, value: &str) {
        self.variables.push(format!("{} {} = {};", type_name, name, value).replace(";", "") + ";");
    }

    pub fn with_dec_var(mut self, type_name: &str, name: &str, value: &str) -> ShaderClosure {
        self.dec_var(type_name, name, value);
        self
    }

    pub fn call_function(&mut self, function_name: &str, args: &Vec<&str>) {
        let mut call_string = String::from(function_name) + "(";

        for (i, arg) in args.iter().enumerate() {
            call_string.push_str(arg);

            if i < args.len() - 1 {
                call_string.push_str(", ");
            }
        }

        call_string.push_str(");");

        self.instructions.push(call_string);
    }

    pub fn with_call_function(mut self, function_name: &str, args: &Vec<&str>) -> ShaderClosure {
        self.call_function(function_name, args);
        self
    }

    pub fn do_action(&mut self, instruction: &str) {
        self.instructions.push(String::from(instruction).replace(";", "") + ";");
    }

    pub fn with_do_action(mut self, instruction: &str) -> ShaderClosure {
        self.do_action(instruction);
        self
    }

    pub fn if_statement(&mut self, condition: &str, action: &ShaderClosure) {
        self.instructions.push(format!("if ({}) {{", condition));
        self.instructions.push(String::from(action.to_string().as_str()));
        self.instructions.push(String::from("}"));
    }

    pub fn with_if_statement(mut self, condition: &str, action: &ShaderClosure) -> ShaderClosure {
        self.if_statement(condition, action);
        self
    }

    pub fn else_if_statement(&mut self, condition: &str, action: &ShaderClosure) {
        self.instructions.push(format!("else if ({}) {{", condition));
        self.instructions.push(String::from(action.to_string().as_str()));
        self.instructions.push(String::from("}"));
    }

    pub fn with_else_if_statement(mut self, condition: &str, action: &ShaderClosure) -> ShaderClosure {
        self.else_if_statement(condition, action);
        self
    }

    pub fn else_statement(&mut self, action: &ShaderClosure) {
        self.instructions.push(String::from("else {"));
        self.instructions.push(String::from(action.to_string().as_str()));
        self.instructions.push(String::from("}"));
    }

    pub fn with_else_statement(mut self, action: &ShaderClosure) -> ShaderClosure {
        self.else_statement(action);
        self
    }

    pub fn return_statement(&mut self, value: &str) {
        self.instructions.push(format!("return {};", value).replace(";", "") + ";");
    }

    pub fn with_return_statement(mut self, value: &str) -> ShaderClosure {
        self.return_statement(value);
        self
    }

    pub fn dec_for_loop(&mut self, expr: &str, action: &ShaderClosure) {
        self.instructions.push(format!("for ({}) {{", expr));
        self.instructions.push(String::from(action.to_string().as_str()));
        self.instructions.push(String::from("}"));
    }

    pub fn with_for_loop(mut self, expr: &str, action: &ShaderClosure) -> ShaderClosure {
        self.dec_for_loop(expr, action);
        self
    }

    pub fn dec_while_loop(&mut self, expr: &str, action: &ShaderClosure) {
        self.instructions.push(format!("while ({}) {{", expr));
        self.instructions.push(String::from(action.to_string().as_str()));
        self.instructions.push(String::from("}"));
    }

    pub fn with_while_loop(mut self, expr: &str, action: &ShaderClosure) -> ShaderClosure {
        self.dec_while_loop(expr, action);
        self
    }

    pub fn dec_closure(&mut self, closure: &ShaderClosure) {
        self.closures.push(closure.clone());
    }

    pub fn with_closure(mut self, closure: &ShaderClosure) -> ShaderClosure {
        self.dec_closure(closure);
        self
    }

    pub fn to_string(&self) -> String {
        let mut closure_string = String::new();
        closure_string.push_str("\n");

        for constant in &self.constants {
            closure_string.push_str(constant.as_str());
            closure_string.push_str("\n");
        }

        for variable in &self.variables {
            closure_string.push_str(variable.as_str());
            closure_string.push_str("\n");
        }

        for instruction in &self.instructions {
            closure_string.push_str(instruction.as_str());
            closure_string.push_str("\n");
        }

        for closure in &self.closures {
            closure_string.push_str(closure.to_string().as_str());
            closure_string.push_str("\n");
        }

        closure_string
    }
}

impl Display for ShaderClosure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct ShaderFunction {
    pub name: String,
    pub return_type: String,
    pub arguments: Vec<String>,
    pub closure: ShaderClosure,
}

impl ShaderFunction {
    pub fn new(return_type: &str, name: &str, args: &Vec<String>) -> ShaderFunction {
        ShaderFunction {
            name: String::from(name),
            return_type: String::from(return_type),
            arguments: args.clone(),
            closure: ShaderClosure::new(),
        }
    }

    pub fn dec_const(&mut self, type_name: &str, name: &str, value: &str) {
        self.closure.dec_const(type_name, name, value);
    }

    pub fn with_dec_const(mut self, type_name: &str, name: &str, value: &str) -> ShaderFunction {
        self.dec_const(type_name, name, value);
        self
    }

    pub fn dec_var(&mut self, type_name: &str, name: &str, value: &str) {
        self.closure.dec_var(type_name, name, value);
    }

    pub fn with_dec_var(mut self, type_name: &str, name: &str, value: &str) -> ShaderFunction {
        self.dec_var(type_name, name, value);
        self
    }

    pub fn call_function(&mut self, function_name: &str, args: &Vec<&str>) {
        self.closure.call_function(function_name, args);
    }

    pub fn with_call_function(mut self, function_name: &str, args: &Vec<&str>) -> ShaderFunction {
        self.call_function(function_name, args);
        self
    }

    pub fn do_action(&mut self, instruction: &str) {
        self.closure.do_action(instruction);
    }

    pub fn with_do_action(mut self, instruction: &str) -> ShaderFunction {
        self.do_action(instruction);
        self
    }

    pub fn if_statement(&mut self, condition: &str, action: &ShaderClosure) {
        self.closure.if_statement(condition, action);
    }

    pub fn with_if_statement(mut self, condition: &str, action: &ShaderClosure) -> ShaderFunction {
        self.if_statement(condition, action);
        self
    }

    pub fn else_if_statement(&mut self, condition: &str, action: &ShaderClosure) {
        self.closure.else_if_statement(condition, action);
    }

    pub fn with_else_if_statement(mut self, condition: &str, action: &ShaderClosure) -> ShaderFunction {
        self.else_if_statement(condition, action);
        self
    }

    pub fn else_statement(&mut self, action: &ShaderClosure) {
        self.closure.else_statement(action);
    }

    pub fn with_else_statement(mut self, action: &ShaderClosure) -> ShaderFunction {
        self.else_statement(action);
        self
    }

    pub fn return_statement(&mut self, value: &str) {
        self.closure.return_statement(value);
    }

    pub fn with_return_statement(mut self, value: &str) -> ShaderFunction {
        self.return_statement(value);
        self
    }

    pub fn dec_for_loop(&mut self, expr: &str, action: &ShaderClosure) {
        self.closure.dec_for_loop(expr, action);
    }

    pub fn with_for_loop(mut self, expr: &str, action: &ShaderClosure) -> ShaderFunction {
        self.dec_for_loop(expr, action);
        self
    }

    pub fn dec_while_loop(&mut self, expr: &str, action: &ShaderClosure) {
        self.closure.dec_while_loop(expr, action);
    }

    pub fn with_while_loop(mut self, expr: &str, action: &ShaderClosure) -> ShaderFunction {
        self.dec_while_loop(expr, action);
        self
    }

    pub fn with_closure(mut self, closure: &ShaderClosure) -> ShaderFunction {
        self.closure = closure.clone();
        self
    }

    pub fn to_string(&self) -> String {
        let mut function_string = String::new();

        function_string.push_str(format!("{} {}(", self.return_type, self.name).as_str());

        for (i, arg) in self.arguments.iter().enumerate() {
            function_string.push_str(arg.as_str());

            if i < self.arguments.len() - 1 {
                function_string.push_str(", ");
            }
        }

        function_string.push_str(") {");
        function_string.push_str(self.closure.to_string().as_str());
        function_string.push_str("}");

        function_string
    }
}

impl Display for ShaderFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct ShaderBuilder {
    pub version: String,
    pub constants: Vec<String>,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub variables: Vec<String>,
    pub uniforms: Vec<String>,

    //TODO: Gotta figure these out later
    pub functions: HashMap<String, ShaderFunction>,
    pub main: ShaderFunction,
}

impl ShaderBuilder {
    pub fn new(version: &str) -> ShaderBuilder {
        ShaderBuilder {
            version: String::from(version),
            constants: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            variables: Vec::new(),
            uniforms: Vec::new(),
            functions: HashMap::new(),
            main: ShaderFunction::new("void", "main", &vec![]),
        }
    }

    pub fn dec_const(&mut self, type_name: &str, name: &str, value: &str) {
        self.constants.push(format!("const {} {} = {};", type_name, name, value).replace(";", "") + ";");
    }

    pub fn dec_in(&mut self, location: usize, type_name: &str, name: &str) {
        self.inputs.push(format!("layout (location = {}) in {} {};", location, type_name, name).replace(";", "") + ";");
    }

    pub fn dec_in_no_location(&mut self, type_name: &str, name: &str) {
        self.inputs.push(format!("in {} {};", type_name, name).replace(";", "") + ";");
    }

    pub fn dec_out(&mut self, type_name: &str, name: &str) {
        self.outputs.push(format!("out {} {};", type_name, name).replace(";", "") + ";");
    }

    pub fn dec_var(&mut self, type_name: &str, name: &str, value: &str) {
        self.variables.push(format!("{} {} = {};", type_name, name, value).replace(";", "") + ";");
    }

    pub fn dec_uniform(&mut self, type_name: &str, name: &str) {
        self.uniforms.push(format!("uniform {} {};", type_name, name).replace(";", "") + ";");
    }

    pub fn dec_function(&mut self, function: &ShaderFunction) {
        self.functions.insert(function.name.clone(), function.clone());
    }

    pub fn to_string(&self) -> String {
        let mut shader_string = String::new();
        shader_string.push_str(&self.version.as_str());
        shader_string.push_str("\n");

        for constant in &self.constants {
            shader_string.push_str(constant.as_str());
            shader_string.push_str("\n");
        }

        for input in &self.inputs {
            shader_string.push_str(input.as_str());
            shader_string.push_str("\n");
        }

        for output in &self.outputs {
            shader_string.push_str(output.as_str());
            shader_string.push_str("\n");
        }

        for uniform in &self.uniforms {
            shader_string.push_str(uniform.as_str());
            shader_string.push_str("\n");
        }

        for variable in &self.variables {
            shader_string.push_str(variable.as_str());
            shader_string.push_str("\n");
        }

        for function in &self.functions {
            shader_string.push_str(function.1.to_string().as_str());
            shader_string.push_str("\n");
        }

        shader_string.push_str(self.main.to_string().as_str());

        shader_string
    }

    pub fn build(&self, shader_type: &ShaderType) -> ShaderSource {
        ShaderSource::from_source_string(shader_type, &self.to_string())
    }
}

impl Display for ShaderBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub struct ShaderBuilderTemplate {}
impl ShaderBuilderTemplate {
    pub fn basic_vertex_shader(version: &str) -> ShaderBuilder {
        let mut shader_builder = ShaderBuilder::new(version);

        shader_builder.dec_in(0, "vec3", "in_position");
        shader_builder.dec_in(1, "vec3", "in_tex_coords");
        shader_builder.dec_in(2, "vec3", "in_normal");
        shader_builder.dec_in(3, "vec3", "in_bone_ids");
        shader_builder.dec_in(4, "vec3", "in_bone_weights");
        shader_builder.dec_in(5, "vec3", "in_color");

        shader_builder.dec_out("vec3", "vertex_position");
        shader_builder.dec_out("vec3", "tex_coords");
        shader_builder.dec_out("vec3", "vertex_normals");
        shader_builder.dec_out("vec3", "vertex_color");

        shader_builder.dec_uniform("mat4", "mvp");

        shader_builder.main.do_action("gl_Position = mvp * vec4(in_position, 1.0)");

        shader_builder.main.do_action("vertex_position = in_position");
        shader_builder.main.do_action("tex_coords = in_tex_coords");
        shader_builder.main.do_action("vertex_normals = in_normal");
        shader_builder.main.do_action("vertex_color = in_color");

        shader_builder
    }

    pub fn animated_vertex_shader(version: &str) -> ShaderBuilder {
        /*
        #version 450 core

        const int MAX_BONES = 100;
        const int MAX_WEIGHTS = 3;
        
        layout (location = 0) in vec3 in_position;
        layout (location = 1) in vec3 in_tex_coords;
        layout (location = 2) in vec3 in_normal;
        layout (location = 3) in vec3 in_bone_ids;
        layout (location = 4) in vec3 in_bone_weights;
        layout (location = 5) in vec3 in_color;
        
        out vec3 vertex_position;
        out vec3 tex_coords;
        out vec3 vertex_normals;
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
                    mat4 joint_transform = joint_transforms[int(in_bone_ids[i])];
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
            vertex_normals = in_normal;
            vertex_color = in_color;
            vertex_position = gl_Position.xyz;
        }
        */

        let mut shader_builder = ShaderBuilder::new(version);
            
        shader_builder.dec_const("int", "MAX_BONES", "100");
        shader_builder.dec_const("int", "MAX_WEIGHTS", "3");

        shader_builder.dec_in(0, "vec3", "in_position");
        shader_builder.dec_in(1, "vec3", "in_tex_coords");
        shader_builder.dec_in(2, "vec3", "in_normal");
        shader_builder.dec_in(3, "vec3", "in_bone_ids");
        shader_builder.dec_in(4, "vec3", "in_bone_weights");
        shader_builder.dec_in(5, "vec3", "in_color");

        shader_builder.dec_out("vec3", "vertex_position");
        shader_builder.dec_out("vec3", "tex_coords");
        shader_builder.dec_out("vec3", "vertex_normals");
        shader_builder.dec_out("vec3", "vertex_color");

        shader_builder.dec_uniform("mat4", "mvp");
        shader_builder.dec_uniform("mat4", "joint_transforms[MAX_BONES]");

        shader_builder.main.dec_var("bool", "should_animate", "false");
        shader_builder.main.dec_var("vec4", "total_local_pos", "vec4(0.0)");

        shader_builder.main.if_statement(
            "in_bone_ids.x != 0.0 || in_bone_ids.y != 0.0 || in_bone_ids.z != 0.0", 
            &ShaderClosure::new()
                .with_do_action("should_animate = true")
        );
                
        shader_builder.main.if_statement(
            "should_animate", &ShaderClosure::new()
                .with_dec_var("vec4", "total_local_pos", "vec4(0.0)")
                .with_dec_var("vec4", "total_normal", "vec4(0.0)")
                .with_for_loop("int i = 0; i < MAX_WEIGHTS; i++", &ShaderClosure::new()
                    .with_dec_var("mat4", "joint_transform", "joint_transforms[int(in_bone_ids[i])]")
                    .with_dec_var("vec4", "pos_position", "joint_transform * vec4(in_position, 1.0)")
                    .with_do_action("total_local_pos += pos_position * in_bone_weights[i]")
                    .with_dec_var("vec4", "world_normal", "joint_transform * vec4(in_normal, 0.0)")
                    .with_do_action("total_normal += world_normal * in_bone_weights[i]")
                )
                .with_do_action("gl_Position = mvp * total_local_pos")
        );
                
        shader_builder.main.else_statement(
            &ShaderClosure::new().with_do_action("gl_Position = mvp * vec4(in_position, 1.0)")
        );
        
        shader_builder.main.do_action("vertex_position = in_position");
        shader_builder.main.do_action("tex_coords = in_tex_coords");
        shader_builder.main.do_action("vertex_normals = in_normal");
        shader_builder.main.do_action("vertex_color = in_color");

        shader_builder
    }

    pub fn basic_fragment_shader(version: &str) -> ShaderBuilder {
        let mut shader_builder = ShaderBuilder::new(version);

        shader_builder.dec_in_no_location("vec3", "vertex_position");
        shader_builder.dec_in_no_location("vec3", "tex_coords");
        shader_builder.dec_in_no_location("vec3", "vertex_normals");
        shader_builder.dec_in_no_location("vec3", "vertex_color");

        shader_builder.dec_out("vec4", "frag_color");

        shader_builder.main.do_action("frag_color = vec4(vertex_color, 1.0)");

        shader_builder
    }

    pub fn texture_fragment_shader(version: &str) -> ShaderBuilder {
        /*
        #version 450 core

        in vec3 vertex_position;
        in vec3 tex_coords;
        in vec3 vertex_normals;
        in vec3 vertex_color;

        out vec4 output_color;

        uniform bool should_sample_texture;
        uniform sampler2D sampler_objs[32];

        void main() {
            highp int sampler_index = int(tex_coords.z);

            if (should_sample_texture) {
                output_color = texture(sampler_objs[sampler_index], tex_coords.xy);
            } else {
                output_color = vec4(vertex_color, 1.0);
            }
        }
         */

        let mut shader_builder = ShaderBuilder::new(version);

        shader_builder.dec_in_no_location("vec3", "vertex_position");
        shader_builder.dec_in_no_location("vec3", "tex_coords");
        shader_builder.dec_in_no_location("vec3", "vertex_normals");
        shader_builder.dec_in_no_location("vec3", "vertex_color");

        shader_builder.dec_out("vec4", "output_color");

        shader_builder.dec_uniform("bool", "should_sample_texture");
        shader_builder.dec_uniform("sampler2D", "sampler_objs[32]");

        shader_builder.main.dec_var("highp int", "sampler_index", "int(tex_coords.z)");

        shader_builder.main.if_statement(
            "should_sample_texture", &ShaderClosure::new()
                .with_do_action("output_color = texture(sampler_objs[sampler_index], tex_coords.xy)")
        );

        shader_builder.main.else_statement(
            &ShaderClosure::new()
                .with_do_action("output_color = vec4(vertex_color, 1.0)")
        );

        shader_builder
    }


    //TODO: Fix this: I'm bad at lighting lol
    /*
    pub fn lighting_fragment_shader(version: &str) -> ShaderBuilder {
        let mut shader_builder = ShaderBuilder::new(version);

        shader_builder.dec_const("int", "MAX_LIGHTS", "10");

        shader_builder.dec_in_no_location("vec3", "vertex_position");
        shader_builder.dec_in_no_location("vec3", "tex_coords");
        shader_builder.dec_in_no_location("vec3", "vertex_normals");
        shader_builder.dec_in_no_location("vec3", "vertex_color");

        shader_builder.dec_out("vec4", "output_color");

        shader_builder.dec_uniform("bool", "should_sample_texture");
        shader_builder.dec_uniform("sampler2D", "sampler_objs[32]");
        shader_builder.dec_uniform("vec3", "light_sources[MAX_LIGHTS]");
        shader_builder.dec_uniform("vec3", "light_colors[MAX_LIGHTS]");

        shader_builder.dec_uniform("vec3", "light_position_uni");

        shader_builder.main.dec_var("vec3", "normal", "normalize(vertex_normals.xyz)");

        shader_builder.main.dec_var("vec3", "light_position", "vec3(0.0, 0.0, 10.0)");
        shader_builder.main.dec_var("vec3", "light_color", "light_position_uni");
        shader_builder.main.dec_var("vec3", "light_direction", "normalize(light_position - vertex_position)");
        shader_builder.main.dec_var("float", "ambient_light", "0.1");

        shader_builder.main.dec_var("float", "diffuse_strength", "max(0.0, dot(light_direction, normal))");
        shader_builder.main.dec_var("vec3", "diffuse_color", "diffuse_strength * light_color");

        shader_builder.main.dec_var("vec3", "lighting", "ambient_light + diffuse_color");

        shader_builder.main.dec_var("highp int", "sampler_index", "int(tex_coords.z)");

        shader_builder.main.dec_var("vec3", "final_vertex_color", "vec3(0.0)");

        shader_builder.main.if_statement(
            "should_sample_texture", &ShaderClosure::new()
                .with_do_action("final_vertex_color = texture(sampler_objs[sampler_index], tex_coords.xy).xyz")
        );

        shader_builder.main.else_statement(
            &ShaderClosure::new()
                .with_do_action("final_vertex_color = vertex_color")
        );

        shader_builder.main.do_action("output_color = vec4(final_vertex_color * lighting, 1.0)");

        println!("{}", shader_builder.to_string());

        shader_builder
    }
    */
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

    pub fn set_uniform_vec4_vec(&self, name: &str, vec: &Vec<Vec4>) {
        let c_str = CString::new(name.as_bytes()).unwrap();

        self.use_program(true);

        unsafe {
            gl::Uniform4fv(gl::GetUniformLocation(self.program_id, c_str.as_ptr()), vec.len() as i32, vec.as_ptr() as *const f32);
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