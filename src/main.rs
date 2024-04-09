mod asset_loader;
mod asteroid;
mod camera;
mod collission_detection;
mod debug;
mod despawn;
mod movement;
mod spaceship;

use asset_loader::AssetLoaderPlugin;
use asteroid::AsteroidPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use collission_detection::CollisionDetectionPlugin;
use debug::DebugPlugin;
use despawn::DespawnPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1_000.,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(DebugPlugin { enabled: true })
        .add_plugins(DespawnPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(SpaceshipPlugin)
        .run();
}
