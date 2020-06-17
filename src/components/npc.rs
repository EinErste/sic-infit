use amethyst::{
    ecs::{Component, DenseVecStorage},
};


#[derive(Component, Default)]
#[storage(DenseVecStorage)]
///An empty component that marks a player and ties it in to the control system from keyboard
pub struct NPC {
    pub line: String
}

impl NPC {
    pub(crate) fn new(line: &str) -> NPC {
        NPC { line: line.to_string() }
    }
}