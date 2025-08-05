use crate::organisms::OrganismType;
use crate::prelude::*;

#[derive(Component)]
pub struct RedAI {
    vision_range: f32,
    target_type: OrganismType,
}

impl RedAI {
    pub fn new(vision_range: f32, target_type: OrganismType) -> Self {
        Self {
            vision_range,
            target_type,
        }
    }

    pub fn vision_range(&self) -> f32 {
        self.vision_range
    }

    pub fn target_type(&self) -> OrganismType {
        self.target_type
    }

    pub fn set_vision_range(&mut self, range: f32) {
        self.vision_range = range;
    }

    pub fn set_target_type(&mut self, target_type: OrganismType) {
        self.target_type = target_type;
    }

    pub fn can_see(&self, distance: f32) -> bool {
        distance <= self.vision_range
    }
}
