use amethyst::ecs::{Component, DenseVecStorage};

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    pub fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: 4.0,
            height: 16.0,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}
