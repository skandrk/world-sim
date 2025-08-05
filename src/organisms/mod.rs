mod organismtype;
mod organismtypemarker;
mod spawn;

use crate::prelude::*;
pub use organismtype::OrganismType;
pub use organismtypemarker::OrganismTypeMarker;
pub use spawn::spawn_organisms;

#[derive(Component)]
pub struct Organism;
