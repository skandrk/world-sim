use crate::components::{Energy, Movement, Position, RedAI};
use crate::organisms::{Organism, OrganismType, OrganismTypeMarker};
use crate::prelude::*;

pub fn blue_movement_system(
    time: Res<Time>,
    mut query: Query<
        (
            &mut Position,
            &mut Movement,
            &mut Transform,
            &mut Energy,
            &OrganismTypeMarker,
        ),
        (With<Organism>, Without<RedAI>),
    >,
) {
    let mut rng = thread_rng();

    for (mut pos, mut movement, mut transform, mut energy, organism_marker) in query.iter_mut() {
        if !organism_marker.is_type(OrganismType::Blue) {
            continue;
        }

        movement.tick(time.delta());

        if movement.just_finished() {
            if energy.can_move() {
                // Random walk behavior
                let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                let distance = rng.gen_range(0.0..1.0) * movement.max_jump_distance();

                let new_x = pos.x() + angle.cos() * distance;
                let new_y = pos.y() + angle.sin() * distance;

                pos.set_position(new_x, new_y);
                pos.clamp_to_world(WORLD_SIZE);

                // Update transform immediately when position changes (5 FPS)
                transform.translation.x = pos.x();
                transform.translation.y = pos.y();

                energy.consume_movement();
            }
        }
    }
}

pub fn red_movement_system(
    time: Res<Time>,
    mut predator_query: Query<
        (
            &mut Position,
            &mut Movement,
            &mut Transform,
            &mut Energy,
            &RedAI,
            &OrganismTypeMarker,
        ),
        With<Organism>,
    >,
    prey_query: Query<(&Position, &OrganismTypeMarker), (With<Organism>, Without<RedAI>)>,
) {
    for (
        mut predator_pos,
        mut predator_movement,
        mut predator_transform,
        mut predator_energy,
        red_ai,
        predator_marker,
    ) in predator_query.iter_mut()
    {
        if !predator_marker.is_type(OrganismType::Red) {
            continue;
        }

        predator_movement.tick(time.delta());

        if predator_movement.just_finished() {
            if predator_energy.can_move() {
                // Find blue prey
                let mut target_position: Option<&Position> = None;

                for (prey_pos, prey_marker) in prey_query.iter() {
                    if prey_marker.is_type(red_ai.target_type()) {
                        let distance = predator_pos.distance_to(prey_pos);

                        if red_ai.can_see(distance) {
                            target_position = Some(prey_pos);
                            break;
                        }
                    }
                }

                let (new_x, new_y) = if let Some(target_pos) = target_position {
                    // Chase behavior
                    let dx = target_pos.x() - predator_pos.x();
                    let dy = target_pos.y() - predator_pos.y();
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance > 0.0 {
                        let move_distance = predator_movement.max_jump_distance() * 0.8;
                        let unit_x = dx / distance;
                        let unit_y = dy / distance;

                        let new_x = predator_pos.x() + unit_x * move_distance;
                        let new_y = predator_pos.y() + unit_y * move_distance;

                        println!("🔴 Red chasing prey!");
                        (new_x, new_y)
                    } else {
                        (predator_pos.x(), predator_pos.y())
                    }
                } else {
                    // Random walk when no prey in sight
                    let mut rng = thread_rng();
                    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                    let distance = rng.gen_range(0.0..1.0) * predator_movement.max_jump_distance();

                    let new_x = predator_pos.x() + angle.cos() * distance;
                    let new_y = predator_pos.y() + angle.sin() * distance;
                    (new_x, new_y)
                };

                predator_pos.set_position(new_x, new_y);
                predator_pos.clamp_to_world(WORLD_SIZE);

                // Update transform immediately when position changes (5 FPS)
                predator_transform.translation.x = predator_pos.x();
                predator_transform.translation.y = predator_pos.y();

                predator_energy.consume_movement();
            }
        }
    }
}
