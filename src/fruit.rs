use bevy::{
    color::palettes::basic::*,
    // math::bounding::{Aabb2d, BoundingCircle},
    prelude::*,
};


const RIGHT: f32 = 300.;
const LEFT: f32 = -300.;
const TOP: f32 = 300.;
const BOTTOM: f32 = -300.;
const THICKNESS: f32 = 1.;

const RADIUS_BLUEBERRY: f32 = 10.;
const RADIUS_CHERRY: f32 = 14.;
const RADIUS_APRICOT: f32 = 20.;
const RADIUS_PLUM: f32 = 28.;
const RADIUS_ORANGE: f32 = 40.;
const RADIUS_APPLE: f32 = 56.;
const RADIUS_GRAPEFRUIT: f32 = 80.;
const RADIUS_HONEYDEW: f32 = 112.;
const RADIUS_BASKETBALL: f32 = 160.;
const RADIUS_WATERMELON: f32 = 224.;

const GRAVITY: f32 = -100.;
const DENSITY: f32 = 1e2;
const SPRING: f32 = 1e2;
const DAMPER: f32 = 1e1;
const BOUNCE: f32 = 1.3;

pub struct FruitGame;

impl Plugin for FruitGame {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, (load_container, load_player))
        .add_systems(Update, (
            player_input,
        ))
        .add_systems(FixedUpdate, (
            apply_velocity,
            apply_acceleration,
            apply_gravity,
            check_wall_collisions,
            check_fruit_collisions,
            merge,
        ).chain())
        .add_systems(RunFixedMainLoop, (
            interpolate_rendered_transform.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
        ))
        .add_event::<CollisionEvent>()
        ;
    }
}


#[derive(Component, Clone, Copy, Default, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum FruitType {
    #[default]
    Blueberry,
    Cherry,
    Apricot,
    Plum,
    Orange,
    Apple,
    Grapefruit,
    Honeydew,
    Basketball,
    Watermelon,
}

impl FruitType {
    pub fn next(&self) -> Option<FruitType> {
        match self {
            FruitType::Blueberry => Some(FruitType::Cherry),
            FruitType::Cherry => Some(FruitType::Apricot),
            FruitType::Apricot => Some(FruitType::Plum),
            FruitType::Plum => Some(FruitType::Orange),
            FruitType::Orange => Some(FruitType::Apple),
            FruitType::Apple => Some(FruitType::Grapefruit),
            FruitType::Grapefruit => Some(FruitType::Honeydew),
            FruitType::Honeydew => Some(FruitType::Basketball),
            FruitType::Basketball => Some(FruitType::Watermelon),
            FruitType::Watermelon => None,
        }
    }
    pub fn to_circle(&self) -> Circle {
        match self {
            FruitType::Blueberry => Circle::new(RADIUS_BLUEBERRY),
            FruitType::Cherry => Circle::new(RADIUS_CHERRY),
            FruitType::Apricot => Circle::new(RADIUS_APRICOT),
            FruitType::Plum => Circle::new(RADIUS_PLUM),
            FruitType::Orange => Circle::new(RADIUS_ORANGE),
            FruitType::Apple => Circle::new(RADIUS_APPLE),
            FruitType::Grapefruit => Circle::new(RADIUS_GRAPEFRUIT),
            FruitType::Honeydew => Circle::new(RADIUS_HONEYDEW),
            FruitType::Basketball => Circle::new(RADIUS_BASKETBALL),
            FruitType::Watermelon => Circle::new(RADIUS_WATERMELON),
        }
    }

    pub fn color(&self) -> Color {
        match self {
            FruitType::Blueberry => Color::from(BLUE),
            FruitType::Cherry => Color::from(RED),
            FruitType::Apricot => Color::from(OLIVE),
            FruitType::Plum => Color::from(PURPLE),
            FruitType::Orange => Color::from(AQUA),
            FruitType::Apple => Color::from(LIME),
            FruitType::Grapefruit => Color::from(GRAY),
            FruitType::Honeydew => Color::from(SILVER),
            FruitType::Basketball => Color::from(NAVY),
            FruitType::Watermelon => Color::from(GREEN),
        }
    }

    pub fn mass(&self) -> f32 {
        let r = self.to_circle().radius;
        r * r * DENSITY
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
            material: MeshMaterial2d(materials.add(typ.color())),
            ..Default::default()
        }
    }
}

#[derive(Component, Default)]
pub struct Wall;

fn load_container(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let layer = 100.;
    let color = Color::from(WHITE);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(THICKNESS, TOP - BOTTOM))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(RIGHT, 0., layer),
        Wall,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(THICKNESS, TOP - BOTTOM))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(LEFT, 0., layer),
        Wall,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(RIGHT - LEFT, THICKNESS))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0., BOTTOM, layer),
        Wall,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(RIGHT - LEFT, THICKNESS))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0., TOP, layer),
        Wall,
    ));
}

#[derive(Component)]
pub struct Player;


#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Position(Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousPosition(Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Velocity(Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Acceleration(Vec2);

#[derive(Component)]
pub struct DigitalInput {
    keys: Vec<String>,
}

impl DigitalInput {
    pub fn to_x(&self) -> f32 {
        let input_string = format!("0.{}", self.keys.join(""));
        LEFT + (RIGHT - LEFT) * input_string.parse::<f32>().unwrap()
    }

    pub fn add_digit(&mut self, key: Res<ButtonInput<KeyCode>>) {
        let mut keys = key.get_just_pressed();
        if keys.len() != 1 {
            return;
        }

        let key = keys.next().unwrap();

        let s = match key {
            KeyCode::Digit0 => 0.to_string(),
            KeyCode::Digit1 => 1.to_string(),
            KeyCode::Digit2 => 2.to_string(),
            KeyCode::Digit3 => 3.to_string(),
            KeyCode::Digit4 => 4.to_string(),
            KeyCode::Digit5 => 5.to_string(),
            KeyCode::Digit6 => 6.to_string(),
            KeyCode::Digit7 => 7.to_string(),
            KeyCode::Digit8 => 8.to_string(),
            KeyCode::Digit9 => 9.to_string(),
            _ => "".to_string(),
        };
        if !s.is_empty() {
            self.keys.push(s);
        }
    }
}

fn load_player(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player {},
        DigitalInput { keys: vec![] },
        Fruit::new(FruitType::Blueberry, meshes, materials),
        Transform::from_xyz(0., TOP + RADIUS_BLUEBERRY, 0.),
    ));
}

fn player_input(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Single<(
        // &Player,
        &mut DigitalInput,
        &FruitType,
        &Mesh2d,
        &MeshMaterial2d<ColorMaterial>,
        &mut Transform,
        )>,
) {
    let (mut digital_input, typ, mesh, material, mut transform) = query.into_inner();

    if keyboard_input.just_pressed(KeyCode::Backspace) {
        digital_input.keys.pop();
    }
    else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        let mut fruit = Fruit {
            typ: *typ,
            mesh: mesh.clone(),
            material: material.clone(),
            pos: Position(transform.translation.truncate()),
            pre: PreviousPosition(transform.translation.truncate()),
            vel: Default::default(),
            acc: Default::default(),
        };
        fruit.vel.0 += Vec2::new(0., -50.);
        // drop fruit
        commands.spawn((
            fruit,
            *transform,
            Collider,
        ));
    }
    else {
        digital_input.add_digit(keyboard_input);
    }

    transform.translation.x = digital_input.to_x();
}


fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Position, &mut PreviousPosition, &Velocity, &Acceleration)>
) {
    let dt = time.delta_secs();
    let dt2 = 0.5 * dt * dt;
    for (mut pos, mut pre, vel, acc) in &mut query {
        pre.0 = pos.0;
        pos.0 += vel.0 * dt + 0.5 * acc.0 * dt2;
    }
}

fn apply_acceleration(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut Acceleration)>,
) {
    let dt = time.delta_secs();
    for (mut vel, mut acc) in &mut query {
        vel.0 += acc.0 * dt;
        acc.0 *= 0.;
    }
}

fn apply_gravity(
    mut query: Query<&mut Acceleration, (With<FruitType>, Without<Player>)>,
) {
    for mut acc in &mut query {
        acc.0 += Vec2::new(0., GRAVITY);
    }
}



fn interpolate_rendered_transform(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut Transform,
        &Position,
        &PreviousPosition,
    ), (With<FruitType>, Without<Player>)>,
) {
    for (mut transform, pos, pre) in query.iter_mut() {
        let previous = pre.0;
        let current = pos.0;
        // The overstep fraction is a value between 0 and 1 that tells us how far we are between two fixed timesteps.
        let alpha = fixed_time.overstep_fraction();

        let rendered_translation = previous.lerp(current, alpha);
        transform.translation = rendered_translation.extend(1.0);
    }
}

#[derive(Component, Default)]
pub struct Collider;

#[derive(Event)]
pub struct CollisionEvent([(Entity, FruitType, Position, Velocity, Acceleration); 2]);

fn check_wall_collisions(
    collider_query: Query<(&FruitType, &mut Position, &mut Velocity, &mut Acceleration), With<Collider>>,
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
    }
}

fn check_fruit_collisions(
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
                // warn!("{fruit:?} - toward_other: {toward_other:?}");
                pos.0 -= squish;

                let vel_toward_other = vel.dot(toward_other);
                // warn!("{fruit:?} - vel before: {vel:?}");
                vel.0 -= toward_other * vel_toward_other * BOUNCE;
                // warn!("{fruit:?} - vel after: {vel:?}");

                let spring_force = squish / fruit.to_circle().radius * SPRING;
                // warn!("{fruit:?} - spring_force: {spring_force:?}");
                let damp_force = toward_other * (vel_toward_other * DAMPER).max(0.);
                // warn!("{fruit:?} - damp_force: {damp_force:?}");
                // warn!("{fruit:?} - acc before: {acc:?}");
                acc.0 -= (spring_force + damp_force) / fruit.mass();
                // warn!("{fruit:?} - acc after: {acc:?}");
            }

            let toward_other = seg.direction().as_vec2();
            reaction(toward_other, overlap, &fruit0, &mut pos0, &mut vel0, &mut acc0);
            // let squish = toward_other * (overlap / 2.);
            // warn!("toward_other: {toward_other:?}");
            // pos0.0 -= squish;

            // let vel0_toward_other = vel0.dot(toward_other);
            // warn!("vel0 before: {vel0:?}");
            // vel0.0 -= toward_other * vel0_toward_other * BOUNCE;
            // warn!("vel0 before: {vel0:?}");

            // let spring_force = squish / radius0 * SPRING;
            // warn!("spring_force: {spring_force:?}");
            // let damp_force0 = toward_other * (vel0_toward_other * DAMPER).max(0.);
            // warn!("damp_force: {damp_force0:?}");
            // warn!("acc0 before: {acc0:?}");
            // acc0.0 -= (spring_force + damp_force0) / fruit0.mass();
            // warn!("acc0 after: {acc0:?}");


            let toward_other = seg.direction().as_vec2() * -1.;
            reaction(toward_other, overlap, &fruit1, &mut pos1, &mut vel1, &mut acc1);
        //     let squish = toward_other * (overlap / 2.);
        //     pos1.0 -= squish;

        //     let vel1_toward_other = vel1.dot(toward_other);
        //     vel1.0 -= toward_other * vel1_toward_other * BOUNCE;

        //     let spring_force = squish / radius1 * SPRING;
        //     let damp_force1 = toward_other * (vel1_toward_other * DAMPER).max(0.);
        //     warn!("acc1 before: {acc1:?}");
        //     acc1.0 -= (spring_force + damp_force1) / fruit1.mass();
        //     warn!("acc1 after: {acc1:?}");
        }
    }

}

fn merge(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for collision in collisions.read() {
        let (entity0, fruit0, pos0, _, _) = collision.0[0];
        let (entity1, fruit1, pos1, _, _) = collision.0[1];
        if fruit0 != fruit1 {
            continue;
        }
        if let Some(new_type) = fruit0.next() {
            let midpoint = (pos0.0 + pos1.0) / 2.;
            // warn!("{midpoint:?}");

            commands.entity(entity0).despawn();
            commands.entity(entity1).despawn();

            let mut merged_fruit = Fruit::new(new_type, meshes, materials);
            merged_fruit.pos.0 = midpoint;
            merged_fruit.pre.0 = midpoint;
            commands.spawn((
                merged_fruit,
                Transform::from_xyz(midpoint.x, midpoint.y, 0.),
                Collider,
            ));
            break;
        }

    }
}
