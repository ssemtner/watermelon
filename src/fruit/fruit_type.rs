use bevy::prelude::*;

#[derive(Component)]
pub struct Fruit(pub FruitType);

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum FruitType {
    Blueberry,
    Strawberry,
    Grapes,
    Lemon,
    Coconut,
    Apple,
    Orange,
    Pomegranate,
    Peach,
    Pineapple,
    Melon,
    Watermelon,
}

impl FruitType {
    pub fn random() -> FruitType {
        match rand::random::<u8>() % 5 {
            0 => FruitType::Blueberry,
            1 => FruitType::Strawberry,
            2 => FruitType::Grapes,
            3 => FruitType::Lemon,
            4 => FruitType::Coconut,
            _ => unreachable!(),
        }
    }

    pub fn size(&self) -> f32 {
        match self {
            FruitType::Blueberry => 30.0,
            FruitType::Strawberry => 50.0,
            FruitType::Grapes => 70.0,
            FruitType::Lemon => 100.0,
            FruitType::Coconut => 120.0,
            FruitType::Apple => 150.0,
            FruitType::Orange => 180.0,
            FruitType::Pomegranate => 210.0,
            FruitType::Peach => 240.0,
            FruitType::Pineapple => 270.0,
            FruitType::Melon => 300.0,
            FruitType::Watermelon => 330.0,
        }
    }

    pub fn merge(&self) -> Option<FruitType> {
        match self {
            FruitType::Blueberry => Some(FruitType::Strawberry),
            FruitType::Strawberry => Some(FruitType::Grapes),
            FruitType::Grapes => Some(FruitType::Lemon),
            FruitType::Lemon => Some(FruitType::Coconut),
            FruitType::Coconut => Some(FruitType::Apple),
            FruitType::Apple => Some(FruitType::Orange),
            FruitType::Orange => Some(FruitType::Pomegranate),
            FruitType::Pomegranate => Some(FruitType::Peach),
            FruitType::Peach => Some(FruitType::Pineapple),
            FruitType::Pineapple => Some(FruitType::Melon),
            FruitType::Melon => Some(FruitType::Watermelon),
            FruitType::Watermelon => None,
        }
    }
}
