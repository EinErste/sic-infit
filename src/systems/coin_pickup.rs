use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Entities, System, SystemData, WriteStorage, Read, Write, World, ReaderId, Join, ReadExpect},
    shrev::EventChannel,
    ui::UiText,
    assets::AssetStorage,
    audio::{Source, output::Output}
};

use crate::entities::CoinSign;
use crate::components::Player;
use crate::audio::{play_coin_sound, Sounds};
use std::ops::Deref;

///This system controls the camera and ties it to a character at al
pub struct CoinPickupSystem {
    reader_id: ReaderId<CoinPicked>,
}

impl CoinPickupSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let reader_id = world.fetch_mut::<EventChannel<CoinPicked>>().register_reader();
        Self { reader_id }
    }
}

pub struct CoinPicked();

impl<'s> System<'s> for CoinPickupSystem {
    type SystemData = (
        Read<'s, EventChannel<CoinPicked>>,
        Write<'s, CoinSign>,
        WriteStorage<'s, UiText>,
        WriteStorage<'s, Player>,
        Entities<'s>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(&mut self, (coinChannel, coinSign, mut uiText, mut players, mut entities, storage, sounds, audio_output): Self::SystemData) {
        for (player, _) in (&mut players, &entities).join() {
            for coinEvent in coinChannel.read(&mut self.reader_id) {
                player.coins +=1;
                play_coin_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));

                let entity = coinSign.0.unwrap();
                if let Some(text) = uiText.get_mut(entity) {
                    text.text = player.coins.to_string();
                }
            }
        }
    }
}