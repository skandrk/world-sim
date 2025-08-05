use crate::prelude::*;

#[derive(Component)]
pub struct Position {
    x: f32,
    y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn distance_to(&self, other: &Position) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn clamp_to_world(&mut self, world_size: f32) {
        self.x = self.x.clamp(0.0, world_size);
        self.y = self.y.clamp(0.0, world_size);
    }
}
