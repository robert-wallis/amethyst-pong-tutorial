use crate::components::{Ball, Velocity};
use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

pub struct BallMoveSystem;

impl<'s> System<'s> for BallMoveSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (balls, velocities, mut locals, time): Self::SystemData) {
        for (_ball, velocity, local) in (&balls, &velocities, &mut locals).join() {
            let velocity = velocity.0;
            local.translate_x(velocity.x * time.delta_seconds());
            local.translate_y(velocity.y * time.delta_seconds());
        }
    }
}
