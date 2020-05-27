use amethyst::{
    assets::{ProgressCounter},
    prelude::{GameData, SimpleState, SimpleTrans, StateData, Trans, WorldExt},
    ui::UiCreator,
};
use crate::components::{Motion, Direction, SimpleAnimation, Parallax};
use crate::resources::{load_assets, AssetType};
use crate::entities::{load_background_forest, load_character, init_camera, load_intro};
use amethyst::prelude::World;
use crate::states::GameplayState;
use std::{thread, time};

#[derive(Default)]
pub struct LoadingState {
    progress_counter: Option<ProgressCounter>,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
        world.register::<Motion>();
        world.register::<Direction>();
        world.register::<SimpleAnimation>();
        world.register::<Parallax>();
        self.progress_counter = Some(load_assets(&mut world,vec![
            AssetType::BackgroundForest,
            AssetType::Character,
            AssetType::Intro
        ]));

    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {

        if let Some(ref progress_counter) = self.progress_counter {
            if progress_counter.is_complete() {
                println!("end");
                let mut world: &mut World = data.world;
                let camera = init_camera(world);
                let intro = load_intro(&mut world);
                load_background_forest(&mut world);
                let character = load_character(&mut world);
                world.delete_entity(intro).unwrap();
                return Trans::Switch(Box::new(GameplayState{dispatcher: None, character, camera}));
            }else{
                println!("start");
            }
        }
        Trans::None
    }
}