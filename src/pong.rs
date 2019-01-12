use crate::{arena::Arena, ball::Ball, paddle::Paddle, score::ScoreBoard};
use amethyst::{
    assets::{AssetStorage, Loader},
    core::{nalgebra::Vector2, transform::Transform},
    ecs::{Builder, World},
    input::InputHandler,
    renderer::{
        Camera, PngFormat, Projection, ScreenDimensions, SpriteSheet, SpriteSheetFormat,
        SpriteSheetHandle, Texture, TextureMetadata,
    },
    {GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans},
};

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let screen = screen_dimensions(world);
        let sprite_sheet = init_sprite_sheet(world);
        let arena = Arena::new(screen.x / 4., screen.y / 4.);
        init_camera(world, &arena);
        Paddle::init_entities(world, &arena, sprite_sheet.clone());
        Ball::init_entity(world, &arena, sprite_sheet);
        ScoreBoard::init_entities(world);
        world.add_resource(arena);
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

fn screen_dimensions(world: &World) -> Vector2<f32> {
    let screen = world.read_resource::<ScreenDimensions>();
    Vector2::new(screen.width(), screen.height())
}
