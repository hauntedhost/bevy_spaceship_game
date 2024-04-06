use bevy::prelude::*;
use crate::movement::Velocity;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship);
    }
}

fn spawn_spaceship(mut commands: Commands) {
    commands.spawn((SpatialBundle::default(), Velocity { value: Vec3::new(0.0, 0.0, 0.0) }));
}
