use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Entity, System, SystemData, WriteStorage, ReadStorage, Write, Read, World, ReaderId, Join},
    shrev::EventChannel,
    ui::UiText,
};
use crate::components::{NPC, Player};
use crate::entities::InteractButton;

///This system controls the camera and ties it to a character at al
pub struct InteractButtonSystem {
    reader_id: ReaderId<Interact>,
}

impl InteractButtonSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let reader_id = world.fetch_mut::<EventChannel<Interact>>().register_reader();
        Self { reader_id }
    }
}

pub struct Interact();

impl<'s> System<'s> for InteractButtonSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, NPC>,
        ReadStorage<'s, Player>,
        Read<'s, InteractButton>,
        WriteStorage<'s, UiText>,
        Read<'s, EventChannel<Interact>>
    );

    fn run(&mut self, (transforms, npcs, player, button, mut text, interactions): Self::SystemData) {
        for (_, npc_transform) in (&npcs, &transforms).join() {
            for (_, player_transform) in (&player, &transforms).join() {
                let npc = npc_transform.translation();
                let plyr = player_transform.translation();
                let button_component = text.get_mut(button.0.unwrap()).unwrap();
                let x = (npc.x - plyr.x).abs();
                let y = (npc.y - plyr.y).abs();

                if x < 75. && y < 75. {
                    button_component.color = [0., 0., 0., 1.];
                    for interaction in interactions.read(&mut self.reader_id) {
                        println!("Talking");
                    }
                } else {
                    button_component.color = [0., 0., 0., 0.];
                }
            }
        }
    }
}