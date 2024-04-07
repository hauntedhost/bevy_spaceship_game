use crate::asset_loader::SceneAssets;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use bevy::prelude::*;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const MOVEMENT_SPEED: f32 = 25.0;
const ROTATION_SPEED: f32 = 2.5;
const ROLL_SPEED: f32 = 3.0;
const MISSILE_SPEED: f32 = 55.0;
const MISSILE_FORWARD_SPAWN: f32 = 6.5;

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship).add_systems(
            Update,
            (spaceship_movement_controls, spaceship_weapon_controls),
        );
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
    if keyboard_input.pressed(KeyCode::K) {
        roll = -ROLL_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::L) {
        roll = ROLL_SPEED * time.delta_seconds();
    }

    // y-axis rotate
    if keyboard_input.pressed(KeyCode::D) {
        rotation = -ROTATION_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::A) {
        rotation = ROTATION_SPEED * time.delta_seconds();
    }

    // forward/reverse movement
    if keyboard_input.pressed(KeyCode::S) {
        movement = -MOVEMENT_SPEED;
    } else if keyboard_input.pressed(KeyCode::W) {
        movement = MOVEMENT_SPEED;
    }

    transform.rotate_y(rotation);
    transform.rotate_local_z(roll);
    velocity.value = -transform.forward() * movement;
}

fn spaceship_weapon_controls(
    mut commands: Commands,
    query: Query<&Transform, With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    let transform = query.single();

    if keyboard_input.pressed(KeyCode::Space) {
        let missile_transform = Transform::from_translation(
            transform.translation + -transform.forward() * MISSILE_FORWARD_SPAWN,
        )
        .with_rotation(transform.rotation)
        .with_scale(Vec3::splat(0.25));

        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(-transform.forward() * MISSILE_SPEED),
                acceleration: Acceleration::new(Vec3::ZERO),
                model: SceneBundle {
                    scene: scene_assets.missile.clone(),
                    transform: missile_transform,
                    ..default()
                },
            },
            SpaceshipMissile,
        ));
    }
}
