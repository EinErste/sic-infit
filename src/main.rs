use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    audio::AudioBundle,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow, RenderSkybox},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    input::{InputBundle, StringBindings},
    ui::{RenderUi, UiBundle},
    ecs::prelude::ReadExpect,
};
use crate::states::{LoadingState, StartState};
use amethyst_physics::{PhysicsBundle, prelude::*};
use amethyst_nphysics::NPhysicsBackend;
use crate::systems::{PhysicsSystem, PlayerSystem, DirectionSystem};
use std::panic;

mod audio;
mod states;
mod systems;
mod components;
mod resources;
mod entities;

///Main function is an entry point for the game
fn main() -> amethyst::Result<()> {


    //TODO (u know)
    //amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("assets");
    let display_config = resources.join("display_config.ron");


    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle = InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;
    //main point where we basically construct the game from all the plugins and systems we have
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(PhysicsBundle::<f32, NPhysicsBackend>::new()
            .with_frames_per_seconds(60)
            .with_max_sub_steps(8)
            .with_pre_physics(PhysicsSystem::default(), String::from("physics_system"), vec![])
            .with_pre_physics(PlayerSystem::default(), String::from("player_system"), vec![String::from("physics_system")])
        )?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        //.with_clear([0.98,0.965,0.875,1.00]),
                        //.with_clear([0.403921,0.701960,0.90234375,1.0])
                        .with_clear([0.18, 0.531960, 0.9023, 1.0])
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_system_desc(systems::UiEventHandlerSystemDesc::default(), "ui_event_handler", &[])
        .with_bundle(input_bundle)?
        .with_bundle(AudioBundle::default())?
        .with_bundle(UiBundle::<StringBindings>::new())?;

    let mut game = Application::new(resources, StartState::default(), game_data)?;
    game.run();

    Ok(())
}
