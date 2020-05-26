use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, System, SystemData, WriteStorage, ReadStorage},
};

use crate::components::{Directions, Direction};

#[derive(SystemDesc)]
pub struct DirectionSystem {}

impl<'s> System<'s> for DirectionSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Direction>
    );

    fn run(&mut self, (mut transforms, dirs): Self::SystemData) {
        for (transform, dir) in (&mut transforms, &dirs).join() {
            match dir.dir {
                Directions::Right => transform.set_rotation_y_axis(0.),
                Directions::Left => transform.set_rotation_y_axis(std::f32::consts::PI),
            };
        }
    }
}
