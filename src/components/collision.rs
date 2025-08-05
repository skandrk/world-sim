use crate::prelude::*;

#[derive(Component)]
pub struct Collision {
    radius: f32,
}

impl Collision {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    pub fn is_colliding_with(&self, other: &Collision, distance: f32) -> bool {
        distance < (self.radius + other.radius)
    }

    pub fn min_distance_to(&self, other: &Collision) -> f32 {
        self.radius + other.radius
    }
}
