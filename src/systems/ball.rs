use crate::components::{Ball, Velocity};
use crate::arena::Arena;
use amethyst::{
    prelude::*,
    core::{timing::Time, transform::Transform, nalgebra::Vector2},
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
    renderer::{SpriteRender, SpriteSheetHandle},
};

pub const BALL_VELOCITY_X: f32 = 12.0;
pub const BALL_VELOCITY_Y: f32 = 25.0;
pub const BALL_RADIUS: f32 = 2.0;

pub struct BallMoveSystem;

impl BallMoveSystem {
    pub fn init_ball_entity(world: &mut World, arena: &Arena, sprite_sheet: SpriteSheetHandle) {
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
