use crate::components::{Ball, Paddle, Side, Velocity};
use crate::pong::ARENA_HEIGHT;
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

pub struct BounceBallSystem;

impl<'s> System<'s> for BounceBallSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, mut velocities, paddles, transforms): Self::SystemData) {
        for (ball, velocity, ball_transform) in (&mut balls, &mut velocities, &transforms).join() {
            let ball_pos = ball_transform.translation();
            let velocity = &mut velocity.0;

            // bounce off arena ceiling
            if ball_pos.y >= ARENA_HEIGHT - ball.radius && velocity.y > 0.0 {
                velocity.y = -velocity.y;
            }
            // bounce off arena floor
            if ball_pos.y <= ball.radius && velocity.y < 0.0 {
                velocity.y = -velocity.y;
            }

            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                let pt = paddle_transform.translation();
                let px = pt.x - paddle.width * 0.5;
                let py = pt.y - paddle.height * 0.5;
                if point_in_rect(
                    ball_pos.x,
                    ball_pos.y,
                    px - ball.radius,
                    px + paddle.width + ball.radius,
                    py + paddle.height + ball.radius,
                    py - ball.radius,
                ) && (paddle.side == Side::Left && velocity.x < 0.0
                    || paddle.side == Side::Right && velocity.x > 0.0)
                {
                    velocity.x = -velocity.x;
                }
            }
        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, right: f32, top: f32, bottom: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}
