use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Entities, System, SystemData, WriteStorage, Read, Write, World, ReaderId, Join},
    shrev::EventChannel,
    ui::UiText,
};

use crate::entities::{CoinSign, HeartsSign};
use crate::components::Player;
use crate::systems::PlayerSystem;

///This system controls the camera and ties it to a character at al
pub struct HealthSystem {
    reader_id: ReaderId<HpEvent>,
}

pub enum HpEvent {
    HpGained,
    HpLost,
}

impl HealthSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let reader_id = world.fetch_mut::<EventChannel<HpEvent>>().register_reader();
        Self { reader_id }
    }
}

impl<'s> System<'s> for HealthSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, EventChannel<HpEvent>>,
        WriteStorage<'s, Player>,
        Write<'s, HeartsSign>,
        WriteStorage<'s, UiText>,
    );

    fn run(&mut self, (mut entities, hpEvents, mut players, mut heartsSign, mut uiText): Self::SystemData) {
        for hpEvent in hpEvents.read(&mut self.reader_id) {
            for (player, _) in (&mut players, &entities).join() {
                match hpEvent {
                    HpEvent::HpGained => {
                        player.hp += 1;
                    }
                    HpEvent::HpLost => {
                        player.hp -= 1;
                        if player.hp == 0 {
                            dbg!("ALARM LOW HP");
                        }
                    }
                }

                if let Some(text) = uiText.get_mut(heartsSign.0.unwrap()) {
                    text.text = player.hp.to_string();
                }
            }
        }
    }
}