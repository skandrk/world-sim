pub mod prelude {
    pub use bevy::prelude::*;
    pub use rand::prelude::*;
    pub const WORLD_SIZE: f32 = 1024.0;
    pub const MOVEMENT_INTERVAL: f32 = 0.2;
    pub const MONITORING_INTERVAL: f32 = 1.0;
}

//pub(crate) mod monitoring;
pub mod components;
pub mod organisms;
pub mod physics;
pub mod utils;
