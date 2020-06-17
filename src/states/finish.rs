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
use crate::states::{LoadingState, StartState};

#[derive(Default, Debug)]
pub struct FinishState {
    ui: Option<Entity>,
    to_menu: Option<Entity>,
}
///End state from which we can go to the main menu
impl SimpleState for FinishState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.ui = data.world.exec(|mut creator: UiCreator<'_>| {
            Some(creator.create("prefabs/ui/finish.ron", ()))
        });
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(ui) = self.ui {
            data.world.delete_entity(ui);
        }
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Ui(ui) = event {
            if ui.event_type == UiEventType::Click&& ui.target == self.to_menu.unwrap() {
                // return Trans::Replace(Box::new(StartState::default()))
                return Trans::Quit
            }
        }
        Trans::None
    }

    fn shadow_update(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        if self.to_menu == None {
            self.to_menu = _data.world.exec(|finder: UiFinder<'_>| finder.find("to_menu"));
        }
    }
}