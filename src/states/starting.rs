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
use crate::states::LoadingState;
use amethyst_audio::AudioSink;
use amethyst_audio::output::Output;
use crate::audio::initialise_audio;

pub struct DJWrapper {
    dj: AudioSink
}

#[derive(Default, Debug)]
pub struct StartState {
    ui: Option<Entity>,
    b1: Option<Entity>,
    b2: Option<Entity>,
    b3: Option<Entity>,
}

impl SimpleState for StartState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.ui = data.world.exec(|mut creator: UiCreator<'_>| {
            Some(creator.create("prefabs/ui/start.ron", ()))
        });
        let mut world = data.world;
        // let dj = DJWrapper{dj: AudioSink::new(&world.read_resource::<Output>())};
        // world.insert(dj);
        initialise_audio(world);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(ui) = self.ui {
            data.world.delete_entity(ui);
        }
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Ui(ui) = event {
            if ui.event_type == UiEventType::Click {
                if ui.target == self.b1.unwrap() {
                    return Trans::Switch(Box::new(LoadingState::default()));
                } else if ui.target == self.b2.unwrap() {
                    //TODO add here transition to settings
                } else if ui.target == self.b3.unwrap() {
                    return Trans::Quit;
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