use crate::components::{Energy, Health};
use crate::organisms::Organism;
use crate::prelude::*;

pub fn health_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Health, &Energy), With<Organism>>,
) {
    for (entity, mut health, energy) in query.iter_mut() {
        // Pure health logic - no visual updates
        if energy.is_depleted() {
            health.take_damage(10.0 * time.delta_seconds());
        }

        // Remove dead organisms
        if !health.is_alive() {
            commands.entity(entity).despawn();
            println!("💀 An organism died from starvation!");
        }
    }
}
