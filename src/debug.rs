use bevy::prelude::*;

pub struct DebugPlugin {
    pub enabled: bool,
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if self.enabled {
            app.add_systems(Update, print_position);
        }
    }
}

fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        info!("Entity {:?} is at position {:?}", entity, transform);
    }
}
