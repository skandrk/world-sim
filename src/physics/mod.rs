mod collision;
mod energy;
mod health;
mod movement;
mod visuals;

pub use collision::collision_system;
pub use energy::energy_system;
pub use health::health_system;
pub use movement::blue_movement_system;
pub use movement::red_movement_system;
pub use visuals::visual_system;
