use amethyst::{
    assets::{ProgressCounter},
    prelude::{GameData, SimpleState, SimpleTrans, StateData, Trans, WorldExt},
    ui::UiCreator,
};
use crate::components::{Direction, SimpleAnimation, Player, PhysicsBodyDescription};
use crate::resources::{load_assets, AssetType};
use crate::entities::{load_player, init_camera, load_forest, load_lion};
use amethyst::prelude::World;
use crate::states::GameplayState;
use amethyst_physics::PhysicsTime;

#[derive(Default)]
///State used to avoid displaying an empty screen while all of the resources are being loaded
pub struct LoadingState {
    progress_counter: Option<ProgressCounter>,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
        world.register::<PhysicsBodyDescription>();
        world.register::<Direction>();
        world.register::<SimpleAnimation>();
        world.register::<Player>();
        self.progress_counter = Some(load_assets(&mut world,vec![
            AssetType::BackgroundForest,
            AssetType::Character,
            AssetType::Platforms,
            AssetType::Collectables
        ]));

    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {

        if let Some(ref progress_counter) = self.progress_counter {
            if progress_counter.is_complete() {
                dbg!("loading done");
                let mut world: &mut World = data.world;
                //Pause physics
                world.fetch_mut::<PhysicsTime>().set_frames_per_seconds(0);

                let camera = init_camera(world);
                load_forest(&mut world);
                let player = load_player(&mut world);
                load_lion(&mut world);
                return Trans::Switch(Box::new(GameplayState{dispatcher: None, player, camera}));
            } else {
                dbg!("loading in progress");
            }
        }
        Trans::None
    }
}