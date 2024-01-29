use glam::{Vec3, Mat4, Quat};

pub struct Joint {
    pub id: i32,
    pub name: String,
    pub children: Vec<Box<Joint>>,
    pub local_bind_transform: Mat4,
    pub inverse_bind_transform: Mat4,
}

impl Joint {
    pub fn new(id: i32, name: &str) -> Self {
        Joint {
            id: id,
            name: String::from(name),
            children: Vec::new(),
            local_bind_transform: Mat4::IDENTITY,
            inverse_bind_transform: Mat4::IDENTITY,
        }
    }

    pub fn add_child(&mut self, child: &Joint) {
        self.children.push(Box::new(child.clone()));
    }

    pub fn calculate_inverse_bind_transform(&mut self, parent_bind_transform: &Mat4) {
        let bind_transform = parent_bind_transform.clone() * self.local_bind_transform;
        self.inverse_bind_transform = self.local_bind_transform.inverse();

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

    pub fn get_joint_matrices(&self) -> Vec<Mat4> {
        let mut joint_matrices: Vec<Mat4> = Vec::new();

        joint_matrices.push(self.inverse_bind_transform);

        for child in &self.children {
            joint_matrices.append(&mut child.get_joint_matrices());
        }

        joint_matrices
    }

    pub fn apply_joint_transform(&mut self, joint_transform: &JointTransform) {
        if self.name == joint_transform.joint {
            let translation = Mat4::from_translation(joint_transform.position);
            let rotation = Mat4::from_quat(joint_transform.rotation);
            self.local_bind_transform = translation * rotation;
        }

        for child in &mut self.children {
            child.apply_joint_transform(joint_transform);
        }
    }

    pub fn apply_keyframe(&mut self, keyframe: &KeyFrame) {
        for joint_transform in &keyframe.pose {
            self.apply_joint_transform(joint_transform);
        }
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
        }
    }
}

#[derive(Clone)]
pub struct JointTransform {
    pub joint: String,
    pub position: Vec3,
    pub rotation: Quat,
}

impl JointTransform {
    pub fn new(joint: &str, position: &Vec3, rotation: &Quat) -> Self {
        JointTransform {
            joint: String::from(joint),
            position: position.clone(),
            rotation: rotation.clone(),
        }
    }
}

impl std::fmt::Display for JointTransform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[JointTransform: name: {0}, position: {1}, rotation: {2}]", self.joint, self.position, self.rotation)
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
    pub key_frames: Vec<KeyFrame>,
}

impl AnimationData {
    pub fn new(key_frames: &Vec<KeyFrame>) -> Self {
        AnimationData {
            key_frames: key_frames.clone(),
        }
    }

    pub fn print_animation(&self) {
        println!("AnimationData:");
        for key_frame in &self.key_frames {
            key_frame.print_keyframe();
        }
    }
}