use amethyst::{
    derive::SystemDesc,
    ecs::{Entity, Read, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    core::math::Vector2,
};

use crate::components::Motion;

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
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut motions, input): Self::SystemData) {
        let char = motions.get_mut(self.character).unwrap();

        let speed = 4.;

        if let Some(x) = input.axis_value("x-axis") {
            if x == 0. {
               char.update_velocity((0.,0.));
            } else {
                let speed = speed * x;
                char.update_velocity(
                    (speed , 0.)
                );
            }
        }
    }
}