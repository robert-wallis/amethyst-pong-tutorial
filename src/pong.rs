use crate::{arena::Arena, ball, paddle, score};
use amethyst::{
    assets::{AssetStorage, Loader},
    core::{nalgebra::Vector2, transform::Transform},
    ecs::{Builder, Entity, World},
    input::InputHandler,
    renderer::{
        Camera, PngFormat, Projection, ScreenDimensions, SpriteSheet, SpriteSheetFormat,
        SpriteSheetHandle, Texture, TextureMetadata,
    },
    winit::{Event::WindowEvent, WindowEvent::Resized},
    {GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans},
};

/// The gameplay state.
pub struct Pong {
    /// Current main camera.
    camera: Option<Entity>,
}

impl Pong {
    pub fn new() -> Pong {
        Pong { camera: None }
    }

    /// Create a new arena and update systems related to arena size.
    fn resize_arena(&mut self, world: &mut World, size: (f32, f32)) {
        let arena = Arena::new_from_screen(size.0, size.1);
        self.init_main_camera(world, &arena);
        paddle::update_paddle_locations(&world, &arena);
        world.add_resource(arena);
    }

    /// Re-initialize the main camera, esp. when the arena size changes.
    fn init_main_camera(&mut self, world: &mut World, arena: &Arena) {
        if let Some(camera) = self.camera {
            let _ = world.delete_entity(camera);
        }
        self.camera = Some(init_camera(world, arena))
    }
}

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let screen = screen_dimensions(world);
        let sprite_sheet = init_sprite_sheet(world);
        let ball_resources = ball::BallResources {
            sprite_sheet: sprite_sheet.clone(),
        };
        world.add_resource(ball_resources);
        let arena = Arena::new_from_screen(screen.x, screen.y);
        self.init_main_camera(world, &arena);
        paddle::init_entities(world, &arena, sprite_sheet);
        score::init_entities(world);
        world.add_resource(arena);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let mut world = data.world;
        {
            let input = world.read_resource::<InputHandler<String, String>>();
            if input.action_is_down("quit").unwrap_or(false) {
                println!("SimpleState::action::quit");
                return Trans::Quit;
            }
        }

        #[allow(clippy::single_match)]
        match event {
            StateEvent::Window(WindowEvent {
                event: Resized(size),
                ..
            }) => {
                self.resize_arena(&mut world, (size.width as f32, size.height as f32));
            }
            _ => (),
        }

        Trans::None
    }
}

fn init_camera(world: &mut World, arena: &Arena) -> Entity {
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
        .build()
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
