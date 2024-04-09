use bevy::prelude::*;

const DESPAWN_DISTANCE: f32 = 100.;

#[derive(Component, Debug)]
pub struct DespawnWhenRemote;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_remote_entities);
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
