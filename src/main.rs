use bevy::prelude::*;

use drive::fruit::FruitGame;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FruitGame)
        .run();
}

