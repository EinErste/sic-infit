mod background;
mod npc;
mod camera;
pub use self::{
    background::{load_forest, load_intro},
    npc::load_player,
    camera::init_camera
};