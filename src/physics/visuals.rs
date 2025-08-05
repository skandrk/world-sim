use crate::components::{Energy, Health};
use crate::organisms::{Organism, OrganismTypeMarker};
use crate::prelude::*;

pub fn visual_system(
    mut query: Query<(&Energy, &Health, &OrganismTypeMarker, &mut Sprite), With<Organism>>,
) {
    for (energy, health, organism_marker, mut sprite) in query.iter_mut() {
        // Update sprite with combined health and energy effects (60 FPS)
        update_combined_sprite_effects(energy, health, organism_marker, &mut sprite);
    }
}

fn update_combined_sprite_effects(
    energy: &Energy,
    health: &Health,
    organism_marker: &OrganismTypeMarker,
    sprite: &mut Sprite,
) {
    // Get base color for organism type
    let base_color = organism_marker.organism_type().color();

    // Calculate health color effect
    let health_ratio = health.ratio();
    let (r, g, b) = if health.is_low(0.5) {
        // Apply red tint when health is low
        let red_intensity = 1.0 - health_ratio;
        (
            1.0,
            base_color.g() * (1.0 - red_intensity * 0.5),
            base_color.b() * (1.0 - red_intensity * 0.5),
        )
    } else {
        // Normal base color
        (base_color.r(), base_color.g(), base_color.b())
    };

    // Calculate energy alpha effect
    let energy_ratio = energy.ratio();
    let alpha = if energy.is_low(0.3) {
        // Dim when energy is low
        0.5 + energy_ratio * 0.5
    } else {
        // Full opacity when energy is sufficient
        1.0
    };

    // Apply combined effects
    sprite.color = Color::rgba(r, g, b, alpha);
}
