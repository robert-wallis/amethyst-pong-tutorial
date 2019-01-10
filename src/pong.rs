use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::{nalgebra::Vector2, transform::Transform};
use amethyst::input::InputHandler;
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat,
    SpriteSheetHandle, Texture, TextureMetadata,
};

use super::arena::Arena;
use super::components::{Ball, Paddle, Side, Velocity};

pub const BALL_VELOCITY_X: f32 = 12.0;
pub const BALL_VELOCITY_Y: f32 = 25.0;
pub const BALL_RADIUS: f32 = 2.0;

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let sprite_sheet = init_sprite_sheet(data.world);
        let arena = init_arena();
        init_camera(data.world, &arena);
        init_paddles(data.world, &arena, sprite_sheet.clone());
        init_ball(data.world, &arena, sprite_sheet);
        data.world.add_resource(arena);
    }
    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        _event: StateEvent,
    ) -> SimpleTrans {
        let input = _data.world.read_resource::<InputHandler<String, String>>();
        if input.action_is_down("quit").unwrap_or(false) {
            println!("SimpleState::action::quit");
            return Trans::Quit;
        }
        Trans::None
    }
}

fn init_arena() -> Arena {
    Arena {
        width: 100.0,
        height: 100.0,
    }
}

fn init_camera(world: &mut World, arena: &Arena) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            arena.width,
            0.0,
            arena.height,
        )))
        .with(transform)
        .build();
}

fn init_paddles(world: &mut World, arena: &Arena, sprite_sheet: SpriteSheetHandle) {
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

    world
        .create_entity()
        .with(paddle_left)
        .with(left_transform)
        .with(sprite_render.clone())
        .build();

    world
        .create_entity()
        .with(paddle_right)
        .with(right_transform)
        .with(sprite_render)
        .with(Flipped::Horizontal)
        .build();
}

fn init_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let loader = world.read_resource::<Loader>();

    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "resources/pong_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "resources/pong_spritesheet.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}

fn init_ball(world: &mut World, arena: &Arena, sprite_sheet: SpriteSheetHandle) {
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
