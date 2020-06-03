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
};
use crate::{
    systems::{CameraSystem, CharacterSystem, DirectionSystem, SimpleAnimationSystem},
    states::PauseState,

};
use log::{info};

pub struct GameplayState<'a, 'b> {
    pub dispatcher: Option<Dispatcher<'a, 'b>>,
    pub player: Entity,
    pub camera: Entity
}

impl<'a, 'b> SimpleState for GameplayState<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let mut dispatcher = DispatcherBuilder::new()
            .with(DirectionSystem{}, "direction_system", &[])
            .with(CameraSystem { character: self.player, camera: self.camera }, "camera_system", &[])
            .with(CharacterSystem::new(self.player),"character_system", &[] )
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
