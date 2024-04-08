use bevy::prelude::*;
use iyes_perf_ui::diagnostics::{PerfUiEntryEntityCount, PerfUiEntryFPS};
use iyes_perf_ui::{PerfUiPlugin, PerfUiRoot};

pub struct DebugPlugin {
    pub position_info: bool,
    pub perf_ui: bool,
}

impl Default for DebugPlugin {
    fn default() -> Self {
        Self {
            position_info: false,
            perf_ui: false,
        }
    }
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if self.position_info {
            app.add_systems(Update, print_position);
        }

        if self.perf_ui {
            app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
                .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
                .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
                .add_plugins(PerfUiPlugin)
                .add_systems(Startup, add_perf);
        }
    }
}

fn add_perf(mut commands: Commands) {
    info!("hi!");
    commands.spawn((
        PerfUiRoot { ..default() },
        PerfUiEntryFPS::default(),
        PerfUiEntryEntityCount::default(),
    ));
}

fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        info!("Entity {:?} is at position {:?}", entity, transform);
    }
}
