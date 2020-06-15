//! Mod that contains all of the entity descriptions
mod background;
mod npc;
mod camera;
mod utils;
pub use self::{
    background::{load_world_forest},
    npc::{load_ui, load_player, load_enemy, load_npc, CoinSign, InteractButton, HeartsSign},
    camera::init_camera,
    utils::AdjustToDistance
};