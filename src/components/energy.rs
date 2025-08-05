use crate::prelude::*;

#[derive(Component)]
pub struct Energy {
    current: f32,
    max: f32,
    movement_cost: f32,
    regen_rate: f32,
}

impl Energy {
    pub fn new(max_energy: f32, movement_cost: f32, regen_rate: f32) -> Self {
        Self {
            current: max_energy,
            max: max_energy,
            movement_cost,
            regen_rate,
        }
    }

    pub fn current(&self) -> f32 {
        self.current
    }

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn movement_cost(&self) -> f32 {
        self.movement_cost
    }

    pub fn regen_rate(&self) -> f32 {
        self.regen_rate
    }

    pub fn set_current(&mut self, energy: f32) {
        self.current = energy.clamp(0.0, self.max);
    }

    pub fn set_max(&mut self, max_energy: f32) {
        self.max = max_energy;
        if self.current > self.max {
            self.current = self.max;
        }
    }

    pub fn set_movement_cost(&mut self, cost: f32) {
        self.movement_cost = cost;
    }

    pub fn set_regen_rate(&mut self, rate: f32) {
        self.regen_rate = rate;
    }

    pub fn consume(&mut self, amount: f32) -> bool {
        if self.current >= amount {
            self.current -= amount;
            true
        } else {
            false
        }
    }

    pub fn consume_movement(&mut self) -> bool {
        self.consume(self.movement_cost)
    }

    pub fn regenerate(&mut self, delta_seconds: f32) {
        self.current = (self.current + self.regen_rate * delta_seconds).min(self.max);
    }

    pub fn ratio(&self) -> f32 {
        if self.max <= 0.0 {
            0.0
        } else {
            self.current / self.max
        }
    }

    pub fn is_depleted(&self) -> bool {
        self.current <= 0.0
    }

    pub fn is_low(&self, threshold: f32) -> bool {
        self.ratio() < threshold
    }

    pub fn can_move(&self) -> bool {
        self.current >= self.movement_cost
    }
}
