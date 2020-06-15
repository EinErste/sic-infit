use amethyst::{
    assets::Loader,
    audio::{OggFormat, SourceHandle, Source, output::Output},
    ecs::{World, WorldExt},
    assets::AssetStorage,
};

const COIN_SOUND: &str = "audio/coin.ogg";
const DAMAGE_SOUND: &str = "audio/damage.ogg";
const SCORE_SOUND: &str = "audio/score.ogg";

#[derive(Clone)]
pub struct Sounds {
    pub score_sfx: SourceHandle,
    pub coin_sfx: SourceHandle,
    pub damage_sfx: SourceHandle,
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
            coin_sfx: load_audio_track(&loader, &world, COIN_SOUND),
            damage_sfx: load_audio_track(&loader, &world, DAMAGE_SOUND),
            score_sfx: load_audio_track(&loader, &world, SCORE_SOUND),
        };

        sound
    };

    // Add sound effects to the world. We have to do this in another scope because
    // world won't let us insert new resources as long as `Loader` is borrowed.
    world.insert(sound_effects);
}

pub fn play_coin_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.coin_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}

pub fn play_damage_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.damage_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}