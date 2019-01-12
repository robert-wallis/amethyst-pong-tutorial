use crate::arena::Arena;
use amethyst::{
    core::{nalgebra::clamp, timing::Time, Transform},
    ecs::{
        Builder, Component, DenseVecStorage, Entity, Join, Read, ReadExpect, ReadStorage, System,
        World, WriteStorage,
    },
    input::InputHandler,
    renderer::{Flipped, SpriteRender, SpriteSheetHandle},
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
    pub fn init_entities(
        world: &mut World,
        arena: &Arena,
        sprite_sheet: SpriteSheetHandle,
    ) -> (Entity, Entity) {
        let paddle_left = Paddle::new(Side::Left);
        let paddle_right = Paddle::new(Side::Right);

        let mut left_transform = Transform::default();
        let mut right_transform = Transform::default();
        let y = arena.height / 2.0;
        left_transform.set_xyz(paddle_left.width * 0.5, y, 0.0);
        right_transform.set_xyz(arena.width - paddle_left.width * 0.5, y, 0.0);

        let sprite_render = SpriteRender {
            sprite_sheet,
            sprite_number: 0,
        };

        (
            world
                .create_entity()
                .with(paddle_left)
                .with(left_transform)
                .with(sprite_render.clone())
                .build(),
            world
                .create_entity()
                .with(paddle_right)
                .with(right_transform)
                .with(sprite_render)
                .with(Flipped::Horizontal)
                .build(),
        )
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
