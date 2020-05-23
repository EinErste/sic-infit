use amethyst::{
    ecs::{Join, Entity, ReadStorage, System, SystemData, WriteStorage},
    derive::SystemDesc,
    core::Transform
};

use crate::components::Motion;

///This system controls the character control
#[derive(SystemDesc)]
pub struct MotionSystem{}

impl<'s> System<'s> for MotionSystem {
    type SystemData = (
        ReadStorage<'s, Motion>,
        WriteStorage<'s, Transform>
    );

    fn run(&mut self, (motions, mut transforms): Self::SystemData) {

        for (motion, transform) in (&motions, &mut transforms).join() {
            transform.prepend_translation_x(motion.velocity.x);
            transform.prepend_translation_y(motion.velocity.y);
        }
    }
}