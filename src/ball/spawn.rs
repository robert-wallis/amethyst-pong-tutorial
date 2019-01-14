use rand::Rng;
use super::{Ball, BallResources};
use crate::{arena::Arena, velocity::Velocity};
use amethyst::{
    core::{nalgebra::Vector2, transform::Transform},
    ecs::{Builder, Entities, LazyUpdate, Read},
    ecs::{ReadExpect, ReadStorage, System},
    renderer::{SpriteRender},
};

pub const BALL_RADIUS: f32 = 2.0;

pub struct BallSpawnSystem;

impl<'s> System<'s> for BallSpawnSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        Read<'s, LazyUpdate>,
        Entities<'s>,
        ReadExpect<'s, Arena>,
        ReadExpect<'s, BallResources>,
    );

    fn run(&mut self, (balls, lazy, entities, arena, ball_resources): Self::SystemData) {
        if balls.is_empty() {
            let mut transform = Transform::default();
            transform.set_xyz(arena.width / 2.0, arena.height / 2.0, 0.0);

            let sprite_render = SpriteRender {
                sprite_sheet: ball_resources.sprite_sheet.clone(),
                sprite_number: 1,
            };

            let mut rng = rand::thread_rng();
            let vx: f32 = rng.gen_range(12., 40.);
            let vy: f32 = rng.gen_range(12., 40.);

            lazy.create_entity(&entities)
                .with(sprite_render)
                .with(Ball {
                    radius: BALL_RADIUS,
                })
                .with(Velocity(Vector2::new(vx, vy)))
                .with(transform)
                .build();
        }
    }
}
