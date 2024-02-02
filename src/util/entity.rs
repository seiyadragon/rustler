
use std::any::Any;
use std::collections::HashMap;
use std::time::Duration;

use crate::graphics::view::GraphicsLayer;
use crate::Window;
use super::event::EventQueue;
use super::event::Input;

pub struct Entity {
    pub children: Vec<Entity>,
    pub variables: EntityVariableArray,
    pub window: Option<Box<Window>>,

    pub init_func: fn(entity: &mut Entity),
    pub render_func: fn(entity: &mut Entity, graphics: &mut GraphicsLayer),
    pub update_func: fn(entity: &mut Entity, event_queue: &mut EventQueue, input: &mut Input, delta: &Duration),
    pub exit_func: fn(entity: &mut Entity),
}

impl Entity {
    pub fn new() -> Self {
        Entity {
            children: Vec::new(),
            variables: EntityVariableArray::new(),
            window: None,

            init_func: |entity: &mut Entity| {},
            render_func: |entity: &mut Entity, graphics: &mut GraphicsLayer| {},
            update_func: |entity: &mut Entity, event_queue: &mut EventQueue, input: &mut Input, delta: &Duration| {},
            exit_func: |entity: &mut Entity| {},
        }
    }

    pub fn with_init(mut self, init_func: fn(entity: &mut Entity)) -> Self {
        self.init_func = init_func;
        self
    }

    pub fn with_render(mut self, render_func: fn(entity: &mut Entity, graphics: &mut GraphicsLayer)) -> Self {
        self.render_func = render_func;
        self
    }

    pub fn with_update(mut self, update_func: fn(entity: &mut Entity, event_queue: &mut EventQueue, input: &mut Input, delta: &Duration)) -> Self {
        self.update_func = update_func;
        self
    }

    pub fn with_exit(mut self, exit_func: fn(entity: &mut Entity)) -> Self {
        self.exit_func = exit_func;
        self
    }

    // Parent Entity has ownership over all its children.
    pub fn push(&mut self, mut child: Entity) {
        child.init();
        self.children.push(child);
    }

    pub fn pop(&mut self, index: usize) -> Entity {
        let mut child = self.children.remove(index);
        child.exit();

        child
    }

    pub fn init(&mut self) {
        (self.init_func)(self);
    }

    pub fn render(&mut self, graphics: &mut GraphicsLayer) {
        for i in 0..self.children.len() {
            self.children[i].render(graphics);
        }

        (self.render_func)(self, graphics);
    }

    pub fn update(&mut self, event_queue: &mut EventQueue, input: &mut Input, delta: &Duration) {
        for i in 0..self.children.len() {
            self.children[i].update(event_queue, input, delta);
        }

        (self.update_func)(self, event_queue, input, delta);
    }

    pub fn exit(&mut self) {
        (self.exit_func)(self);
    }
}

pub struct EntityVariableArray {
    pub variables: HashMap<String, Box<dyn Any>>,
}

impl EntityVariableArray {
    pub fn new() -> Self {
        EntityVariableArray {
            variables: HashMap::new(),
        }
    }

    pub fn declare<T: 'static>(&mut self, name: &str, value: T) -> &mut T {
        self.variables.insert(name.to_string(), Box::new(value));

        self.get(name)
    }

    pub fn delete<T: 'static>(&mut self, name: &str) -> T {
        *self.variables.remove(name).unwrap().downcast().unwrap()
    }

    pub fn get<T: 'static>(&mut self, name: &str) -> &mut T {
        let ptr = self.variables.get_mut(name).unwrap();

        ptr.downcast_mut::<T>().unwrap()
    }

    pub fn len(&self) -> usize {
        self.variables.len()
    }
}