use amethyst::{
    ecs::{Component, DenseVecStorage},
    renderer::{SpriteSheetHandle},
};

mod bounce;
pub use self::bounce::BallBounceSystem;

mod movement;
pub use self::movement::BallMoveSystem;

mod spawn;
pub use self::spawn::BallSpawnSystem;

pub struct Ball {
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

impl Ball {}

pub struct BallResources {
    pub sprite_sheet: SpriteSheetHandle,
}
