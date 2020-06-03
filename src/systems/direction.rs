use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, System, SystemData, WriteStorage, ReadStorage},
};

use crate::components::{Directions, Direction, Motion};

#[derive(SystemDesc)]
pub struct DirectionSystem {}

impl<'s> System<'s> for DirectionSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Direction>,
        ReadStorage<'s, Motion>
    );

    fn run(&mut self, (mut transforms, mut dirs, motions): Self::SystemData) {
        for (motion, dir) in (&motions, &mut dirs,).join() {
            if motion.velocity.x < 0. {
                dir.dir = Directions::Left;
            } else if motion.velocity.x > 0.{
                dir.dir = Directions::Right;
            }
        }
        for (transform, dir) in (&mut transforms, &dirs).join() {
            match dir.dir {
                Directions::Right => transform.set_rotation_y_axis(0.),
                Directions::Left => transform.set_rotation_y_axis(std::f32::consts::PI),
            };
        }
    }
}
