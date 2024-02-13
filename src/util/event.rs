use std::collections::VecDeque;
use glam::DVec2;
use glam::UVec2;
use winit::event::Event;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;
use winit::keyboard::PhysicalKey;
use winit::platform::scancode::PhysicalKeyExtScancode;

pub struct EventQueue {
    pub internal_queue: VecDeque<Event<()>>,
}

impl EventQueue {
    pub fn new() -> Self {
        EventQueue {
            internal_queue: VecDeque::new(),
        }
    }

    pub fn process_events(&mut self, func: fn(event: Event<()>)) {
        for event in self.internal_queue.clone() {
            func(event);

            self.internal_queue.pop_front();
        }

        self.internal_queue = VecDeque::<Event<()>>::new();
    }

    pub fn ignore_events(&mut self) {
        self.internal_queue = VecDeque::<Event<()>>::new();
    }

    pub fn push(&mut self, event: Event<()>) {
        self.internal_queue.push_back(event);
    }

    pub fn len(&self) -> usize {
        self.internal_queue.len()
    }
}

pub struct Input {
    pub keys: [bool; 255],
    pub keys_last: [bool; 255],
    pub buttons: [bool; 32],
    pub buttons_last: [bool; 32],
    pub mouse_position: DVec2,
    pub mouse_position_last: DVec2,
}

impl Input {
    pub fn new() -> Self {
        let keys: [bool; 255] = [false; 255];
        let keys_last: [bool; 255] = [false; 255];

        let buttons: [bool; 32] = [false; 32];
        let buttons_last: [bool; 32] = [false; 32];

        Input {
            keys: keys,
            keys_last: keys_last,
            buttons: buttons,
            buttons_last: buttons_last,
            mouse_position: DVec2::new(0.0, 0.0),
            mouse_position_last: DVec2::new(0.0, 0.0),
        }
    }

    pub fn set_key_pressed(&mut self, scancode: u32) {
        if scancode as usize >= self.keys.len() {
            return;
        }

        self.keys[scancode as usize] = true;
    }

    pub fn set_key_unpressed(&mut self, scancode: u32) {
        if scancode as usize >= self.keys.len() {
            return;
        }

        self.keys[scancode as usize] = false;
    }

    pub fn set_button_pressed(&mut self, button: u32) {
        if button as usize >= self.buttons.len() {
            return;
        }

        self.buttons[button as usize] = true;
    }

    pub fn set_button_unpressed(&mut self, button: u32) {
        if button as usize >= self.buttons.len() {
            return;
        }

        self.buttons[button as usize] = false;
    }

    pub fn set_mouse_position(&mut self, position: &DVec2, window_size: &UVec2) {
        //We need to shift the mouse position to be centered at the origin
        let position = DVec2::new(position.x - window_size.x as f64 / 2.0, window_size.y as f64 / 2.0 - position.y);

        self.mouse_position = position;
    }

    pub fn update(&mut self) {
        for i in 0..self.keys.len() {
            self.keys_last[i] = self.keys[i];
        }

        for i in 0..self.buttons.len() {
            self.buttons_last[i] = self.buttons[i];
        }

        self.mouse_position_last = self.mouse_position;
    }

    pub fn key_to_scancode(key: KeyCode) -> u32 {
        PhysicalKey::Code(key).to_scancode().unwrap()
    }

    pub fn mouse_button_to_index(button: MouseButton) -> u32 {
        match button {
            MouseButton::Left => { 0 },
            MouseButton::Right => { 1 },
            MouseButton::Middle => { 2 },
            MouseButton::Back => { 3 },
            MouseButton::Forward => { 4 },
            MouseButton::Other(num) => { num as u32 },
        }
    }

    pub fn was_key_just_pressed(&self, key: KeyCode) -> bool {
        if Self::key_to_scancode(key) as usize >= self.keys.len() {
            return false;
        }

        self.keys[Self::key_to_scancode(key) as usize] && !self.keys_last[Self::key_to_scancode(key) as usize]
    }

    pub fn was_key_just_released(&self, key: KeyCode) -> bool {
        if Self::key_to_scancode(key) as usize >= self.keys.len() {
            return false;
        }

        !self.keys[Self::key_to_scancode(key) as usize] && self.keys_last[Self::key_to_scancode(key) as usize]
    }

    pub fn is_key_being_held_down(&self, key: KeyCode) -> bool {
        if Self::key_to_scancode(key) as usize >= self.keys.len() {
            return false;
        }

        self.keys[Self::key_to_scancode(key) as usize]
    }

    pub fn was_button_just_pressed(&self, button: MouseButton) -> bool {
        if Self::mouse_button_to_index(button) as usize >= self.buttons.len() {
            return false;
        }

        self.buttons[Self::mouse_button_to_index(button) as usize] && !self.buttons_last[Self::mouse_button_to_index(button) as usize]
    }

    pub fn was_button_just_released(&self, button: MouseButton) -> bool {
        if Self::mouse_button_to_index(button) as usize >= self.buttons.len() {
            return false;
        }

        !self.buttons[Self::mouse_button_to_index(button) as usize] && self.buttons_last[Self::mouse_button_to_index(button) as usize]
    }

    pub fn is_button_being_held(&self, button: MouseButton) -> bool {
        if Self::mouse_button_to_index(button) as usize >= self.buttons.len() {
            return false;
        }

        self.buttons[Self::mouse_button_to_index(button) as usize]
    }

    pub fn get_mouse_position(&self) -> DVec2 {
        self.mouse_position
    }

    pub fn get_mouse_speed_per_frame(&self) -> DVec2 {
        self.mouse_position - self.mouse_position_last
    }

    pub fn mouse_is_moving(&self) -> bool {
        self.mouse_position != self.mouse_position_last
    }

    pub fn mouse_is_moving_in_direction(&self, direction: DVec2) -> bool {
        (self.mouse_position - self.mouse_position_last).normalize() == direction
    }

    pub fn mouse_is_stopped(&self) -> bool {
        self.mouse_position == self.mouse_position_last
    }

    pub fn mouse_is_at_position(&self, position: DVec2) -> bool {
        self.mouse_position == position
    }

    pub fn mouse_is_at_position_with_tolerance(&self, position: DVec2, tolerance: f32) -> bool {
        (self.mouse_position - position).length() < tolerance as f64
    }

    pub fn mouse_is_in_area(&self, position: DVec2, size: DVec2) -> bool {
        self.mouse_position.x > position.x && self.mouse_position.x < position.x + size.x && self.mouse_position.y > position.y && self.mouse_position.y < position.y + size.y
    }

    pub fn mouse_is_in_area_with_tolerance(&self, position: DVec2, size: DVec2, tolerance: f32) -> bool {
        self.mouse_position.x > position.x - tolerance as f64 && self.mouse_position.x < position.x + size.x + tolerance as f64 && self.mouse_position.y > position.y - tolerance as f64 && self.mouse_position.y < position.y + size.y + tolerance as f64
    }

}