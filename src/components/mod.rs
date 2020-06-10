//! Mod that contains all of the components
mod physics;
mod direction;
mod animation;
mod player;
pub use {
    direction::{Direction, Directions},
    animation::{SimpleAnimation,StateAnimation},
    player::{NPC, Role},
    physics::{PhysicsBodyDescription,CollisionGroupType}
};