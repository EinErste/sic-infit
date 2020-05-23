use amethyst::{
    derive::SystemDesc,
    ecs::{Entity, Read, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    core::math::Vector2
};

use crate::components::Motion;

///This system controls the character control
#[derive(SystemDesc)]
pub struct CharacterSystem {
    pub(crate) character: Entity,
}

impl<'s> System<'s> for CharacterSystem {
    type SystemData = (
        WriteStorage<'s, Motion>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut motions, input): Self::SystemData) {
        let char_transform = motions.get_mut(self.character).unwrap();

        const Y_SCALE: f32 = 0.1;
        const X_SCALE: f32 = 0.1;

        if let Some(x) = input.axis_value("x-axis") {
            char_transform.update_velocity(
                Vector2::new(x * X_SCALE, 0.)
            )
        }

        if let Some(y) = input.axis_value("y-axis") {
            char_transform.update_velocity(
                Vector2::new(0., y * Y_SCALE)
            )
        }
    }
}