use bevy::prelude::*;
use bevy::color::palettes::basic::*;
use rand::seq::IndexedRandom;

const RADIUS_BLUEBERRY: f32 = 10.0;
const DENSITY: f32 = 1e2;

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

    pub fn rand_up_to(upper: Self) -> Self {
        let mut rng = rand::rng();
        let choices: Vec<FruitType> = FruitType::ALL.into_iter().take_while(|typ| typ <= &upper).collect();
        *choices.choose(&mut rng).unwrap()
    }

    pub fn next(&self) -> Option<FruitType> {
        for i in 1..Self::ALL.len() - 1 {
            if Self::ALL[i - 1] == *self {
                return Some(Self::ALL[i]);
            }
        }
        None
    }

    pub fn to_circle(&self) -> Circle {
        Circle::new(self.radius())
    }

    pub fn radius(&self) -> f32 {
        let mut r = RADIUS_BLUEBERRY;
        let f = FruitType::Blueberry;
        while let Some(f) = f.next() && f < *self {
            r *= 1.41421356237;
        }
        r
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
        let r = self.radius();
        r * r * DENSITY
    }
}
