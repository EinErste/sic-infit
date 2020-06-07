#[allow(unused_imports)]
use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
    ui::{Anchor, TtfFormat, UiText, UiFinder, UiTransform, UiCreator, UiEventType, UiWidget},
    ecs::prelude::{Entity, ResourceId},
};
use crate::states::FinishState;

#[derive(Default, Debug)]
pub struct PauseState {
    ui: Option<Entity>,
    b1: Option<Entity>,
    b2: Option<Entity>,
    b3: Option<Entity>,
}

impl SimpleState for PauseState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.init_ui(data.world);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // data.world.delete_entity(self.label.unwrap()).expect("Failed to delete entity. Was it already removed?");
        if let Some(x) = self.ui {
            data.world.delete_entity(x);
        }
    }


    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Ui(ui) = event {
            if ui.event_type == UiEventType::Click {
                if ui.target == self.b1.unwrap() {
                    return Trans::Pop;
                } else if ui.target == self.b2.unwrap() {
                    //TODO add here transition to settings
                } else if ui.target == self.b3.unwrap() {
                    return Trans::Replace(Box::new(FinishState::default()));
                }
            }
        }
        Trans::None
    }

    fn shadow_update(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        if self.b1 == None {
            self.b1 = _data.world.exec(|finder: UiFinder<'_>| finder.find("button_1"));
        }
        if self.b2 == None {
            self.b2 = _data.world.exec(|finder: UiFinder<'_>| finder.find("button_2"));
        }
        if self.b3 == None {
            self.b3 = _data.world.exec(|finder: UiFinder<'_>| finder.find("button_3"));
        }
    }
}

impl PauseState {
    fn init_ui(&mut self, world: &mut World) {
        self.ui = world.exec(|mut creator: UiCreator<'_>| {
            Some(creator.create("prefabs/ui/pause.ron", ()))
        });
    }
}
// self.b1 = world.exec(|finder: UiFinder<'_>| finder.find("button_1"));
// self.b2 = world.exec(|finder: UiFinder<'_>| finder.find("button_2"));
// self.b3 = world.exec(|finder: UiFinder<'_>| finder.find("button_3"));
//
// dbg!(self);


//
// let font = world.read_resource::<Loader>().load(
//     "../font/square.ttf",
//     TtfFormat,
//     (),
//     &world.read_resource(),
// );
// let p1_transform = UiTransform::new(
//     "PAUSE".to_string(),
//     Anchor::Middle,
//     Anchor::Middle,
//     0.,
//     0.,
//     3.,
//     640.,
//     300.,
// );
//
// world
//     .create_entity()
//     .with(p1_transform)
//     .with(UiText::new(
//         font.clone(),
//         "PAUSE".to_string(),
//         [0.,0.,0.,1.],
//         100.,
//     ))
//     .build()