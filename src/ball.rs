use crate::{
    arena::Arena,
    paddle::{Paddle, Side},
    velocity::Velocity,
};
use amethyst::{
    core::{nalgebra::Vector2, timing::Time, transform::Transform},
    ecs::{
        Builder, Component, DenseVecStorage, Join, Read, ReadExpect, ReadStorage, System, World,
        WriteStorage,
    },
    renderer::{SpriteRender, SpriteSheetHandle},
};

pub const BALL_VELOCITY_X: f32 = 12.0;
pub const BALL_VELOCITY_Y: f32 = 25.0;
pub const BALL_RADIUS: f32 = 2.0;

pub struct Ball {
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

impl Ball {
    pub fn init_entity(world: &mut World, arena: &Arena, sprite_sheet: SpriteSheetHandle) {
        let mut transform = Transform::default();
        transform.set_xyz(arena.width / 2.0, arena.height / 2.0, 0.0);

        let sprite_render = SpriteRender {
            sprite_sheet,
            sprite_number: 1,
        };

        world
            .create_entity()
            .with(sprite_render)
            .with(Ball {
                radius: BALL_RADIUS,
            })
            .with(Velocity(Vector2::new(BALL_VELOCITY_X, BALL_VELOCITY_Y)))
            .with(transform)
            .build();
    }
}

pub struct BallMoveSystem;

impl<'s> System<'s> for BallMoveSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (balls, velocities, mut locals, time): Self::SystemData) {
        for (_ball, velocity, local) in (&balls, &velocities, &mut locals).join() {
            let velocity = velocity.0;
            local.translate_x(velocity.x * time.delta_seconds());
            local.translate_y(velocity.y * time.delta_seconds());
        }
    }
}

pub struct BallBounceSystem;

#[allow(clippy::type_complexity)]
impl<'s> System<'s> for BallBounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, Arena>,
    );

    fn run(&mut self, (mut balls, mut velocities, paddles, transforms, arena): Self::SystemData) {
        for (ball, velocity, ball_transform) in (&mut balls, &mut velocities, &transforms).join() {
            let ball_pos = ball_transform.translation();
            let velocity = &mut velocity.0;

            // bounce off arena ceiling
            if ball_pos.y >= arena.height - ball.radius && velocity.y > 0.0 {
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
