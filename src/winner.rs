use crate::{
    arena::Arena,
    ball::Ball,
    score::{ScoreBoard, ScoreText},
    velocity::Velocity,
};
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadExpect, ReadStorage, System, Write, WriteStorage},
    ui::UiText,
};
use std::fmt;

pub struct WinnerSystem;

#[allow(clippy::type_complexity)]
impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
        ReadExpect<'s, Arena>,
    );

    fn run(
        &mut self,
        (balls, mut velocities, mut locals, mut ui_texts, mut score_board, score_text, arena): Self::SystemData,
    ) {
        for (ball, velocity, transform) in (&balls, &mut velocities, &mut locals).join() {
            let ball_pos = transform.translation();

            let winner = if ball_pos.x <= ball.radius {
                Winner::Left
            } else if ball_pos.x >= arena.width - ball.radius {
                Winner::Right
            } else {
                Winner::None
            };

            match winner {
                Winner::Left => {
                    score_board.left = (score_board.left + 1).min(999);
                    if let Some(text) = ui_texts.get_mut(score_text.left) {
                        text.text = score_board.left.to_string();
                    }
                }
                Winner::Right => {
                    score_board.right = (score_board.right + 1).min(999);
                    if let Some(text) = ui_texts.get_mut(score_text.right) {
                        text.text = score_board.right.to_string();
                    }
                }
                Winner::None => (),
            }

            if let Winner::None = winner {
            } else {
                // reset ball
                velocity.0.x = -velocity.0.x;
                transform.set_x(arena.width / 2.0);
            }
        }
    }
}

enum Winner {
    Left,
    Right,
    None,
}

impl fmt::Display for Winner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Winner::Left => write!(f, "Left Wins"),
            Winner::Right => write!(f, "Right Wins"),
            Winner::None => write!(f, "No Winner"),
        }
    }
}
