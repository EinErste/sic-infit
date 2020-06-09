//! Mod that contains all of the entity descriptions
mod background;
mod npc;
mod camera;
mod utils;
pub use self::{
    background::{load_forest, load_intro},
    npc::{load_player, load_lion},
    camera::init_camera,
    utils::AdjustToDistance
};