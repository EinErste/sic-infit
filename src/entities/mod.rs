mod background;
mod npc;
mod camera;
mod utils;
pub use self::{
    background::{load_world_forest},
    npc::{load_player, load_enemy},
    camera::init_camera,
    utils::AdjustToDistance
};