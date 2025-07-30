use bevy::prelude::*;

use crate::fruit::typ::FruitType;

const GRAVITY: f32 = -100.;

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Position(pub Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousPosition(pub Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Acceleration(pub Vec2);

pub fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Position, &mut PreviousPosition, &Velocity, &Acceleration)>
) {
    let dt = time.delta_secs();
    let dt2 = 0.5 * dt * dt;
    for (mut pos, mut pre, vel, acc) in &mut query {
        pre.0 = pos.0;
        pos.0 += vel.0 * dt + acc.0 * dt2;
    }
}

pub fn apply_acceleration(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut Acceleration)>,
) {
    let dt = time.delta_secs();
    for (mut vel, mut acc) in &mut query {
        vel.0 += acc.0 * dt;
        acc.0 *= 0.;
    }
}

pub fn apply_gravity(
    mut query: Query<&mut Acceleration, With<FruitType>>,
) {
    for mut acc in &mut query {
        acc.0 += Vec2::new(0., GRAVITY);
    }
}
