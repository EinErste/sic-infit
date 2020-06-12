mod background;
mod npc;
mod camera;
mod utils;
pub use self::{
    background::{load_forest},
    npc::{load_player, load_lion},
    camera::init_camera,
    utils::AdjustToDistance
};