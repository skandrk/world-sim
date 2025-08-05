use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrganismType {
    Blue,
    Red,
}

impl OrganismType {
    pub fn color(&self) -> Color {
        match self {
            OrganismType::Blue => Color::rgb(0.2, 0.4, 1.0),
            OrganismType::Red => Color::rgb(1.0, 0.2, 0.2),
        }
    }

    pub fn max_jump_distance(&self) -> f32 {
        let mut rng = thread_rng();
        match self {
            OrganismType::Blue => 8.0 * rng.gen_range(0.9..=1.1), // Agile prey
            OrganismType::Red => 15.0 * rng.gen_range(0.9..=1.1), // Faster predator
        }
    }

    pub fn collision_radius(&self) -> f32 {
        match self {
            OrganismType::Blue => 8.0, // Bigger prey
            OrganismType::Red => 4.0,  // Smaller predator
        }
    }

    pub fn max_health(&self) -> f32 {
        match self {
            OrganismType::Blue => 100.0,
            OrganismType::Red => 80.0,
        }
    }

    pub fn max_energy(&self) -> f32 {
        match self {
            OrganismType::Blue => 100.0, // High stamina for escaping
            OrganismType::Red => 80.0,   // Lower stamina
        }
    }

    pub fn movement_cost(&self) -> f32 {
        match self {
            OrganismType::Blue => 1.0, // Efficient movement
            OrganismType::Red => 2.0,  // High energy cost
        }
    }

    pub fn energy_regen(&self) -> f32 {
        match self {
            OrganismType::Blue => 3.0, // Fast recovery
            OrganismType::Red => 2.5,  // Slow recovery
        }
    }

    pub fn sprite_size(&self) -> f32 {
        match self {
            OrganismType::Blue => 10.0,
            OrganismType::Red => 6.0,
        }
    }
}
