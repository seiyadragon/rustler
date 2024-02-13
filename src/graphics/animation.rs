use std::time::Duration;

use glam::{Vec3, Mat4, Quat};

use crate::graphics::texture::Texture;

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

    pub fn interpolate(&self, other: &JointTransform, progression: f32) -> JointTransform {
        let position = self.position.lerp(other.position, progression);
        let rotation = self.rotation.slerp(other.rotation, progression);

        JointTransform {
            joint_name: self.joint_name.clone(),
            position: position,
            rotation: rotation,
        }
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
pub struct Animation {
    pub key_frames: Vec<KeyFrame>,
}

impl Animation {
    pub fn new(key_frames: &Vec<KeyFrame>) -> Self {
        Self {
            key_frames: key_frames.clone(),
        }
    }

    pub fn print_animation(&self) {
        println!("AnimationData:");
        for key_frame in &self.key_frames {
            key_frame.print_keyframe();
        }
    }

    // Allows you to apply a pose to a mesh at a given time. The poses refer to the keyframes in the animation.
    // The time is going to be a float value that represents both the current keyframe index as well as how far it is from the next index.
    // Eg. 0.0 would be the first keyframe, 0.5 would be halfway between the first and second keyframes, and 1.0 would be the second keyframe.
    // It achieves this by using the floor of the time to get the current keyframe index and the fractional part of the time to get the interpolation value.
    pub fn apply_pose_to_mesh(&self, time: f32, skeleton: &mut Joint, parent_transform: &Mat4) {
        let joint_transforms: Vec<JointTransform> = self.key_frames[time.floor() as usize].pose.clone();

        let mut next_joint_transforms_index = time.floor() as usize + 1;

        if next_joint_transforms_index >= self.key_frames.len() {
            next_joint_transforms_index = 0;
        }

        let next_joint_transforms: Vec<JointTransform> = self.key_frames[next_joint_transforms_index].pose.clone();

        let actual_joint_transforms: Vec<JointTransform> = joint_transforms.iter().map(|joint| {
            let next_joint = next_joint_transforms.iter().find(|next_joint| next_joint.joint_name == joint.joint_name).unwrap();
            joint.interpolate(next_joint, time.fract())
        }).collect();

        let current_joint_transform = actual_joint_transforms.iter().find(|joint| joint.joint_name == skeleton.name).unwrap();

        let current_local_pose = current_joint_transform.get_local_transform();
        let mut current_global_pose = *parent_transform * current_local_pose;

        for child in &mut skeleton.children {
            self.apply_pose_to_mesh(time, child, &current_global_pose);
        }

        current_global_pose = current_global_pose * skeleton.inverse_bind_transform;
        skeleton.animation_transform = current_global_pose;
    }
}

#[derive(Clone)]
pub struct AnimationPlayer {
    pub animation: Animation,
    pub paused: bool,
    pub animation_time: f32,
    pub skeleton: Joint,
}

impl AnimationPlayer {
    pub fn new(animation: &Animation, skeleton: &Joint) -> Self {
        Self {
            animation: animation.clone(),
            animation_time: 0.0,
            paused: false,
            skeleton: skeleton.clone(),
        }
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn pause_to_pose(&mut self, time: &Duration) {
        self.animation_time = time.as_secs_f32();
        self.animation.apply_pose_to_mesh(time.as_secs_f32(), &mut self.skeleton, &Mat4::IDENTITY);
        self.paused = true;
    }

    pub fn reset(&mut self) {
        self.animation_time = 0.0;
        self.animation.apply_pose_to_mesh(0.0, &mut self.skeleton, &Mat4::IDENTITY);
    }

    pub fn animate(&mut self, delta: &Duration, duration: &Duration) {
        if !self.paused {
            self.animation_time += delta.as_secs_f32();

            if self.animation_time > duration.as_secs_f32() {
                self.animation_time = 0.0;
            }

            let scale = (self.animation.key_frames.len() - 1) as f32;
            let scaled_animation_time = self.animation_time * scale / duration.as_secs_f32();

            self.animation.apply_pose_to_mesh(scaled_animation_time, &mut self.skeleton, &Mat4::IDENTITY);
        }
    }
}

#[derive(Clone)]
pub struct SpriteAnimation {
    pub frames: Vec<Texture>,
    pub current_frame: usize,
    pub elapsed_time: Duration,
}

impl SpriteAnimation {
    pub fn new(frames: &Vec<Texture>) -> Self {
        Self {
            frames: frames.clone(),
            current_frame: 0,
            elapsed_time: Duration::from_secs(0),
        }
    }

    pub fn animate(&mut self, delta: &Duration, length: &Duration) {
        let frame_time = Duration::from_secs_f32(length.as_secs_f32() / self.frames.len() as f32);

        if &self.elapsed_time >= &frame_time {
            self.current_frame += 1;

            if self.current_frame >= self.frames.len() {
                self.current_frame = 0;
            }

            self.elapsed_time = Duration::from_secs(0);
        } else {
            self.elapsed_time += *delta;
        }
    }

    pub fn get_current_frame(&self) -> &Texture {
        &self.frames[self.current_frame]
    }
}