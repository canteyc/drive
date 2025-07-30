use bevy::prelude::*;

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Theta(pub f32);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Omega(pub f32);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Alpha(pub f32);


pub fn apply_omega(
    time: Res<Time>,
    mut query: Query<(&mut Theta, &Omega, &Alpha)>,
) {
    let dt = time.delta_secs();
    let dt2 = 0.5 * dt * dt;
    for (mut theta, omega, alpha) in query {
        **theta += **omega * dt + **alpha * dt2;
    }
}

pub fn apply_alpha(
    time: Res<Time>,
    mut query: Query<(&mut Omega, &mut Alpha)>,
) {
    let dt = time.delta_secs();
    for (mut omega, mut alpha) in &mut query {
        **omega += **alpha * dt;
        **alpha *= 0.;
    }
}
