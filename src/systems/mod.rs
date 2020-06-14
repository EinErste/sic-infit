//! Mod for storing all of the systems in one place
mod direction;
mod camera;
mod player;
mod physics;
mod animation;
mod ui;
mod utils;
mod coin_pickup;

pub use self::{
    camera::CameraSystem,
    player::PlayerSystem,
    physics::PhysicsSystem,
    direction::DirectionSystem,
    animation::SimpleAnimationSystem,
    ui::UiEventHandlerSystemDesc,
    utils::CurrentState,
    coin_pickup::{CoinPicked, CoinPickupSystem}
};