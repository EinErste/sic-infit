use amethyst::{
    core::Time,
    derive::SystemDesc,
    ecs::{Join, System, SystemData, WriteStorage, Read},
    renderer::SpriteRender,
};
use crate::components::SimpleAnimation;

#[derive(SystemDesc)]
///System that loops through the sprites in a fixed amount of time enabling the animations such as
/// running
pub struct SimpleAnimationSystem {}

impl<'s> System<'s> for SimpleAnimationSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, SimpleAnimation>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut sprite_renders, mut animations, time): Self::SystemData) {
        for (sprite_render, anim) in (&mut sprite_renders, &mut animations).join() {
            let (start, end, time_per_frame) = anim.states[anim.current_state];
            let total = end - start;
            anim.time_elapsed += time.delta_seconds();

            let frame_count = ((anim.time_elapsed / time_per_frame) as usize % total) + start;
            if frame_count != sprite_render.sprite_number {
                sprite_render.sprite_number = frame_count;
            }
        }
    }
}