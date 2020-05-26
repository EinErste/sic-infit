use amethyst::{
    ecs::{Join, ReadStorage, System, SystemData, WriteStorage,Entity},
    derive::SystemDesc,
    core::Transform,
};

use crate::components::{Motion, Parallax};

#[derive(SystemDesc)]
pub struct ParallaxSystem {
    character: Entity,
}

impl ParallaxSystem {
    pub(crate) fn new(character: Entity) -> Self {
        ParallaxSystem {
            character,
        }
    }
}

impl<'s> System<'s> for ParallaxSystem {
    type SystemData = (
        ReadStorage<'s, Parallax>,
        WriteStorage<'s, Motion>
    );

    fn run(&mut self, (parallaxes, mut motions): Self::SystemData) {
        let char_motion = motions.get_mut(self.character).unwrap();
        let x = char_motion.velocity.x;
        let y = char_motion.velocity.y;
        for (parallax, motion) in (&parallaxes, &mut motions).join() {
            motion.velocity.x = parallax.velocity_ratio.x*x;
        }

    }
}