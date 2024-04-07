use crate::asset_loader::SceneAssets;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use bevy::prelude::*;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const MOVEMENT_SPEED: f32 = 25.0;

#[derive(Component, Debug)]
pub struct Spaceship;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship)
            .add_systems(Update, spaceship_movement_controls);
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            model: SceneBundle {
                scene: scene_assets.spaceship.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        Spaceship,
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let (transform, mut velocity) = query.single_mut();
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::S) {
        movement = -MOVEMENT_SPEED;
    } else if keyboard_input.pressed(KeyCode::W) {
        movement = MOVEMENT_SPEED;
    }

    velocity.value = -transform.forward() * movement;
}
