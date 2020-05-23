#[allow(unused_imports)]
use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
    ecs::prelude::Entity,
};
#[derive(Default)]
pub struct PauseState {
    label: Option<Entity>
}

impl SimpleState for PauseState {



    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.label = Some(init_ui(data.world));
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.delete_entity(self.label.unwrap()).expect("Failed to delete entity. Was it already removed?");
    }


    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::P) {
                return Trans::Pop;
            }
        }
        Trans::None
    }
}

fn init_ui(world: &mut World) -> Entity {
    let font = world.read_resource::<Loader>().load(
        "../font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let p1_transform = UiTransform::new(
        "PAUSE".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.,
        0.,
        3.,
        640.,
        300.,
    );

    world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            "PAUSE".to_string(),
            [0.,0.,0.,1.],
            100.,
        ))
        .build()
}