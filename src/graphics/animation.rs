use crate::graphics::renderable::MatrixBuilder;
pub struct Joint {
    pub id: i32,
    pub name: String,
    pub children: Vec<Box<Joint>>,
    pub local_bind_transform: glm::Mat4,
    pub inverse_bind_transform: glm::Mat4,
}

impl Joint {
    pub fn new(id: i32, name: String) -> Self {
        Joint {
            id: id,
            name: name,
            children: Vec::new(),
            local_bind_transform: MatrixBuilder::identity(1.0),
            inverse_bind_transform: MatrixBuilder::identity(1.0),
        }
    }

    pub fn add_child(&mut self, child: Joint) {
        self.children.push(Box::new(child));
    }

    pub fn calculate_inverse_bind_transform(&mut self, parent_bind_transform: glm::Mat4) {
        let bind_transform = parent_bind_transform * self.local_bind_transform;
        self.inverse_bind_transform = glm::inverse(&bind_transform);

        for child in &mut self.children {
            child.calculate_inverse_bind_transform(bind_transform);
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