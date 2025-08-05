use crate::components::{Collision, Position};
use crate::organisms::Organism;
use crate::prelude::*;

pub fn collision_system(
    mut query: Query<(Entity, &mut Position, &mut Transform, &Collision), With<Organism>>,
) {
    let mut combinations = query.iter_combinations_mut();

    while let Some(
        [
            (_entity_a, mut pos_a, mut transform_a, collision_a),
            (_entity_b, mut pos_b, mut transform_b, collision_b),
        ],
    ) = combinations.fetch_next()
    {
        let distance = pos_a.distance_to(&pos_b);
        let min_distance = collision_a.min_distance_to(collision_b);

        if collision_a.is_colliding_with(collision_b, distance) && distance > 0.0 {
            let overlap = min_distance - distance;
            let dx = pos_b.x() - pos_a.x();
            let dy = pos_b.y() - pos_a.y();
            let unit_x = dx / distance;
            let unit_y = dy / distance;
            let separation = overlap / 2.0;

            // Calculate new positions before applying them
            let new_a_x = pos_a.x() - unit_x * separation;
            let new_a_y = pos_a.y() - unit_y * separation;
            let new_b_x = pos_b.x() + unit_x * separation;
            let new_b_y = pos_b.y() + unit_y * separation;

            // Apply the new positions
            pos_a.set_position(new_a_x, new_a_y);
            pos_b.set_position(new_b_x, new_b_y);

            pos_a.clamp_to_world(WORLD_SIZE);
            pos_b.clamp_to_world(WORLD_SIZE);
            transform_a.translation.x = pos_a.x();
            transform_a.translation.y = pos_a.y();
            transform_b.translation.x = pos_b.x();
            transform_b.translation.y = pos_b.y();
        }
    }
}
