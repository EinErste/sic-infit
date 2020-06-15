//! Mod that contains all of the entity descriptions
mod background;
mod npc;
mod camera;
mod utils;
pub use self::{
    background::{load_world_forest},
    npc::{load_player, load_lion, load_npc, load_coins, load_ui_imgs, load_hearts, load_interact_button, CoinSign, InteractButton, HeartsSign},
    camera::init_camera,
    utils::AdjustToDistance
};