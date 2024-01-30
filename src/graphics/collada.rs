use glam::Mat3;
use glam::Quat;
use glam::{Vec2, Vec3, Mat4};
use dae_parser::*;
use crate::graphics::vertex::*;
use crate::graphics::animation::*;

pub struct ColladaLoader;
impl ColladaLoader {
    pub fn load_collada_mesh_data(doc: &Document) -> (Vec<Vertex>, Vec<u32>) {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut normal_array: Vec<Vec3> = Vec::new();
        let mut texture_array: Vec<Vec2> = Vec::new();
        let mut color_array: Vec<Vec3> = Vec::new();

        let mut indices: Vec<u32> = Vec::new();
        let mut normal_indices: Vec<u32> = Vec::new();
        let mut texture_indices: Vec<u32> = Vec::new();
        let mut color_indices: Vec<u32> = Vec::new();

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

                                vertices.push(Vertex::new(&Vec3::new(x, y, z), &Vec3::new(0.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, 0.0)));
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
                let mut found_semantics: (bool, bool, bool) = (false, false, false);

                let mut max_offset = stride - 1;
                for input in inputs.iter() {
                    let offset = input.offset as usize;
                    let semantic = input.semantic.clone().to_string();

                    if semantic == "NORMAL" {
                        normal_offset = offset;
                        found_semantics.0 = true;
                    }

                    if semantic == "TEXCOORD" {
                        texture_offset = offset;
                        found_semantics.1 = true;
                    }

                    if semantic == "COLOR" {
                        color_offset = offset;
                        found_semantics.2 = true;
                    }

                    if offset > max_offset {
                        max_offset = offset;
                    }
                }

                stride = max_offset + 1;
                    
                for i in (0..prim_vec.len()).step_by(stride) {
                    indices.push(prim_vec[i]);

                    if found_semantics.0 {
                        normal_indices.push(prim_vec[i + normal_offset]);
                    }

                    if found_semantics.1 {
                        texture_indices.push(prim_vec[i + texture_offset]);
                    }

                    if found_semantics.2 {
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

        (vertices, indices)
    }

    pub fn load_collada_skeleton(doc: &Document, vertices: &mut Vec<Vertex>) -> (Joint, Vec<Joint>) {
        let mut skin_weights: Vec<f32> = Vec::new();
        let mut joints: Vec<Joint> = Vec::new();

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
                                        let joint = Joint::new(i.try_into().unwrap(), joint_name.as_str());

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
                            let index = prim[last_vertex_weight_index + 2 * j as usize] as usize;
                            println!("Vertex[{0}]: {1}", i, index);
                    
                            bone_ids.push(index as f32);
                            bone_weights_indices.push(prim[last_vertex_weight_index + 2 * j as usize + 1] as f32);
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
                            let mut real_bone_weights_copy = real_bone_weights.clone();
                            
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
                                real_bone_weights_copy[max_weight_index] = 0.0; // Set the max weight to 0
                            }

                            final_bone_ids = max_weights_ids;
                            final_bone_weights = max_weights;

                            let sum = final_bone_weights.x + final_bone_weights.y + final_bone_weights.z;
                            if sum != 0.0 {
                                final_bone_weights = Vec3::new(
                                    final_bone_weights.x / sum,
                                    final_bone_weights.y / sum,
                                    final_bone_weights.z / sum,
                                );
                            }
                        }

                        vertices[i].bone_ids = final_bone_ids;
                        vertices[i].bone_weights = final_bone_weights;

                        println!("Vertex[{0}]: {1}", i, vertices[i].bone_ids);
                        println!("Vertex[{0}]: {1}", i, vertices[i].bone_weights);

                        last_vertex_weight_index += 2 * vcount[i] as usize;
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
                    local_bind_transform: Mat4::IDENTITY, // initialize with identity matrix
                    children: Vec::new(),
                    inverse_bind_transform: Mat4::IDENTITY, // initialize with identity matrix
                    animation_transform: Mat4::IDENTITY, // initialize with identity matrix
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
                        let mut matrix_data_vecs: [[f32; 4]; 4] = [[0.0; 4]; 4];

                        for x in 0..4 {
                            for y in 0..4 {
                                matrix_data_vecs[x][y] = matrix_data[x + y * 4];
                            }
                        }

                        joint.local_bind_transform = Mat4::from_cols_array_2d(&matrix_data_vecs).clone();
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
            local_bind_transform: Mat4::IDENTITY,
            children: Vec::new(),
            inverse_bind_transform: Mat4::IDENTITY,
            animation_transform: Mat4::IDENTITY,
        };

        // Call the post-order traversal function
        for visual_scene in doc.iter::<VisualScene>() {
            for node in visual_scene.clone().nodes {
                if node.id.clone().unwrap().contains("Armature") {
                    for child in node.children {
                        let mut root_joint = Joint {
                            id: 0,
                            name: "".to_string(),
                            local_bind_transform: Mat4::IDENTITY,
                            children: Vec::new(),
                            inverse_bind_transform: Mat4::IDENTITY,
                            animation_transform: Mat4::IDENTITY,
                        };

                        post_order_traversal(&child, &mut root_joint, &joints);
                        final_root_joint = *root_joint.children[0].clone();
                    }
                }
            }
        }

        (final_root_joint, joints)
    }

    pub fn load_collada_animations(doc: &Document, joints: &Vec<Joint>) -> AnimationData {
        //todo: Load animation section data.
        let mut animation_keyframes: Vec<KeyFrame> = Vec::new();
        let mut time_transform_pairs: Vec<(f32, JointTransform)> = Vec::new();
        let mut times_vec: Vec<f32> = Vec::new();

        for animation in doc.iter::<Animation>() {
            let mut joint_names: Vec<String> = Vec::new();
            let mut times: Vec<f32> = Vec::new();
            let mut transforms: Vec<JointTransform> = Vec::new();

            for channel in &animation.channel {
                let target = channel.target.to_string().clone();

                let mut joint_name = String::new();

                for i in 0..joints.len() {
                    if target.contains(joints[i].name.as_str()) {
                        joint_name = joints[i].name.clone();
                        break;
                    }
                }
                joint_names.push(joint_name);
            }
            
            for source in &animation.source {
                let array = source.array.clone().unwrap();

                if source.id.clone().unwrap().contains("output") {

                    match array {
                        ArrayElement::Float(float_array) => {
                            for i in 0..float_array.len() / 16 {
                                let mut float_array_data = [0.0; 16];
                                for j in 0..16 {
                                    float_array_data[j] = float_array[i * 16 + j];
                                }

                                let float_array_data = float_array_data.to_vec();
                                let mut float_array_data_vecs: [[f32; 4]; 4] = [[0.0; 4]; 4];

                                for x in 0..4 {
                                    for y in 0..4 {
                                        float_array_data_vecs[x][y] = float_array_data[x + y * 4];
                                    }
                                }

                                let mat = Mat4::from_cols_array_2d(&float_array_data_vecs).clone();

                                // Extract translation (position)
                                let translation: Vec3 = mat.w_axis.truncate();
                                // Extract scale
                                let scale = glam::Vec3::new(
                                    mat.x_axis.length(),
                                    mat.y_axis.length(),
                                    mat.z_axis.length(),
                                );
                                // Extract rotation as a quaternion
                                let rotation_mat = Mat3::from_cols(
                                    mat.x_axis.truncate() / scale.x,
                                    mat.y_axis.truncate() / scale.y,
                                    mat.z_axis.truncate() / scale.z,
                                );

                                let roation_quat: Quat = Quat::from_mat3(&rotation_mat);
                                let transform = JointTransform::new(joint_names[0].as_str(), &translation, &roation_quat);

                                transforms.push(transform);
                            }
                        },
                        _ => {},
                    }
                }

                else if source.id.clone().unwrap().contains("input") {
                    match array {
                        ArrayElement::Float(float_array) => {
                            for i in 0..float_array.len() {
                                times.push(float_array[i]);
                            }
                        },
                        _ => {},
                    }
                }
            }

            //combine time and transform data and add it to a full vec where we can retrieve into a keyframe based on the time value
            for i in 0..times.len() {
                let time = times[i];
                let transform = transforms[i].clone();

                time_transform_pairs.push((time, transform));
            }

            
            times_vec.append(&mut times.clone());
        }
        
        // Sort the time_transform_pairs by time
        time_transform_pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // Create keyframes from the sorted time_transform_pairs
        let mut transforms_by_time: Vec<(f32, Vec<JointTransform>)> = Vec::new();

        for (time, transform) in time_transform_pairs {
            // Check if there is an existing entry with the same time
            if let Some(entry) = transforms_by_time.iter_mut().find(|(t, _)| *t == time) {
                // Add the joint transform to the existing entry
                entry.1.push(transform);
            } else {
                // Create a new entry with the time and the joint transform
                transforms_by_time.push((time, vec![transform]));
            }
        }

        // Create the keyframes from the sorted time_transform_pairs
        for (time, transforms) in transforms_by_time {
            let keyframe = KeyFrame::new(time, &transforms);

            animation_keyframes.push(keyframe);
        }
        
        AnimationData::new(&animation_keyframes)
    }
}