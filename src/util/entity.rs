use crate::graphics::window::*;
use crate::graphics::view::*;

pub trait Entity {
    fn init(&mut self);
    fn render(&mut self, graphics: &mut GraphicsLayer);
    fn update(&mut self, event_queue: &mut EventQueue, input: &mut Input);
    fn exit(&mut self);
}

pub struct EntityManager {
    pub entity_list: Vec<Box<dyn Entity>>,
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            entity_list: Vec::new(),
        }
    }

    pub fn push(&mut self, mut entity: Box<dyn Entity>) {
        entity.init();
        self.entity_list.push(entity);
    }

    pub fn pop(&mut self, index: usize) -> Box<dyn Entity> {
        let mut entity = self.entity_list.remove(index);
        entity.exit();

        entity
    }

    pub fn render(&mut self, graphics: &mut GraphicsLayer) {
        let boxed_self = Box::new(self);

        for i in 0..boxed_self.entity_list.len() {
            boxed_self.entity_list[i].render(graphics);
        }
    }

    pub fn update(&mut self, event_queue: &mut EventQueue, input: &mut Input) {
        let boxed_self = Box::new(self);

        for i in 0..boxed_self.entity_list.len() {
            boxed_self.entity_list[i].update(event_queue, input);
        }
    }
}