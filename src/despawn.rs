use bevy::prelude::*;

use crate::{health::Health, schedule::InGameSet};

const DESPAWN_DISTANCE: f32 = 100.;

#[derive(Component, Debug)]
pub struct DespawnWhenRemote;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (despawn_remote_entities, despawn_dead_entities).in_set(InGameSet::DespawnEntities),
        );
    }
}

fn despawn_remote_entities(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), With<DespawnWhenRemote>>,
) {
    for (entity, transform) in query.iter() {
        let distance_from_origin = transform.translation().distance(Vec3::ZERO);
        if distance_from_origin > DESPAWN_DISTANCE {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_dead_entities(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in query.iter() {
        if health.value <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
