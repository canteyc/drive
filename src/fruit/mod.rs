pub(crate) mod collision;
pub(crate) mod input;
pub(crate) mod pva;
pub(crate) mod reset;
pub(crate) mod toa;
pub(crate) mod typ;
pub(crate) mod world;

use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

use collision::{Collider, CollisionEvent, check_fruit_collisions, check_wall_collisions};
use input::{DropEvent, KeyHoldEvent, Player, record_key_press, load_player, load_input_display, player_input, fast_drop};
use pva::{Acceleration, Position, PreviousPosition, Velocity, apply_acceleration, apply_gravity, apply_velocity};
use reset::{reset, ResetEvent};
use toa::{Omega, Theta};
use typ::FruitType;
use world::{load_container, TOP};

const INPUT_RATE_HZ: u64 = 1;
const REPEAT_RATE_HZ: u64 = 2;

pub struct FruitGame;

impl Plugin for FruitGame {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, (load_container, load_player, load_input_display))
        .add_systems(Update, (
            player_input.run_if(on_timer(Duration::from_millis(1000 / INPUT_RATE_HZ))),
            fast_drop.run_if(on_timer(Duration::from_millis(1000 / REPEAT_RATE_HZ))),
        ))
        .add_systems(FixedUpdate, (
            record_key_press,
            drop_fruit.run_if(on_event::<DropEvent>),
            apply_velocity,
            apply_acceleration,
            apply_gravity,
            check_wall_collisions,
            check_fruit_collisions,
            merge,
        ).chain())
        .add_systems(RunFixedMainLoop, (
            interpolate_rendered_transform.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
            // indicate_spin.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
            reset.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop).run_if(on_event::<ResetEvent>),
        ))
        .add_event::<CollisionEvent>()
        .add_event::<ResetEvent>()
        .add_event::<DropEvent>()
        .init_resource::<Events<KeyHoldEvent>>()
        ;
    }
}

#[derive(Bundle, Clone, Default)]
pub struct Fruit {
    pub typ: FruitType,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub pos: Position,
    pub pre: PreviousPosition,
    pub vel: Velocity,
    pub acc: Acceleration,
    pub theta: Theta,
    pub omega: Omega,
}

impl Fruit {
    pub fn new(
        typ: FruitType,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        Self {
            typ,
            mesh: Mesh2d(meshes.add(typ.to_circle())),
            material: MeshMaterial2d(materials.add(typ.color().with_alpha(0.5))),
            ..Default::default()
        }
    }

    pub fn rand_to(
        upper: FruitType,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        Self::new(FruitType::rand_up_to(upper), meshes, materials)
    }
}

fn drop_fruit(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    query: Single<(
        &mut FruitType,
        &mut Mesh2d,
        &mut MeshMaterial2d<ColorMaterial>,
        &mut Transform,
        ), With<Player>>,
    mut drop_event: ResMut<Events<DropEvent>>,
) {
    drop_event.clear();
    let (mut typ, mut mesh, mut material, mut transform) = query.into_inner();
    let radius = typ.radius();
    let mut spawn_location = *transform;
    spawn_location.translation.y -= radius * 2.;
    let fruit = Fruit {
        typ: *typ,
        mesh: mesh.clone(),
        material: material.clone(),
        pos: Position(spawn_location.translation.truncate()),
        pre: PreviousPosition(spawn_location.translation.truncate()),
        vel: Velocity(Vec2::new(0., -100.)),
        acc: Default::default(),
        theta: Default::default(),
        omega: Default::default(),
    };
    commands.spawn((
        fruit,
        spawn_location,
        Collider,
    ));

    let new_fruit = Fruit::rand_to(FruitType::Apricot, meshes, materials);
    *typ = new_fruit.typ;
    *mesh = new_fruit.mesh;
    *material = new_fruit.material;
    transform.translation.y = TOP + typ.radius();
}

fn interpolate_rendered_transform(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut Transform,
        &Position,
        &PreviousPosition,
    ), (With<FruitType>, Without<Player>)>,
) {
    for (mut transform, &pos, &pre) in query.iter_mut() {
        // The overstep fraction is a value between 0 and 1 that tells us how far we are between two fixed timesteps.
        let alpha = fixed_time.overstep_fraction();

        let rendered_translation = pre.lerp(*pos, alpha);
        transform.translation = rendered_translation.extend(1.0);
    }
}

// fn indicate_spin(
//     mut query: Query<(
//         &Omega,
//         &mut MeshMaterial2d<ColorMaterial>,
//     )>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     for (&omega, mut material) in query.into_iter() {
//         let alpha = 0.5 + *omega * 0.05;
//         // if *omega != 0. {
//         //     warn!("{omega:?}");
//         // }
//         if let Some(mut color_material) = materials.get_mut(&material.0) {
//             color_material.color.set_alpha(alpha);
//         }
//     }
// }

fn merge(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for collision in collisions.read() {
        let (entity0, fruit0, pos0, vel0, _) = collision[0];
        let (entity1, fruit1, pos1, vel1, _) = collision[1];
        if fruit0 != fruit1 {
            continue;
        }
        if let Some(new_type) = fruit0.next() {
            let midpoint = (*pos0 + *pos1) / 2.;
            let vel = (*vel0 * fruit0.mass() + *vel1 * fruit1.mass()) / new_type.mass();

            if let Ok(mut e) = commands.get_entity(entity0) {
                e.despawn();
            }
            if let Ok(mut e) = commands.get_entity(entity1) {
                e.despawn();
            }

            let mut merged_fruit = Fruit::new(new_type, meshes, materials);
            *merged_fruit.pos = midpoint;
            *merged_fruit.pre = midpoint;
            *merged_fruit.vel = vel;
            commands.spawn((
                merged_fruit,
                Transform::from_xyz(midpoint.x, midpoint.y, 0.),
                Collider,
            ));
            break;
        }

    }
}

