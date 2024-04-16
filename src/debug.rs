use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use iyes_perf_ui::diagnostics::{PerfUiEntryEntityCount, PerfUiEntryFPS, PerfUiEntryMemUsage};
use iyes_perf_ui::{PerfUiAppExt, PerfUiEntry, PerfUiPlugin, PerfUiRoot};

use crate::schedule::InGameSet;
use crate::spaceship::Spaceship;

pub struct DebugPlugin {
    pub enabled: bool,
}

impl Default for DebugPlugin {
    fn default() -> Self {
        Self { enabled: false }
    }
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if self.enabled {
            app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
                .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
                .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
                .add_plugins(PerfUiPlugin)
                .init_resource::<SpaceshipStatus>()
                .add_perf_ui_entry_type::<PerfUiSpaceshipStatus>()
                .add_systems(Startup, add_perf)
                .add_systems(
                    Update,
                    update_spaceship_position.after(InGameSet::EntityUpdates),
                );
        }
    }
}

fn add_perf(mut commands: Commands) {
    commands.spawn((
        PerfUiRoot { ..default() },
        PerfUiEntryFPS::default(),
        PerfUiEntryMemUsage::default(),
        PerfUiEntryEntityCount::default(),
        PerfUiSpaceshipStatus::default(),
    ));
}

// Custom PerfUiEntry for spaceship status
#[derive(Resource, Debug, Default)]
struct SpaceshipStatus {
    translation: Vec3,
    dead: bool,
}

#[derive(Component, Default)]
struct PerfUiSpaceshipStatus;

impl PerfUiEntry for PerfUiSpaceshipStatus {
    type Value = SpaceshipStatus;
    type SystemParam = SRes<SpaceshipStatus>;

    fn label(&self) -> &str {
        "Ship position"
    }

    fn sort_key(&self) -> i32 {
        4
    }

    fn update_value(
        &self,
        status: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(SpaceshipStatus {
            translation: status.translation,
            dead: status.dead,
        })
    }

    fn format_value(&self, status: &Self::Value) -> String {
        let Vec3 { x, y: _, z } = status.translation;
        let coords = format!("{:.2} x {:.2}", x, z);
        coords
    }

    fn value_color(&self, status: &Self::Value) -> Option<Color> {
        if status.dead {
            return Some(Color::ORANGE_RED);
        } else {
            return None;
        }
    }

    fn width_hint(&self) -> usize {
        16
    }
}

fn update_spaceship_position(
    query: Query<&Transform, With<Spaceship>>,
    mut status: ResMut<SpaceshipStatus>,
) {
    if let Ok(transform) = query.get_single() {
        status.translation = transform.translation;
    } else {
        status.dead = true;
    }
}
