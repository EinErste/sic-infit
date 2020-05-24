#[allow(unused_imports)]
use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    core::math::{Vector2, Vector3, Matrix4},
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
    ecs::{
        prelude::{Dispatcher, DispatcherBuilder},
        Entity,
    },
};
use log::info;
use crate::systems::{CameraSystem, CharacterSystem, MotionSystem, DirectionSystem, SimpleAnimationSystem};
use crate::states::PauseState;
use crate::components::{Motion, Directions, Direction, SimpleAnimation};

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
                info!("handling key event: {:?}", event);
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

    //Hardcoded camera size
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
                "assets/spritesheet.png",
                ImageFormat::default(),
                (),
                &texture_storage,
            ),
            loader.load(
                "assets/character/player_anim_run64.png",
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
                "assets/spritesheet.ron",
                SpriteSheetFormat(texture_handle),
                (),
                &sheet_storage,
            ),
            loader.load(
                "assets/character/player_anim_run64.ron",
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
    let sprites = load_sprites(world);

    let b = &sprites[0];
    let transform =
        Transform::default().set_translation_xyz(960., 180., 0.).to_owned();
    world
        .create_entity()
        .with(b.clone())
        .with(transform)
        .build();

    let c = &sprites[1];
    let transform =
        Transform::default().set_translation_xyz(100., 100., 1.).to_owned();
    world
        .create_entity()
        .with(c.clone())
        .with(transform)
        .with(Motion::new())
        .with(Direction{dir: Directions::Right})
        .named("character")
        .with(SimpleAnimation::new(2,8,0.1))
        .build()
}