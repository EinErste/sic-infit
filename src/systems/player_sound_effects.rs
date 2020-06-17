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
use crate::audio::{play_coin_sound, Sounds, play_damage_sound};
use std::ops::Deref;
use amethyst_audio::AudioSink;
use amethyst::prelude::WorldExt;

///This system controls the camera and ties it to a character at al
pub struct PlayerSoundSystem {
    reader_id: ReaderId<SoundEffect>,
    DJ: AudioSink
}

impl PlayerSoundSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let reader_id = world.fetch_mut::<EventChannel<SoundEffect>>().register_reader();
        Self { reader_id, DJ: AudioSink::new(&world.read_resource::<Output>()) }
    }
}

pub enum  SoundEffect {
    Running,
    Idle
}

impl<'s> System<'s> for PlayerSoundSystem {
    type SystemData = (
        Read<'s, EventChannel<SoundEffect>>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
    );

    fn run(&mut self, (soundChannel, storage, sounds): Self::SystemData) {
        for event in soundChannel.read(&mut self.reader_id) {
            match event {
                SoundEffect::Running => {
                    self.DJ.set_volume(1.);
                    // self.DJ.play();
                    if self.DJ.empty(){
                        if let Some(handle) = &sounds.steps_sfx {
                            if let Some(sound) = storage.get(handle) {
                                self.DJ.append(sound);
                            }
                        }
                    }
                }
                SoundEffect::Idle => {
                    self.DJ.set_volume(0.);
                    //self.DJ.pause();
                }
            }
        }
    }
}