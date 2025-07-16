use bevy::{
    color::palettes::basic::*,
    // math::bounding::{Aabb2d, BoundingCircle},
    prelude::*,
};


const RIGHT: f32 = 200.;
const LEFT: f32 = -200.;
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

pub struct FruitGame;

impl Plugin for FruitGame {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, (load_container, load_player))
        .add_systems(Update, (
            player_input,
            check_wall_collisions,
            check_fruit_collisions,
        ))
        .add_systems(FixedUpdate, (
            apply_velocity,
        ))
        .add_systems(RunFixedMainLoop, (
            apply_gravity.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
            check_wall_collisions.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
            check_fruit_collisions.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
            interpolate_rendered_transform.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
        ))
        ;
    }
}


#[derive(Component, Clone, Copy, Default)]
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
            FruitType::Cherry => Color::from(BLUE),
            FruitType::Apricot => Color::from(BLUE),
            FruitType::Plum => Color::from(BLUE),
            FruitType::Orange => Color::from(BLUE),
            FruitType::Apple => Color::from(BLUE),
            FruitType::Grapefruit => Color::from(BLUE),
            FruitType::Honeydew => Color::from(BLUE),
            FruitType::Basketball => Color::from(BLUE),
            FruitType::Watermelon => Color::from(BLUE),
        }
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
    // pub acc: Acceleration,
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

//     pub fn Cherry() -> Self {}
//
//     pub fn Apricot() -> Self {}
//
//     pub fn Plum() -> Self {}
//
//     pub fn Orange() -> Self {}
//
//     pub fn Apple() -> Self {}
//
//     pub fn Grapefruit() -> Self {}
//
//     pub fn Honeydew() -> Self {}
//
//     pub fn Basketball() -> Self {}
//
//     pub fn Watermelon() -> Self {}

}

// fn spawn_fruit(
//     mut commands: Commands,
//     meshes: ResMut<Assets<Mesh>>,
//     materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     commands.spawn(Fruit::blueberry(meshes, materials));
// }

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
        Transform::from_xyz(0., TOP, 0.),
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
    else if keyboard_input.just_pressed(KeyCode::Space) {
        let fruit = Fruit {
            typ: *typ,
            mesh: mesh.clone(),
            material: material.clone(),
            pos: Position(transform.translation.truncate()),
            pre: PreviousPosition(transform.translation.truncate()),
            vel: Default::default(),
            // acc: Default::default(),
        };
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
    mut query: Query<(&mut Position, &mut PreviousPosition, &Velocity)>
) {
    for (mut pos, mut pre, vel) in &mut query {
        pre.0 = pos.0;
        pos.0 += vel.0 * time.delta_secs();
    }
}

fn apply_gravity(
    time: Res<Time>,
    mut query: Query<&mut Velocity, (With<FruitType>, Without<Player>)>,
) {
    let acc = Vec2::new(0., GRAVITY);
    for mut vel in &mut query {
        vel.0 += acc * time.delta_secs();
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

fn check_wall_collisions(
    collider_query: Query<(&FruitType, &mut Position, &mut PreviousPosition, &mut Velocity), With<Collider>>,
    // wall_query: Query<(&Transform,), With<Wall>>,
) {
    for (fruit, mut fruit_position, mut fruit_previous_position, mut fruit_velocity) in collider_query {
        let radius = fruit.to_circle().radius;
        if fruit_position.x > RIGHT - radius {
            fruit_position.x = RIGHT - radius;
            fruit_previous_position.x = RIGHT - radius;
            fruit_velocity.0.x = 0.;
        }
        if fruit_position.x < LEFT + radius {
            fruit_position.x = LEFT + radius;
            fruit_previous_position.x = LEFT + radius;
            fruit_velocity.0.x = 0.;
        }
        if fruit_position.y < BOTTOM + radius {
            fruit_position.y = BOTTOM + radius;
            fruit_previous_position.y = BOTTOM + radius;
            fruit_velocity.0.y = 0.;
        }
    }
}

fn check_fruit_collisions(
    mut collider_query: Query<(&FruitType, &mut Position, &mut PreviousPosition, &mut Velocity), With<Collider>>,
) {
    let mut combinations = collider_query.iter_combinations_mut();
    while let Some([
        (fruit0, mut fruit_position0, mut fruit_previous_position0, mut fruit_velocity0),
        (fruit1, mut fruit_position1, mut fruit_previous_position1, mut fruit_velocity1),
    ]) = combinations.fetch_next() {
        let radius0 = fruit0.to_circle().radius;
        let radius1 = fruit1.to_circle().radius;

        let seg = Segment2d::new(fruit_position0.0, fruit_position1.0);
        let overlap = radius0 + radius1 - seg.length();
        if overlap > 0. && overlap < radius0 + radius1 {
            // collision!
            let dist0 = radius0 - (overlap / 2.);
            let dist1 = radius1 - (overlap / 2.);
            fruit_position0.0 -= seg.direction() * dist0;
            fruit_position1.0 += seg.direction() * dist1;
            fruit_previous_position0.0 = fruit_position0.0;
            fruit_previous_position1.0 = fruit_position1.0;
            fruit_velocity0.0 -= seg.direction() * 10.;
            fruit_velocity1.0 += seg.direction() * 10.;
        }
    }

}
