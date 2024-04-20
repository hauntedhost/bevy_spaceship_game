use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use iyes_perf_ui::diagnostics::{PerfUiEntryEntityCount, PerfUiEntryFPS, PerfUiEntryMemUsage};
use iyes_perf_ui::{PerfUiAppExt, PerfUiEntry, PerfUiPlugin, PerfUiRoot};

use crate::health::Health;
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
                .add_perf_ui_entry_type::<PerfUiSpaceshipPosition>()
                .add_perf_ui_entry_type::<PerfUiSpaceshipHealth>()
                .add_systems(Startup, add_perf)
                .add_systems(
                    Update,
                    update_spaceship_status.after(InGameSet::EntityUpdates),
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
        PerfUiSpaceshipPosition::default(),
        PerfUiSpaceshipHealth::default(),
    ));
}

// Custom PerfUiEntry for Spaceship status
#[derive(Resource, Debug, Default)]
struct SpaceshipStatus {
    translation: Vec3,
    health: f32,
}

#[derive(Component, Default)]
struct PerfUiSpaceshipPosition;

impl PerfUiEntry for PerfUiSpaceshipPosition {
    type Value = (f32, f32);
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
        let Vec3 { x, y: _, z: y } = status.translation;
        Some((x, y))
    }

    fn format_value(&self, (x, y): &Self::Value) -> String {
        let coords = format!("{:.2} x {:.2}", x, y);
        coords
    }

    fn width_hint(&self) -> usize {
        16
    }
}

#[derive(Component, Default)]
struct PerfUiSpaceshipHealth;

impl PerfUiEntry for PerfUiSpaceshipHealth {
    type Value = f32;
    type SystemParam = SRes<SpaceshipStatus>;

    fn label(&self) -> &str {
        "Ship health"
    }

    fn sort_key(&self) -> i32 {
        5
    }

    fn update_value(
        &self,
        status: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(status.health)
    }

    fn format_value(&self, value: &Self::Value) -> String {
        value.to_string()
    }

    fn value_color(&self, value: &Self::Value) -> Option<Color> {
        if *value <= 0.0 {
            return Some(Color::ORANGE_RED);
        } else {
            return None;
        }
    }

    fn width_hint(&self) -> usize {
        16
    }
}

fn update_spaceship_status(
    query: Query<(&Transform, &Health), With<Spaceship>>,
    mut status: ResMut<SpaceshipStatus>,
) {
    if let Ok((transform, health)) = query.get_single() {
        status.translation = transform.translation;
        status.health = health.value;
    }
}
