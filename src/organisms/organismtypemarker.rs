use super::OrganismType;
use crate::prelude::*;

#[derive(Component)]
pub struct OrganismTypeMarker {
    organism_type: OrganismType,
}

impl OrganismTypeMarker {
    pub fn new(organism_type: OrganismType) -> Self {
        Self { organism_type }
    }

    pub fn organism_type(&self) -> OrganismType {
        self.organism_type
    }

    pub fn set_organism_type(&mut self, organism_type: OrganismType) {
        self.organism_type = organism_type;
    }

    pub fn is_type(&self, organism_type: OrganismType) -> bool {
        self.organism_type == organism_type
    }
}
