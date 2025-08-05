use crate::components::{Energy, Position};
use crate::organisms::{Organism, OrganismType, OrganismTypeMarker};
use crate::prelude::*;

#[derive(Resource)]
pub struct MonitoringTimer(Timer);

impl MonitoringTimer {
    pub fn new(interval: f32) -> Self {
        Self(Timer::from_seconds(interval, TimerMode::Repeating))
    }

    pub fn tick(&mut self, delta: std::time::Duration) {
        self.0.tick(delta);
    }

    pub fn just_finished(&self) -> bool {
        self.0.just_finished()
    }
}

pub fn monitoring_system(
    time: Res<Time>,
    mut timer: ResMut<MonitoringTimer>,
    query: Query<(&OrganismTypeMarker, &Position, &Energy), With<Organism>>,
) {
    timer.tick(time.delta());

    if timer.just_finished() {
        let mut blue_count = 0;
        let mut red_count = 0;
        let mut blue_pos: Option<(f32, f32)> = None;
        let mut red_pos: Option<(f32, f32)> = None;
        let mut blue_energy = 0.0;
        let mut red_energy = 0.0;

        for (organism_marker, position, energy) in query.iter() {
            match organism_marker.organism_type() {
                OrganismType::Blue => {
                    blue_count += 1;
                    blue_pos = Some((position.x(), position.y()));
                    blue_energy = energy.current();
                }
                OrganismType::Red => {
                    red_count += 1;
                    red_pos = Some((position.x(), position.y()));
                    red_energy = energy.current();
                }
            }
        }

        let total = blue_count + red_count;
        println!(
            "📊 Chase Status: 🔵 Prey: {} | 🔴 Predator: {} | Total: {}",
            blue_count, red_count, total
        );

        if let Some((bx, by)) = blue_pos {
            println!(
                "🔵 Blue Position: ({:.1}, {:.1}) Energy: {:.1}",
                bx, by, blue_energy
            );
        }

        if let Some((rx, ry)) = red_pos {
            println!(
                "🔴 Red Position: ({:.1}, {:.1}) Energy: {:.1}",
                rx, ry, red_energy
            );
        }

        if let (Some((bx, by)), Some((rx, ry))) = (blue_pos, red_pos) {
            let distance = ((bx - rx).powi(2) + (by - ry).powi(2)).sqrt();
            println!("📏 Distance between organisms: {:.1} units", distance);
        }

        if blue_count == 0 {
            println!("🏆 PREDATOR WINS! Blue prey was caught!");
        } else if red_count == 0 {
            println!("🏆 PREY WINS! Red predator died of exhaustion!");
        } else {
            println!("🏃‍♂️ Chase continues...");
        }
    }
}
