use amethyst::{
    ecs::{Join, ReadStorage, System, SystemData, WriteStorage},
    derive::SystemDesc,
    core::Transform,
};

use crate::components::{Motion, Directions, Direction};

///This system controls the character control
#[derive(SystemDesc)]
pub struct MotionSystem {}

impl<'s> System<'s> for MotionSystem {
    type SystemData = (
        ReadStorage<'s, Motion>,
        WriteStorage<'s, Direction>,
        WriteStorage<'s, Transform>
    );

    fn run(&mut self, (motions, mut dirs, mut transforms): Self::SystemData) {
        for (motion, dir, transform) in (&motions, &mut dirs, &mut transforms).join() {
            transform.prepend_translation_x(motion.velocity.x);
            if motion.velocity.x < 0. {
                dir.dir = Directions::Left;
            } else if motion.velocity.x > 0. {
                dir.dir = Directions::Right;
            };

            transform.prepend_translation_y(motion.velocity.y);
        }
    }
}