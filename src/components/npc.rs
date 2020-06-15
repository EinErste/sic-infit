use amethyst::{
    ecs::{Component, DenseVecStorage},
};


#[derive(Component, Default)]
#[storage(DenseVecStorage)]
///An empty component that marks a player and ties it in to the control system from keyboard
pub struct NPC {
    line: &'static str
}

impl NPC {
    pub(crate) fn new(line: &'static str) -> NPC {
        NPC { line }
    }
}