mod asset_loader;
mod asteroid;
mod camera;
mod collision;
mod debug;
mod despawn;
mod health;
mod movement;
mod schedule;
mod spaceship;
mod state;

use asset_loader::AssetLoaderPlugin;
use asteroid::AsteroidPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use collision::CollisionPlugin;
use debug::DebugPlugin;
use despawn::DespawnPlugin;
use movement::MovementPlugin;
use schedule::SchedulePlugin;
use spaceship::SpaceshipPlugin;
use state::StatePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1_000.,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(DebugPlugin { enabled: true })
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(StatePlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(SchedulePlugin)
        .run();
}
