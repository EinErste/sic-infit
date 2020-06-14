//! Mod that contains all of the entity descriptions
mod background;
mod npc;
mod camera;
mod utils;
pub use self::{
    background::{load_forest},
    npc::{load_player, load_lion, load_npc, load_coins, load_interact_button, CoinSign, InteractButton},
    camera::init_camera,
    utils::AdjustToDistance
};