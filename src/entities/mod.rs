mod background;
mod npc;
mod camera;
pub use self::{
    background::{load_forest, load_intro},
    npc::load_character,
    camera::init_camera
};