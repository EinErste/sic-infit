use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, System, SystemData, WriteStorage, ReadStorage, Read, Entities, Entity},
    ui::UiCreator,
};

use crate::components::{Directions, Direction, PhysicsBodyDescription, NPC, Role};
use amethyst::prelude::{World, WorldExt};

#[derive(SystemDesc)]
///Rotates sptite based on the direction the player is facing
pub struct DialogSystem {
    pub(crate) player: Entity,
    talk_button: Option<Entity>,
}

impl DialogSystem {
    pub(crate) fn new(player: Entity) -> DialogSystem {
        DialogSystem { player, talk_button: None }
    }
}

impl<'s> System<'s> for DialogSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, NPC>,
        ReadStorage<'s, Transform>,
        UiCreator<'s>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, player, transform, mut creator) = data;
        let player_transform = transform.get(self.player).unwrap();
        for (character, char_loc) in (&player, &transform).join() {


            let distance = <DialogSystem>::distance(player_transform, char_loc);
            if character.role == Role::NPC {
                match distance < 100. {
                    true => if self.talk_button == None {
                        self.talk_button = Some(creator.create("prefabs/ui/interaction.ron", ()));
                    }
                    false => if let Some(button) = self.talk_button {
                        entities.delete(button);
                        self.talk_button = None;
                    }
                }
            }
        }
    }
}

impl DialogSystem {
    fn distance(player_transform: &Transform, char_loc: &Transform) -> f32 {
        let x = (char_loc.translation().x - player_transform.translation().x).abs();
        let y = (char_loc.translation().y - player_transform.translation().y).abs();
        (x * x + y * y).sqrt()
    }
}
