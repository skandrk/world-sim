use crate::prelude::*;

#[derive(Component)]
pub struct Movement {
    max_jump_distance: f32,
    move_timer: Timer,
}

impl Movement {
    pub fn new(max_jump_distance: f32, interval: f32) -> Self {
        Self {
            max_jump_distance,
            move_timer: Timer::from_seconds(interval, TimerMode::Repeating),
        }
    }

    pub fn max_jump_distance(&self) -> f32 {
        self.max_jump_distance
    }

    pub fn set_max_jump_distance(&mut self, distance: f32) {
        self.max_jump_distance = distance;
    }

    pub fn timer(&self) -> &Timer {
        &self.move_timer
    }

    pub fn timer_mut(&mut self) -> &mut Timer {
        &mut self.move_timer
    }

    pub fn tick(&mut self, delta: std::time::Duration) {
        self.move_timer.tick(delta);
    }

    pub fn just_finished(&self) -> bool {
        self.move_timer.just_finished()
    }

    pub fn reset(&mut self) {
        self.move_timer.reset();
    }
}
