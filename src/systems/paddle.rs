use crate::components::{Paddle, Side};
use amethyst::core::{nalgebra::*, timing::Time, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

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
