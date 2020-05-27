mod background;
mod npc;
mod camera;
pub use self::{
    background::load_background_forest,
    npc::load_character,
    camera::init_camera
};