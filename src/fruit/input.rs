use std::collections::BTreeSet;
use std::fmt::{Display, Formatter, Error};

use bevy::prelude::*;

use crate::fruit::Fruit;
use crate::fruit::reset::ResetEvent;
use crate::fruit::typ::FruitType;
use crate::fruit::world::{BOTTOM, LEFT, RIGHT, TOP};

#[derive(Component)]
pub struct Player;

#[derive(Debug, Component, Clone, PartialEq, Default, Deref, DerefMut)]
pub struct DigitalInput {
    keys: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Deref, DerefMut, Resource)]
pub struct AccumulatedInput(BTreeSet<KeyCode>);

#[derive(Debug, Clone, PartialEq, Default, Deref, DerefMut, Resource)]
pub struct PreviousAccumulatedInput(BTreeSet<KeyCode>);

#[derive(Debug, Event, Clone, PartialEq)]
pub struct DropEvent;

#[derive(Debug, Event, Clone, PartialEq, Deref, DerefMut)]
pub struct KeyHoldEvent(KeyCode);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default)]
pub struct PositionDisplay;

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

pub fn load_player(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    let fruit = Fruit::new(FruitType::Blueberry, meshes, materials);
    commands.spawn((
        Player {},
        DigitalInput { keys: vec!["5".to_string()] },
        Transform::from_xyz(0., TOP + fruit.typ.radius(), 0.),
        fruit,
    ));
    commands.insert_resource(AccumulatedInput(Default::default()));
    commands.insert_resource(PreviousAccumulatedInput(Default::default()));
}


pub fn load_input_display(
    mut commands: Commands,
) {
    commands.spawn((
        PositionDisplay,
        Text2d::new(""),
        Transform::from_xyz(LEFT + 20., BOTTOM - 40., 0.),
    ));
}

pub fn player_input(
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

pub fn fast_drop(
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

pub fn record_key_press(
    mut input: ResMut<AccumulatedInput>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for key in keyboard_input.get_pressed() {
        input.insert(*key);
    }
}

