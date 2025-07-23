use bevy::prelude::*;

use crate::fruit::collision::Collider;
use crate::fruit::typ::FruitType;

#[derive(Event)]
pub struct ResetEvent;

pub fn reset(
    mut commands: Commands,
    query: Query<Entity, (With<FruitType>, With<Collider>)>,
    _reader: EventReader<ResetEvent>,
) {
    warn!("reset");
    for entity in query {
        commands.entity(entity).despawn();
    }
}
