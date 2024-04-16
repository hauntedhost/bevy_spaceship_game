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
                .init_resource::<SpaceshipPosition>()
                .add_perf_ui_entry_type::<PerfUiSpaceshipPosition>()
                .add_systems(Startup, add_perf)
                .add_systems(
                    Update,
                    update_spaceship_position.after(InGameSet::EntityUpdates),
                );
        }
    }
}

// Custom PerfUiEntry for spaceship (x, z) position
#[derive(Resource, Default)]
pub struct SpaceshipPosition {
    translation: Vec3,
}

#[derive(Component, Default)]
pub struct PerfUiSpaceshipPosition;

impl PerfUiEntry for PerfUiSpaceshipPosition {
    type Value = Vec3;
    type SystemParam = SRes<SpaceshipPosition>;

    fn label(&self) -> &str {
        "Ship position"
    }

    fn sort_key(&self) -> i32 {
        4
    }

    fn update_value(
        &self,
        position: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(position.translation)
    }

    fn format_value(&self, value: &Self::Value) -> String {
        let Vec3 { x, y: _, z } = value;
        let coords = format!("{:.2} x {:.2}", x, z);
        coords
    }

    fn width_hint(&self) -> usize {
        16
    }
}

fn update_spaceship_position(
    query: Query<&Transform, With<Spaceship>>,
    mut position: ResMut<SpaceshipPosition>,
) {
    let transform = query.single();
    position.translation = transform.translation;
}

fn add_perf(mut commands: Commands) {
    commands.spawn((
        PerfUiRoot { ..default() },
        PerfUiEntryFPS::default(),
        PerfUiEntryMemUsage::default(),
        PerfUiEntryEntityCount::default(),
        PerfUiSpaceshipPosition::default(),
    ));
}
