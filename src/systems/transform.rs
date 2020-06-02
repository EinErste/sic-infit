use amethyst::{
    ecs::{Join, ReadStorage, System, SystemData, WriteStorage,Entity},
    derive::SystemDesc,
    core::Transform,
};

use crate::components::{Motion, Parallax, Directions, Direction};

#[derive(SystemDesc)]
pub struct TransformSystem {
    character: Entity,
}

impl TransformSystem {
    pub(crate) fn new(character: Entity) -> Self {
        TransformSystem {
            character,
        }
    }
}

impl<'s> System<'s> for TransformSystem {
    type SystemData = (
        ReadStorage<'s, Parallax>,
        ReadStorage<'s, Motion>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Direction>,
    );

    fn run(&mut self, (parallaxes, motions, mut transforms, mut dirs): Self::SystemData) {
        for (motion, dir) in (&motions, &mut dirs,).join() {
            if motion.velocity.x < 0. {
                dir.dir = Directions::Left;
            } else if motion.velocity.x > 0.{
                dir.dir = Directions::Right;
            }
        }

        let char_motion = motions.get(self.character).unwrap();
        let x = char_motion.velocity.x;
        let y = char_motion.velocity.y;
        for (parallax, transform) in (&parallaxes, &mut transforms).join() {
            transform.prepend_translation_x(parallax.velocity_ratio.x*x);
            //transform.prepend_translation_y(y);
        }
    }
}