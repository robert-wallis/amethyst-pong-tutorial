use crate::{arena::Arena, velocity::Velocity};
use amethyst::{
    core::{nalgebra::Vector2, transform::Transform},
    ecs::{Builder, Component, DenseVecStorage, World},
    renderer::{SpriteRender, SpriteSheetHandle},
};

mod bounce;
pub use self::bounce::BallBounceSystem;

mod movement;
pub use self::movement::BallMoveSystem;

pub const BALL_VELOCITY_X: f32 = 12.0;
pub const BALL_VELOCITY_Y: f32 = 25.0;
pub const BALL_RADIUS: f32 = 2.0;

pub struct Ball {
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

impl Ball {}

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
