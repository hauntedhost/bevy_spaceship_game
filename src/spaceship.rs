use crate::asset_loader::SceneAssets;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use bevy::prelude::*;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const MOVEMENT_SPEED: f32 = 25.0;
const ROTATION_SPEED: f32 = 2.5;
const ROLL_SPEED: f32 = 3.0;
const SLOW_ROLL_SPEED: f32 = 0.5;

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
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    // barrel roll
    if keyboard_input.pressed(KeyCode::J) {
        roll = -ROLL_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::K) {
        roll = ROLL_SPEED * time.delta_seconds();
    }

    // y-axis rotate plus slight roll unless already rolling
    let is_rolling = keyboard_input.pressed(KeyCode::J) || keyboard_input.pressed(KeyCode::K);
    if keyboard_input.pressed(KeyCode::D) {
        if !is_rolling {
            roll = SLOW_ROLL_SPEED * time.delta_seconds();
        }
        rotation = -ROTATION_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::A) {
        if !is_rolling {
            roll = -SLOW_ROLL_SPEED * time.delta_seconds();
        }
        rotation = ROTATION_SPEED * time.delta_seconds();
    }

    // forward/backward movement
    if keyboard_input.pressed(KeyCode::S) {
        movement = -MOVEMENT_SPEED;
    } else if keyboard_input.pressed(KeyCode::W) {
        movement = MOVEMENT_SPEED;
    }

    transform.rotate_y(rotation);
    transform.rotate_local_z(roll);
    velocity.value = -transform.forward() * movement;
}
