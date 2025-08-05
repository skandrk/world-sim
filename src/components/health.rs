use crate::prelude::*;

#[derive(Component)]
pub struct Health {
    current: f32,
    max: f32,
}

impl Health {
    pub fn new(max_health: f32) -> Self {
        Self {
            current: max_health,
            max: max_health,
        }
    }

    pub fn current(&self) -> f32 {
        self.current
    }

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn set_current(&mut self, health: f32) {
        self.current = health.clamp(0.0, self.max);
    }

    pub fn set_max(&mut self, max_health: f32) {
        self.max = max_health;
        if self.current > self.max {
            self.current = self.max;
        }
    }

    pub fn take_damage(&mut self, damage: f32) {
        self.current = (self.current - damage).max(0.0);
    }

    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn ratio(&self) -> f32 {
        if self.max <= 0.0 {
            0.0
        } else {
            self.current / self.max
        }
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }

    pub fn is_low(&self, threshold: f32) -> bool {
        self.ratio() < threshold
    }
}
