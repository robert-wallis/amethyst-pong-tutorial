use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::arena::Arena;
use crate::components::{Ball, Velocity};
use std::fmt;

pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Arena>,
    );

    fn run(&mut self, (balls, mut velocities, mut locals, arena): Self::SystemData) {
        for (ball, velocity, transform) in (&balls, &mut velocities, &mut locals).join() {
            let ball_pos = transform.translation();

            let winner = if ball_pos.x <= ball.radius {
                Winner::Left
            } else if ball_pos.x >= arena.width - ball.radius {
                Winner::Right
            } else {
                Winner::None
            };

            if let Winner::None = winner {
            } else {
                println!("{}", winner);
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
