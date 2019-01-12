use crate::arena::Arena;
use amethyst::{
    core::{nalgebra::clamp, timing::Time, Transform},
    ecs::{Component, DenseVecStorage, Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::InputHandler,
};

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

pub struct PaddleSystem;

#[allow(clippy::type_complexity)]
impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
        ReadExpect<'s, Arena>,
    );

    fn run(&mut self, (mut transforms, paddles, input, time, arena): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };
            if let Some(mv_amount) = movement {
                let y =
                    transform.translation().y + (time.delta_seconds() * 50.0 * mv_amount as f32);
                let y = clamp(y, 6.0, arena.height - 6.0);
                transform.set_y(y);
            }
        }
    }
}
