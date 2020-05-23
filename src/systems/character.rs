use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::Camera;


#[derive(SystemDesc)]
pub struct CharacterSystem;

impl<'s> System<'s> for CharacterSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms , cameras, input): Self::SystemData) {
        //Camera component to find its entity
        /*
        Now that we have access to the storages of the components we want, we can iterate over them.
        We perform a join operation over the Transform and Camera storages.
        This will iterate over all entities that have both a Camera and Transform attached to them,
        and give us access to the actual components,
        immutable for the Camera and mutable for the Transform.
         */
        for (camera, transform) in (&cameras, &mut transforms).join() {
            let movement =  input.axis_value("move");
            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let scaled_amount = 4 as f32 * mv_amount as f32;
                    transform.prepend_translation_x(scaled_amount);
                }
            }
        }
    }
}