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
    ecs::prelude::{Dispatcher, DispatcherBuilder},
};
use log::info;
use crate::systems::CharacterSystem;
use crate::states::PauseState;

#[derive(Default)]
pub struct GameplayState<'a, 'b>{
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> SimpleState for GameplayState<'a, 'b> {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        init_camera(world, &dimensions);

        let sprites = load_sprites(world);
        init_sprites(world, &sprites, &dimensions);

        init_ui(world);

        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(CharacterSystem, "character_system", &[]);

        // Build and setup the `Dispatcher`.
        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);
    }

    fn handle_event(&mut self, mut _data: StateData<'_, GameData<'_, '_>>, event: StateEvent, ) -> SimpleTrans {
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

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {

    //Hardcoded camera size
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width()/6 as f32, dimensions.height()/6 as f32, 300.);

    world
        .create_entity()
        .with(Camera::standard_3d(dimensions.width()/3 as f32, dimensions.height()/3 as f32))
        .with(transform)
        .build();

}

fn load_sprites(world: &mut World) -> Vec<SpriteRender> {

    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/spritesheet.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    (0..1)
        .map(|i| SpriteRender {
            sprite_sheet: sheet_handle.clone(),
            sprite_number: i,
        })
        .collect()
}

fn init_sprites(world: &mut World, sprites: &[SpriteRender], dimensions: &ScreenDimensions) {
    let b = &sprites[0];
    let mut transform = Transform::default();
    transform.set_translation_xyz(960 as f32, 180 as f32, 0.);
    world
        .create_entity()
        .with(b.clone())
        .with(transform)
        .build();

    // let mut transform = Transform::default();
    // transform.set_translation_xyz(0 as f32, 180 as f32, -100.).set_scale( Vector3::from_element(1.35));
    // world
    //     .create_entity()
    //     .with(b.clone())
    //     .with(transform)
    //     .build();
}

fn init_ui(world: &mut World) {

}