#[allow(unused_imports)]
use amethyst::{
    core::transform::Transform,
    core::{Named, WithNamed},
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    window::ScreenDimensions,
    ecs::{
        prelude::{Dispatcher, DispatcherBuilder},
        Entity,
    },
    audio::{output::Output, Source},
    assets::AssetStorage
};
use crate::{
    systems::{CameraSystem, PlayerSystem, DirectionSystem, SimpleAnimationSystem},
    states::PauseState,
};
use amethyst_physics::PhysicsTime;
use crate::systems::{CoinPickupSystem, InteractButtonSystem, HealthSystem, PlayerSoundSystem};
use crate::audio::{initialise_audio, Sounds};
use crate::states::FinishState;
use amethyst_audio::AudioSink;
#[derive(PartialEq)]
pub enum GameplayStateTypes{
    Active,
    Inactice
}

pub struct GameplayStateType{
    pub state: GameplayStateTypes
}

impl Default for GameplayStateType{
    fn default() -> Self {
        GameplayStateType{state: GameplayStateTypes::Active}
    }
}

///Main state where all the actual gameplay takes place
pub struct GameplayState<'a, 'b> {
    pub dispatcher: Option<Dispatcher<'a, 'b>>,
    pub player: Entity,
    pub camera: Entity,
    pub DJ: AudioSink
}

impl<'a, 'b> SimpleState for GameplayState<'a, 'b> {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
        world.fetch_mut::<GameplayStateType>().state = GameplayStateTypes::Active;
        world.fetch_mut::<PhysicsTime>().set_frames_per_seconds(60);
        let mut dispatcher = DispatcherBuilder::new()
            .with(DirectionSystem {}, "direction_system", &[])
            .with(CameraSystem { character: self.player, camera: self.camera }, "camera_system", &[])
            .with(SimpleAnimationSystem {}, "animation_system", &[])
            .with(CoinPickupSystem::new(&mut world), "coin_system", &[])
            .with(HealthSystem::new(&mut world), "health_system", &[])
            .with(InteractButtonSystem::new(&mut world), "interact_button_system", &[])
            .with(PlayerSoundSystem::new(&mut world), "player_sound_effect_system", &[])
            .build();
        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);
    }
    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.fetch_mut::<PhysicsTime>().set_frames_per_seconds(0);
    }

    fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.fetch_mut::<PhysicsTime>().set_frames_per_seconds(0);
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.fetch_mut::<PhysicsTime>().set_frames_per_seconds(60);
    }

    fn handle_event(&mut self, mut _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) {
                return Trans::Quit;
            } else if is_key_down(&event, VirtualKeyCode::Escape) || is_key_down(&event, VirtualKeyCode::P) {
                return Trans::Push(Box::new(PauseState::default()));
            }
            if let Some(_event) = get_key(&event) {
                //info!("handling key event: {:?}", event);
            }
        }
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        {
            if let Some(dispatcher) = self.dispatcher.as_mut() {
                dispatcher.dispatch(&data.world);
            }
            let fetched = data.world.try_fetch_mut::<GameplayStateType>();
            if let Some(mut state) = fetched {
                if state.state == GameplayStateTypes::Inactice {
                    //?
                    state.state == GameplayStateTypes::Active;
                    return Trans::Switch(Box::new(FinishState::default()));
                }
            }
        }

        if self.DJ.empty(){
            let storage = data.world.read_resource::<AssetStorage<Source>>();
            let sounds = data.world.read_resource::<Sounds>();

            if let Some(handle) = &sounds.ambient_sfx {
                if let Some(music) = storage.get(handle) {
                    self.DJ.append(music);
                    self.DJ.set_volume(0.1);
                }
            }
        }
        Trans::None
    }
}