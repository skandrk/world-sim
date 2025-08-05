use super::*;
use crate::components::*;
use crate::prelude::*;

pub fn spawn_organisms(mut commands: Commands) {
    let mut rng = thread_rng();

    // Spawn Blue prey
    let blue_x = rng.gen_range(100.0..WORLD_SIZE - 100.0);
    let blue_y = rng.gen_range(100.0..WORLD_SIZE - 100.0);
    let blue_type = OrganismType::Blue;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: blue_type.color(),
                custom_size: Some(Vec2::new(blue_type.sprite_size(), blue_type.sprite_size())),
                ..default()
            },
            transform: Transform::from_xyz(blue_x, blue_y, 1.0),
            ..default()
        },
        Position::new(blue_x, blue_y),
        Movement::new(blue_type.max_jump_distance(), MOVEMENT_INTERVAL),
        Collision::new(blue_type.collision_radius()),
        Health::new(blue_type.max_health()),
        Energy::new(
            blue_type.max_energy(),
            blue_type.movement_cost(),
            blue_type.energy_regen(),
        ),
        OrganismTypeMarker::new(blue_type),
        Organism,
    ));

    // Spawn Red predator
    let red_x = rng.gen_range(100.0..WORLD_SIZE - 100.0);
    let red_y = rng.gen_range(100.0..WORLD_SIZE - 100.0);
    let red_type = OrganismType::Red;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: red_type.color(),
                custom_size: Some(Vec2::new(red_type.sprite_size(), red_type.sprite_size())),
                ..default()
            },
            transform: Transform::from_xyz(red_x, red_y, 1.0),
            ..default()
        },
        Position::new(red_x, red_y),
        Movement::new(red_type.max_jump_distance(), MOVEMENT_INTERVAL),
        Collision::new(red_type.collision_radius()),
        Health::new(red_type.max_health()),
        Energy::new(
            red_type.max_energy(),
            red_type.movement_cost(),
            red_type.energy_regen(),
        ),
        OrganismTypeMarker::new(red_type),
        RedAI::new(450.0, OrganismType::Blue),
        Organism,
    ));

    println!("🎯 Predator-Prey Chase Simulation Started!");
    println!("🔵 Blue prey spawned at ({:.1}, {:.1})", blue_x, blue_y);
    println!("🔴 Red predator spawned at ({:.1}, {:.1})", red_x, red_y);
}
