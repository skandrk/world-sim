use crate::prelude::*;

#[derive(Component)]
struct MainCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                scale: 1.2, // Increased from 0.6 to see more area
                ..default()
            },
            transform: Transform::from_xyz(WORLD_SIZE / 2.0, WORLD_SIZE / 2.0, 50.0),
            ..default()
        },
        MainCamera,
    ));

    println!("📷 Camera system initialized");
}
