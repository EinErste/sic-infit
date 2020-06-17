use amethyst::{
    assets::Loader,
    audio::{OggFormat, SourceHandle, Source, output::Output},
    ecs::{World, WorldExt},
    assets::AssetStorage,
};

const COIN_SOUND: &str = "audio/coin.ogg";
const DAMAGE_SOUND: &str = "audio/damage.ogg";
const AMBIENT_SOUND: &str = "audio/ambient.ogg";
const STEPS_SOUND: &str = "audio/steps.ogg";
const MENU_SOUND: &str = "audio/ambient.ogg";

#[derive(Clone, Default)]
pub struct Sounds {
    pub ambient_sfx: Option<SourceHandle>,
    pub coin_sfx: Option<SourceHandle>,
    pub damage_sfx: Option<SourceHandle>,
    pub steps_sfx: Option<SourceHandle>,
}

/// Loads an ogg audio track.
fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

/// Initialise audio in the world. This will eventually include
/// the background tracks as well as the sound effects, but for now
/// we'll just work on sound effects.
pub fn initialise_audio(world: &mut World) {
    let sound_effects = {
        let loader = world.read_resource::<Loader>();

        let sound = Sounds {
            coin_sfx: Some(load_audio_track(&loader, &world, COIN_SOUND)),
            damage_sfx: Some(load_audio_track(&loader, &world, DAMAGE_SOUND)),
            ambient_sfx: Some(load_audio_track(&loader, &world, AMBIENT_SOUND)),
            steps_sfx: Some(load_audio_track(&loader, &world, STEPS_SOUND)),
        };

        sound
    };

    // Add sound effects to the world. We have to do this in another scope because
    // world won't let us insert new resources as long as `Loader` is borrowed.
    world.insert(sound_effects);
}

pub fn play_coin_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&(sounds.coin_sfx.clone().unwrap())) {
            output.play_once(sound, 0.03);
        }
    }
}

pub fn play_damage_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&(sounds.damage_sfx.clone().unwrap())) {
            output.play_once(sound, 0.15);
        }
    }
}
