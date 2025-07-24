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
        let i = Self::ALL.binary_search(self).unwrap();
        Self::ALL.as_slice().get(i + 1).copied()
    }

    pub fn to_circle(&self) -> Circle {
        Circle::new(self.radius())
    }

    pub fn radius(&self) -> f32 {
        let mut r = RADIUS_BLUEBERRY;
        let i = Self::ALL.binary_search(self).unwrap();
        for _ in 0..i {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next() {
        assert_eq!(FruitType::Blueberry.next(), Some(FruitType::Cherry));
        assert_eq!(FruitType::Cherry.next(), Some(FruitType::Apricot));
        assert_eq!(FruitType::Apricot.next(), Some(FruitType::Plum));
        assert_eq!(FruitType::Plum.next(), Some(FruitType::Orange));
        assert_eq!(FruitType::Orange.next(), Some(FruitType::Apple));
        assert_eq!(FruitType::Apple.next(), Some(FruitType::Grapefruit));
        assert_eq!(FruitType::Grapefruit.next(), Some(FruitType::Honeydew));
        assert_eq!(FruitType::Honeydew.next(), Some(FruitType::Basketball));
        assert_eq!(FruitType::Basketball.next(), Some(FruitType::Watermelon));
        assert_eq!(FruitType::Watermelon.next(), None);
    }
}
