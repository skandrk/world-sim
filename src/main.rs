use world_sim::prelude::*;

use bevy::window::WindowResolution;

use world_sim::physics::{
    blue_movement_system, collision_system, energy_system, health_system, red_movement_system,
    visual_system,
};
use world_sim::utils::{MonitoringTimer, monitoring_system, setup_camera};

use world_sim::organisms::spawn_organisms;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "ECS Organism Simulation".into(),
                resolution: WindowResolution::new(1024.0, 1024.0).with_scale_factor_override(2.0),
                resizable: true,
                decorations: false,
                present_mode: bevy::window::PresentMode::AutoVsync,
                mode: bevy::window::WindowMode::Windowed,
                resize_constraints: WindowResizeConstraints {
                    min_width: 400.0,
                    min_height: 400.0,
                    max_width: f32::INFINITY,
                    max_height: f32::INFINITY,
                },
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.15)))
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.15)))
        .insert_resource(MonitoringTimer::new(MONITORING_INTERVAL))
        .add_systems(Startup, (setup_camera, spawn_organisms))
        .add_systems(
            Update,
            (
                // Pure data systems
                energy_system,
                health_system,
                // Movement systems with transform updates (5 FPS)
                blue_movement_system,
                red_movement_system,
                collision_system,
                // Visual system - sprite updates only (60 FPS)
                visual_system,
                monitoring_system,
            ),
        )
        .run();
}
