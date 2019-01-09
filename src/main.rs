extern crate amethyst;

use amethyst::{
    config::ConfigError,
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    utils::application_root_dir,
};

mod components;
mod pong;
mod systems;
use crate::pong::Pong;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let display_config = load_display_config();
    let input_bundle = load_input_bundle()?;
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.02, 0.02, 0.02, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );
    let render_bundle = RenderBundle::new(pipe, Some(display_config)).with_sprite_sheet_processor();

    let game_data = GameDataBuilder::default()
        .with_bundle(render_bundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::BallMoveSystem, "move_ball_system", &[])
        .with(
            systems::BounceBallSystem,
            "bounce_system",
            &["paddle_system", "move_ball_system"],
        );

    let mut game = Application::new("./", Pong, game_data)?;

    game.run();

    Ok(())
}

fn load_display_config() -> DisplayConfig {
    let path = format!("{}/resources/display_config.ron", application_root_dir());
    DisplayConfig::load(&path)
}

fn load_input_bundle() -> Result<InputBundle<String, String>, ConfigError> {
    let path = format!("{}/resources/bindings_config.ron", application_root_dir());
    InputBundle::<String, String>::new().with_bindings_from_file(path)
}
