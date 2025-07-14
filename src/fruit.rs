use bevy::{
    color::palettes::basic::*,
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

pub struct FruitGame;

impl Plugin for FruitGame {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, (load_container, load_player))
        // .add_systems(Update, (
        //     spawn_fruit.run_if(not(any_with_component::<FruitState::Held>))
        // ))
        .add_systems(Update, (
            move_player,
        ))
        ;
    }
}


#[derive(Component)]
pub enum FruitType {
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

#[derive(Component)]
pub enum FruitState {
    Held,
    Dropped,
}

#[derive(Bundle)]
pub struct Fruit {
    pub typ: FruitType,
    pub state: FruitState,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
}

impl Fruit {
    pub fn blueberry(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        Self {
            typ: FruitType::Blueberry,
            state: FruitState::Held,
            mesh: Mesh2d(meshes.add(Circle::new(RADIUS_BLUEBERRY))),
            material: MeshMaterial2d(materials.add(Color::from(BLUE))),
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

fn spawn_fruit(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Fruit::blueberry(meshes, materials));
}

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
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(THICKNESS, TOP - BOTTOM))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(LEFT, 0., layer),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(RIGHT - LEFT, THICKNESS))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0., BOTTOM, layer),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(RIGHT - LEFT, THICKNESS))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0., TOP, layer),
    ));
}

#[derive(Component)]
pub struct Player;

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
        Fruit::blueberry(meshes, materials),
        Transform::from_xyz(0., TOP, 0.),
    ));
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Single<(&Player, &mut DigitalInput, &mut Transform)>,
) {
    let (_player, mut digital_input, mut transform) = query.into_inner();

    if keyboard_input.just_pressed(KeyCode::Backspace) {
        digital_input.keys.pop();
    }
    else if keyboard_input.just_pressed(KeyCode::Space) {
        // drop fruit
    }
    else {
        digital_input.add_digit(keyboard_input);
    }

    transform.translation.x = digital_input.to_x();
}
