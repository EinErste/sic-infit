#[allow(unused_imports)]
use amethyst::{
    assets::{AssetStorage, Loader,ProgressCounter,Handle},
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    window::ScreenDimensions,
    ecs::{
        prelude::{Dispatcher, DispatcherBuilder},
        Entity,
    },
};
use crate::{
    systems::{CameraSystem, CharacterSystem, MotionSystem, DirectionSystem, SimpleAnimationSystem,ParallaxSystem},
    states::PauseState,
    components::{Motion, Direction, SimpleAnimation,Parallax},
    resources::{load_assets,AssetType},
    entities::{load_background_forest,load_character}
};
use log::{info};
use crate::entities::init_camera;

#[derive(Default)]
pub struct GameplayState<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> SimpleState for GameplayState<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
        world.register::<Motion>();
        world.register::<Direction>();
        world.register::<SimpleAnimation>();
        world.register::<Parallax>();

        let camera = init_camera(world);

        load_assets(&mut world,vec![
                AssetType::BackgroundForest,
                AssetType::Character
            ]);
        load_background_forest(&mut world);
        let character = load_character(&mut world);

        let mut dispatcher = DispatcherBuilder::new()
            .with(MotionSystem{}, "motion_system", &[])
            .with(DirectionSystem{}, "direction_system", &[])
            .with(CameraSystem { character, camera }, "camera_system", &[])
            .with(CharacterSystem::new(character),"character_system", &[] )
            .with(ParallaxSystem::new(character),"parallax_system", &[] )
            .with(SimpleAnimationSystem{},"animation_system", &[] )
            .build();
        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);
    }

    fn handle_event(&mut self, mut _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
            if is_key_down(&event, VirtualKeyCode::P) {
                return Trans::Push(Box::new(PauseState::default()));
            }
            if let Some(_event) = get_key(&event) {
                //info!("handling key event: {:?}", event);
            }
        }
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }
        Trans::None
    }
}
