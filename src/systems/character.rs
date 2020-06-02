use amethyst::{
    derive::SystemDesc,
    ecs::{Entity, Read, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components::{Motion, SimpleAnimation, StateAnimation};

///This system controls the character control
#[derive(SystemDesc)]
pub struct CharacterSystem {
    character: Entity,
}

impl CharacterSystem {
    pub(crate) fn new(character: Entity) -> Self {
        CharacterSystem {
            character,
        }
    }
}

#[allow(dead_code)]
impl<'s> System<'s> for CharacterSystem {
    type SystemData = (
        WriteStorage<'s, Motion>,
        WriteStorage<'s, SimpleAnimation>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut motions, mut animations, input): Self::SystemData) {
        let char_motion = motions.get_mut(self.character).unwrap();
        let char_anim = animations.get_mut(self.character).unwrap();
        let speed = 1.7;

        if let Some(x) = input.axis_value("x-axis") {
            if x == 0. {
                char_motion.update_velocity((0., 0.));

                char_anim.change_state(StateAnimation::Idle);
            } else {
                let speed = speed * x;
                char_motion.update_velocity((speed, 0.));

                char_anim.change_state(StateAnimation::Run);
            }
        }

        if let Some(jump) = input.action_is_down("Jump"){
            if jump {
                char_motion.update_velocity((char_motion.velocity.x, 1.));
            } else {
                char_motion.update_velocity((char_motion.velocity.x, 0.));
            }
        }
    }
}