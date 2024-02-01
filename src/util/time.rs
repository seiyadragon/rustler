use std::time::{Duration, Instant};


#[derive(Clone)]
pub struct UpdateCounter {
    pub updates_since_program_start: u32,
    pub updates_since_last_second: u32,
    pub last_update_time: Instant,
    pub target_updates_per_second: u32,
    pub achieved_updates_per_second: f32,
    pub delta_time: Duration,
}

impl UpdateCounter {
    pub fn new(target_updates_per_second: u32) -> Self {
        UpdateCounter {
            updates_since_program_start: 0,
            updates_since_last_second: 0,
            last_update_time: Instant::now(),
            target_updates_per_second: target_updates_per_second,
            achieved_updates_per_second: 0.0,
            delta_time: Duration::new(0, 0),
        }
    }
}

pub struct RenderCounter {
    pub frames_since_program_start: u32,
    pub frames_since_last_second: u32,
    pub last_frame_time: Instant,
    pub target_frames_per_second: u32,
    pub achieved_frames_per_second: f32,
    pub delta_time: Duration,
}

impl RenderCounter {
    pub fn new(target_frames_per_second: u32) -> Self {
        RenderCounter {
            frames_since_program_start: 0,
            frames_since_last_second: 0,
            last_frame_time: Instant::now(),
            target_frames_per_second: target_frames_per_second,
            achieved_frames_per_second: 0.0,
            delta_time: Duration::new(0, 0),
        }
    }
}