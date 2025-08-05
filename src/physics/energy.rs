use crate::components::Energy;
use crate::organisms::Organism;
use crate::prelude::*;

pub fn energy_system(time: Res<Time>, mut query: Query<&mut Energy, With<Organism>>) {
    for mut energy in query.iter_mut() {
        // Pure energy regeneration logic - no visual updates
        energy.regenerate(time.delta_seconds());
    }
}
