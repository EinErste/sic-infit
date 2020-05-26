#[allow(unused_imports)]
use amethyst::{
    assets::{AssetStorage, Loader,ProgressCounter},
    core::transform::Transform,
    core::math::{Vector2, Vector3, Matrix4},
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
    ui::{Anchor, TtfFormat, UiText, UiTransform, UiCreator},
    ecs::{
        prelude::{Dispatcher, DispatcherBuilder},
        Entity,
    },
};
use crate::{
    systems::{CameraSystem, CharacterSystem, MotionSystem, DirectionSystem, SimpleAnimationSystem},
    states::PauseState,
    components::{Motion, Directions, Direction, SimpleAnimation, StateAnimation},
};
use log::{info};
use enum_map::{enum_map};
use std::iter::Iterator;

#[derive(Default)]
pub struct GameplayState<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> SimpleState for GameplayState<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Motion>();
        world.register::<Direction>();
        world.register::<SimpleAnimation>();

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        let camera = init_camera(world, &dimensions);
        let character = init_sprites(world, &dimensions);

        let mut dispatcher = DispatcherBuilder::new()
            .with(MotionSystem{}, "motion_system", &[])
            .with(DirectionSystem{}, "direction_system", &[])
            .with(CameraSystem { character, camera }, "camera_system", &[])
            .with(CharacterSystem::new(character),"character_system", &[] )
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
            if let Some(event) = get_key(&event) {
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

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) -> Entity {


    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() / 6 as f32, dimensions.height() / 6 as f32, 300.);

    world
        .create_entity()
        .with(Camera::standard_3d(dimensions.width() / 3 as f32, dimensions.height() / 3 as f32))
        .with(transform)
        .named("camera")
        .build()
}

fn load_sprites(world: &mut World) -> Vec<SpriteRender> {
    let (texture_handle, char_texture_handle) = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        (
            loader.load(
                "textures/background_forest.png",
                ImageFormat::default(),
                (),
                &texture_storage,
            ),
            loader.load(
                "textures/character.png",
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        )
    };

    let (sheet_handle, char_sheet_handle) = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        (
            loader.load(
                "prefabs/background_forest.ron",
                SpriteSheetFormat(texture_handle),
                (),
                &sheet_storage,
            ),
            loader.load(
                "prefabs/character.ron",
                SpriteSheetFormat(char_texture_handle),
                (),
                &sheet_storage,
            )
        )
    };

    vec![
        SpriteRender {
            sprite_sheet: sheet_handle.clone(),
            sprite_number: 0,
        },
        SpriteRender {
            sprite_sheet: char_sheet_handle.clone(),
            sprite_number: 0,
        }
    ]
}

fn init_sprites(world: &mut World, _dimensions: &ScreenDimensions) -> Entity {
    let mut sprites = load_sprites(world);
    let dist:Vec<f32> = vec![-50.,-30.,-15.,-7.,-5.0,-3.,0.,1.];
    for (i,j) in dist.iter().enumerate(){
        sprites[0].sprite_number = i;
        let sprite = &sprites[0];
        let transform =
            Transform::default().set_translation_xyz(960., 180., *j).to_owned();
        world
            .create_entity()
            .with(sprite.clone())
            .with(transform)
            .build();
    }

    let c = &sprites[1];
    let transform =
        Transform::default().set_translation_xyz(320., 100., 1.).to_owned();
    world
        .create_entity()
        .with(c.clone())
        .with(transform)
        .with(Motion::new())
        .with(Direction{dir: Directions::Right})
        .named("character")
        .with(SimpleAnimation::new(StateAnimation::Idle,enum_map!(
            StateAnimation::Run => (2,10,0.1),
            StateAnimation::Idle => (0,2,0.8),
            _ => (0,1,0.1)
        )))
        .build()
}