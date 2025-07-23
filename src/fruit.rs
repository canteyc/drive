use bevy::{
    color::palettes::basic::*,
    prelude::*,
    time::common_conditions::on_timer,
};
use std::fmt::{Display, Formatter, Error};
use std::time::Duration;
use std::collections::BTreeSet;
use rand::seq::IndexedRandom;


const RIGHT: f32 = 300.;
const LEFT: f32 = RIGHT * -1.0;
const TOP: f32 = 300.;
const BOTTOM: f32 = -300.;
const THICKNESS: f32 = 2.;

const RADIUS_BLUEBERRY: f32 = 10.0;
const RADIUS_CHERRY: f32 = RADIUS_BLUEBERRY * 1.414;
const RADIUS_APRICOT: f32 = RADIUS_BLUEBERRY * 2.0;
const RADIUS_PLUM: f32 = RADIUS_CHERRY * 2.0;
const RADIUS_ORANGE: f32 = RADIUS_APRICOT * 2.0;
const RADIUS_APPLE: f32 = RADIUS_PLUM * 2.0;
const RADIUS_GRAPEFRUIT: f32 = RADIUS_ORANGE * 2.0;
const RADIUS_HONEYDEW: f32 = RADIUS_APPLE * 2.0;
const RADIUS_BASKETBALL: f32 = RADIUS_GRAPEFRUIT * 2.0;
const RADIUS_WATERMELON: f32 = RADIUS_HONEYDEW * 2.0;

const GRAVITY: f32 = -100.;
const DENSITY: f32 = 1e2;
const SPRING: f32 = 1e2;
const DAMPER: f32 = 1e1;
const BOUNCE: f32 = 1.3;

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
            reset.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop).run_if(on_event::<ResetEvent>),
        ))
        .add_event::<CollisionEvent>()
        .add_event::<ResetEvent>()
        .add_event::<DropEvent>()
        .init_resource::<Events<KeyHoldEvent>>()
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
    const ALL: [FruitType; 10] = [
        FruitType::Blueberry,
        FruitType::Cherry,
        FruitType::Apricot,
        FruitType::Plum,
        FruitType::Orange,
        FruitType::Apple,
        FruitType::Grapefruit,
        FruitType::Honeydew,
        FruitType::Basketball,
        FruitType::Watermelon,
    ];

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

    pub fn rand_to(
        upper: FruitType,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        let mut rng = rand::rng();
        let choices: Vec<FruitType> = FruitType::ALL.into_iter().take_while(|typ| typ <= &upper).collect();
        let typ = choices.choose(&mut rng).unwrap();
        Self::new(*typ, meshes, materials)
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

#[derive(Debug, Component, Clone, PartialEq, Default, Deref, DerefMut)]
pub struct DigitalInput {
    keys: Vec<String>,
}

impl DigitalInput {
    pub fn to_x(&self) -> f32 {
        LEFT + (RIGHT - LEFT) * self.to_string().parse::<f32>().unwrap()
    }

    pub fn add_digit(&mut self, key: KeyCode) {
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

impl Display for DigitalInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "0.{}", self.keys.join(""))
    }
}

fn load_player(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player {},
        DigitalInput { keys: vec!["5".to_string()] },
        Fruit::new(FruitType::Blueberry, meshes, materials),
        Transform::from_xyz(0., TOP + RADIUS_BLUEBERRY, 0.),
    ));
    commands.insert_resource(AccumulatedInput(Default::default()));
    commands.insert_resource(PreviousAccumulatedInput(Default::default()));
}


#[derive(Debug, Component, Clone, Copy, PartialEq, Default)]
pub struct PositionDisplay;

fn load_input_display(
    mut commands: Commands,
) {
    commands.spawn((
        PositionDisplay,
        Text2d::new(""),
        Transform::from_xyz(LEFT + 20., BOTTOM - 40., 0.),
    ));
}

fn player_input(
    mut input: ResMut<AccumulatedInput>,
    mut previous_input: ResMut<PreviousAccumulatedInput>,
    digital_input: Single<(&mut DigitalInput, &mut Transform)>,
    position_display: Single<&mut Text2d, With<PositionDisplay>>,
    mut set_reset: EventWriter<ResetEvent>,
    mut hold_event: ResMut<Events<KeyHoldEvent>>,
    mut drop_event: EventWriter<DropEvent>,
) {
    hold_event.update();
    let (mut digital_input, mut transform) = digital_input.into_inner();

    if input.remove(&KeyCode::Backspace) {
        if input.remove(&KeyCode::ShiftLeft) {
            set_reset.write(ResetEvent);
        } else {
            digital_input.keys.pop();
        }
    }
    if input.remove(&KeyCode::ArrowDown) {
        if previous_input.remove(&KeyCode::ArrowDown) {
            hold_event.send(KeyHoldEvent(KeyCode::ArrowDown));
        } else {
            drop_event.write(DropEvent);
        }
        previous_input.insert(KeyCode::ArrowDown);
    } else {
        previous_input.remove(&KeyCode::ArrowDown);
    }
    let (dir, index) = {(
        input.remove(&KeyCode::ArrowRight).then_some(1).or_else( || {
            input.remove(&KeyCode::ArrowLeft).then_some(-1)
        }),
        if input.remove(&KeyCode::ControlLeft) || input.remove(&KeyCode::ControlRight) {
            0
        } else if digital_input.is_empty() {
            0
        } else {
            digital_input.len() - 1
        }
    )};
    if let Some(dir) = dir {
        let mut value = digital_input.is_empty().then_some(0).unwrap_or_else(|| digital_input.remove(index).parse::<i32>().unwrap());
        value += dir;
        value = value.clamp(0, 9);
        digital_input.insert(index, value.to_string());
    }

    while let Some(key) = input.pop_first() {
        digital_input.add_digit(key);
    }

    transform.translation.x = digital_input.to_x();
    position_display.into_inner().0 = digital_input.to_string();
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
    let radius = typ.to_circle().radius;
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
    transform.translation.y = TOP + typ.to_circle().radius;
}

#[derive(Debug, Event, Clone, PartialEq)]
pub struct DropEvent;

#[derive(Debug, Event, Clone, PartialEq, Deref, DerefMut)]
pub struct KeyHoldEvent(KeyCode);

#[derive(Debug, Clone, PartialEq, Default, Deref, DerefMut, Resource)]
pub struct AccumulatedInput(BTreeSet<KeyCode>);

#[derive(Debug, Clone, PartialEq, Default, Deref, DerefMut, Resource)]
pub struct PreviousAccumulatedInput(BTreeSet<KeyCode>);

fn record_key_press(
    mut input: ResMut<AccumulatedInput>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for key in keyboard_input.get_pressed() {
        input.insert(*key);
    }
}

fn fast_drop(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut drop_event: EventWriter<DropEvent>,
    mut hold_event: ResMut<Events<KeyHoldEvent>>,
) {
    let event = hold_event.drain().find(|event|**event == KeyCode::ArrowDown);
    if event.is_some() {
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            drop_event.write(DropEvent);
            hold_event.send(KeyHoldEvent(KeyCode::ArrowDown));
        } else {
            hold_event.clear();
        }
    }
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

            if let Ok(mut e) = commands.get_entity(entity0) {
                e.despawn();
            }
            if let Ok(mut e) = commands.get_entity(entity1) {
                e.despawn();
            }

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


#[derive(Event)]
pub struct ResetEvent;

fn reset(
    mut commands: Commands,
    query: Query<Entity, (With<FruitType>, With<Collider>)>,
    _reader: EventReader<ResetEvent>,
) {
    warn!("reset");
    for entity in query {
        commands.entity(entity).despawn();
    }
}
