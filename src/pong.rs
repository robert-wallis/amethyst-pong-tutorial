use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat,
    SpriteSheetHandle, Texture, TextureMetadata,
};

const ARENA_WIDTH: f32 = 100.0;
const ARENA_HEIGHT: f32 = 100.0;

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let sprite_sheet = init_sprite_sheet(data.world);
        init_camera(data.world);
        init_paddles(data.world, &sprite_sheet);
    }
}

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
    fn new(side: Side) -> Paddle {
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

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}

fn init_paddles(world: &mut World, sprite_sheet: &SpriteSheetHandle) {
    world.register::<Paddle>();

    let paddle_left = Paddle::new(Side::Left);
    let paddle_right = Paddle::new(Side::Right);

    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();
    let y = ARENA_HEIGHT / 2.0;
    left_transform.set_xyz(paddle_left.width * 0.5, y, 0.0);
    right_transform.set_xyz(ARENA_WIDTH - paddle_left.width * 0.5, y, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
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
