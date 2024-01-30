use glam::{Vec3, Mat4, Quat};

pub struct Joint {
    pub id: i32,
    pub name: String,
    pub children: Vec<Box<Joint>>,
    pub local_bind_transform: Mat4,
    pub inverse_bind_transform: Mat4,
    pub animation_transform: Mat4,
}

impl Joint {
    pub fn new(id: i32, name: &str) -> Self {
        Joint {
            id: id,
            name: String::from(name),
            children: Vec::new(),
            local_bind_transform: Mat4::IDENTITY,
            inverse_bind_transform: Mat4::IDENTITY,
            animation_transform: Mat4::IDENTITY,
        }
    }

    pub fn add_child(&mut self, child: &Joint) {
        self.children.push(Box::new(child.clone()));
    }

    pub fn calculate_inverse_bind_transform(&mut self, parent_bind_transform: &Mat4) {
        let bind_transform = *parent_bind_transform * self.local_bind_transform;
        self.inverse_bind_transform = bind_transform.inverse();
    
        for child in &mut self.children {
            child.calculate_inverse_bind_transform(&bind_transform);
        }
    }

    pub fn flatten(&self) -> Vec<Joint> {
        let mut joints: Vec<Joint> = Vec::new();
        joints.push(self.clone());

        for child in &self.children {
            let mut child_joints = child.flatten();
            joints.append(&mut child_joints);
        }

        joints
    }

    pub fn get_global_transform_matrices(&self) -> Vec<Mat4> {
        let mut joint_matrices: Vec<Mat4> = Vec::new();
        let animation_transform = self.animation_transform;

        joint_matrices.push(animation_transform);

        for child in &self.children {
            joint_matrices.append(&mut child.get_global_transform_matrices());
        }

        joint_matrices
    }

    pub fn print_joint(&self, pre: &str) {
        println!("{pre}Joint[{0}]: {1}", self.id, self.name);
        let string_pre = String::from(pre);
        let binding = string_pre + pre;
        let new_pre = binding.as_str();

        for child in &self.children {
            child.print_joint(new_pre);
        }
    }
}

impl Clone for Joint {
    fn clone(&self) -> Self {
        Joint {
            id: self.id,
            name: self.name.clone(),
            children: self.children.iter().map(|child| child.clone()).collect(),
            local_bind_transform: self.local_bind_transform.clone(),
            inverse_bind_transform: self.inverse_bind_transform.clone(),
            animation_transform: self.animation_transform.clone(),
        }
    }
}

#[derive(Clone)]
pub struct JointTransform {
    pub joint_name: String,
    pub position: Vec3,
    pub rotation: Quat,
}

impl JointTransform {
    pub fn new(joint_name: &str, position: &Vec3, rotation: &Quat) -> Self {
        JointTransform {
            joint_name: String::from(joint_name),
            position: position.clone(),
            rotation: rotation.clone(),
        }
    }

    pub fn get_local_transform(&self) -> Mat4 {
        let translation = Mat4::from_translation(self.position);
        let rotation = Mat4::from_quat(self.rotation);
        translation * rotation
    }
}

impl std::fmt::Display for JointTransform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[JointTransform({0}): position: {1}, rotation: {2}]", self.joint_name, self.position, self.rotation)
    }
}

#[derive(Clone)]
pub struct KeyFrame {
    pub time_stamp: f32,
    pub pose: Vec<JointTransform>,
}

impl KeyFrame {
    pub fn new(time_stamp: f32, pose: &Vec<JointTransform>) -> Self {
        KeyFrame {
            time_stamp: time_stamp,
            pose: pose.clone(),
        }
    }

    pub fn print_keyframe(&self) {
        println!("KeyFrame[{}]:", self.time_stamp);
        for joint in &self.pose {
            println!("\t{}", joint);
        }
    }
}

#[derive(Clone)]
pub struct AnimationData {
    pub length: f32,
    pub key_frames: Vec<KeyFrame>,
}

impl AnimationData {
    pub fn new(key_frames: &Vec<KeyFrame>) -> Self {
        let mut length = 0.0;
        for key_frame in key_frames {
            if key_frame.time_stamp > length {
                length = key_frame.time_stamp;
            }
        }

        AnimationData {
            length: length,
            key_frames: key_frames.clone(),
        }
    }

    pub fn print_animation(&self) {
        println!("AnimationData:");
        for key_frame in &self.key_frames {
            key_frame.print_keyframe();
        }
    }

    pub fn apply_keyframe_to_joints(&self, keyframe: usize, skeleton: &mut Joint, parent_transform: &Mat4) {
        let mut joint_transforms: Vec<JointTransform> = self.key_frames[keyframe].pose.clone();

        let current_local_pose = joint_transforms.iter().find(|joint| joint.joint_name == skeleton.name).unwrap().get_local_transform();
        let mut current_global_pose = *parent_transform * current_local_pose;

        for child in &mut skeleton.children {
            self.apply_keyframe_to_joints(keyframe, child, &current_global_pose);
        }

        current_global_pose = current_global_pose * skeleton.inverse_bind_transform;
        skeleton.animation_transform = current_global_pose;
    }
}