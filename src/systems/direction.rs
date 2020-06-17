use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, System, SystemData, WriteStorage, ReadStorage},
};

use crate::components::{Directions, Direction, PhysicsBodyDescription};

#[derive(SystemDesc)]
///Rotates sptite based on the direction the player is facing
pub struct DirectionSystem {}

impl<'s> System<'s> for DirectionSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Direction>,
        ReadStorage<'s, PhysicsBodyDescription>
    );

    fn run(&mut self, (mut transforms, mut dirs, descriptions): Self::SystemData) {
        for (desc, dir) in (&descriptions, &mut dirs,).join() {
            if desc.velocity_direction().x < 0. {
                dir.dir = Directions::Left;
            } else if desc.velocity_direction().x > 0.{
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
