extern crate amethyst;

use crate::pong::Pong;
use amethyst::{
    config::{Config, ConfigError},
    core::transform::TransformBundle,
    input::InputBundle,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    ui::{DrawUi, UiBundle},
    utils::application_root_dir,
    {Application, GameDataBuilder, Logger, LogLevelFilter},
};

mod arena;
mod ball;
mod paddle;
mod pong;
mod score;
mod velocity;
mod winner;

fn main() -> amethyst::Result<()> {
    Logger::from_config(Default::default())
        .level_for("gfx_device_gl", LogLevelFilter::Warn) // TODO: silence: [INFO][gfx_device_gl::factory]  Created buffer 5
        .start();

    let display_config = load_display_config();
    let input_bundle = load_input_bundle()?;
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.02, 0.02, 0.02, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
            .with_pass(DrawUi::new()),
    );
    let render_bundle = RenderBundle::new(pipe, Some(display_config)).with_sprite_sheet_processor();

    let game_data = GameDataBuilder::default()
        .with_bundle(render_bundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(paddle::PaddleSystem, "paddle_system", &["input_system"])
        .with(ball::BallMoveSystem, "ball_move_system", &[])
        .with(
            ball::BallBounceSystem,
            "ball_bounce_system",
            &["paddle_system", "ball_move_system"],
        )
        .with(winner::WinnerSystem, "winner_system", &["ball_move_system"]);

    let mut game = Application::new("./", Pong::new(), game_data)?;

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
