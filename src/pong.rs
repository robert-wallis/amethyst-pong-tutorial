use crate::{arena::Arena, ball::Ball, paddle::Paddle, score::ScoreBoard};
use amethyst::{
    assets::{AssetStorage, Loader},
    core::{nalgebra::Vector2, transform::Transform},
    ecs::{Builder, Entity, World},
    input::InputHandler,
    renderer::{
        Camera, PngFormat, Projection, ScreenDimensions, SpriteSheet, SpriteSheetFormat,
        SpriteSheetHandle, Texture, TextureMetadata, WindowEvent,
    },
    {GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans},
};

pub struct Pong {
    camera: Option<Entity>,
    left_paddle: Option<Entity>,
    right_paddle: Option<Entity>,
}

impl Pong {
    pub fn new() -> Pong {
        Pong {
            camera: None,
            left_paddle: None,
            right_paddle: None,
        }
    }
}

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let screen = screen_dimensions(world);
        let sprite_sheet = init_sprite_sheet(world);
        let arena = Arena::new_from_screen(screen.x, screen.y);
        self.camera = Some(init_main_camera(world, &arena, self.camera));
        let (left, right) = Paddle::init_entities(world, &arena, sprite_sheet.clone());
        self.left_paddle = Some(left);
        self.right_paddle = Some(right);
        Ball::init_entity(world, &arena, sprite_sheet);
        ScoreBoard::init_entities(world);
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
        if let StateEvent::Window(amethyst::winit::Event::WindowEvent {
            event: WindowEvent::Resized(size),
            ..
        }) = event
        {
            let arena = Arena::new_from_screen(size.width as f32, size.height as f32);
            self.camera = Some(init_main_camera(&mut world, &arena, self.camera));
            if let Some(right_paddle) = self.right_paddle {
                let mut transforms = world.write_storage::<Transform>();
                if let Some(transform) = transforms.get(right_paddle) {
                    let mut transform = transform.clone();
                    transform.set_x(arena.width - 2.);
                    let _ = transforms.insert(right_paddle, transform);
                }
            }
            world.add_resource(arena);
        }
        Trans::None
    }
}

fn init_main_camera(world: &mut World, arena: &Arena, camera: Option<Entity>) -> Entity {
    if let Some(camera) = camera {
        let _ = world.delete_entity(camera);
    }
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
