use amethyst::{core::transform::TransformBundle, prelude::*, renderer::types::DefaultBackend,
               renderer::RenderFlat2D,
               renderer::RenderToWindow,
               renderer::RenderingBundle,
               utils::application_root_dir, input::{InputBundle, StringBindings}, ui::{RenderUi, UiBundle}, ecs::prelude::ReadExpect, StateMachine};
use crate::states::LoadingState;
use amethyst_physics::{PhysicsBundle};
use amethyst_nphysics::NPhysicsBackend;
use crate::systems::{PhysicsSystem, PlayerSystem, DirectionSystem};

mod states;
mod systems;
mod components;
mod resources;
mod entities;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("assets");
    let display_config = resources.join("display_config.ron");


    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle = InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(PhysicsBundle::<f32,NPhysicsBackend>::new()
            .with_frames_per_seconds(60)
            .with_max_sub_steps(8)
            .with_pre_physics(PhysicsSystem::default(), String::from("physics_system"),vec![])
            .with_pre_physics(PlayerSystem::default(),String::from("player_system"), vec![String::from("physics_system")])
            //.with_in_physics(DirectionSystem{},String::from("direction_system"), vec![])
        )?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        //.with_clear([0.34, 0.36, 0.52, 1.0]),
                        //.with_clear([	0.788,0.914,0.965,1.00]),
                        //.with_clear([	0.98,0.957,0.875,1.00]),
                        .with_clear([	0.98,0.965,0.875,1.00]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?;

    let mut game = Application::new(resources, LoadingState::default(), game_data)?;
    game.run();
    Ok(())
}
