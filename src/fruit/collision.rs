use bevy::prelude::*;

use crate::fruit::world::{LEFT, RIGHT, BOTTOM, TOP};
use crate::fruit::pva::{Acceleration, Position, Velocity};
use crate::fruit::reset::ResetEvent;
use crate::fruit::typ::FruitType;

const SPRING: f32 = 1e2;
const DAMPER: f32 = 1e1;
const BOUNCE: f32 = 1.3;

#[derive(Component, Default)]
pub struct Collider;

#[derive(Event, Deref, DerefMut)]
pub struct CollisionEvent([(Entity, FruitType, Position, Velocity, Acceleration); 2]);

pub fn check_wall_collisions(
    collider_query: Query<(&FruitType, &mut Position, &mut Velocity, &mut Acceleration), With<Collider>>,
    mut reset_writer: EventWriter<ResetEvent>,
) {
    for (fruit, mut pos, mut vel, mut acc) in collider_query {
        let radius = fruit.to_circle().radius;

        let right_squish = radius - (RIGHT - pos.x);
        let x_force = if right_squish > 0. {
            pos.x = RIGHT - radius;
            -right_squish * SPRING - vel.x * DAMPER
        } else {
            let left_squish = radius - (pos.x - LEFT);
            if left_squish > 0. {
                pos.x = LEFT + radius;
                left_squish * SPRING - vel.x * DAMPER
            } else {
                0.0
            }
        };
        if x_force != 0. {
            acc.x += x_force / fruit.mass();
        }

        let bottom_squish = radius - (pos.y - BOTTOM);
        if bottom_squish > 0. {
            pos.y = BOTTOM + radius;
            vel.y = 0.;
            let y_force = bottom_squish * SPRING - vel.y * DAMPER;
            acc.y += y_force / fruit.mass();
            acc.x -= vel.x * DAMPER * 0.1;
        }
        let top_squish = radius - (TOP - pos.y);
        if top_squish > 0. {
            reset_writer.write(ResetEvent);
        }
    }
}

pub fn check_fruit_collisions(
    mut collisions: EventWriter<CollisionEvent>,
    mut collider_query: Query<(Entity, &FruitType, &mut Position, &mut Velocity, &mut Acceleration), With<Collider>>,
) {
    let mut combinations = collider_query.iter_combinations_mut();
    while let Some([
        (entity0, fruit0, mut pos0, mut vel0, mut acc0),
        (entity1, fruit1, mut pos1, mut vel1, mut acc1),
    ]) = combinations.fetch_next() {
        let radius0 = fruit0.to_circle().radius;
        let radius1 = fruit1.to_circle().radius;

        let seg = Segment2d::new(pos0.0, pos1.0);
        let overlap = radius0 + radius1 - seg.length();
        if overlap > 0. && overlap < radius0 + radius1 {
            // collision!
            collisions.write(CollisionEvent([
                (entity0, fruit0.clone(), pos0.clone(), vel0.clone(), acc0.clone()),
                (entity1, fruit1.clone(), pos1.clone(), vel1.clone(), acc1.clone())
            ]));

            fn reaction(toward_other: Vec2, overlap: f32, fruit: &FruitType, pos: &mut Position, vel: &mut Velocity, acc: &mut Acceleration) {
                let squish = toward_other * (overlap / 2.);
                pos.0 -= squish;

                let vel_toward_other = vel.dot(toward_other).max(0.);
                vel.0 -= toward_other * vel_toward_other * BOUNCE;

                let spring_force = squish / fruit.to_circle().radius * SPRING;
                let damp_force = toward_other * vel_toward_other * DAMPER;
                acc.0 -= (spring_force + damp_force) / fruit.mass();
            }

            let toward_other = seg.direction().as_vec2();
            reaction(toward_other, overlap, &fruit0, &mut pos0, &mut vel0, &mut acc0);

            let toward_other = seg.direction().as_vec2() * -1.;
            reaction(toward_other, overlap, &fruit1, &mut pos1, &mut vel1, &mut acc1);
        }
    }

}

