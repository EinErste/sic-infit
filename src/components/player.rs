use amethyst::{
    ecs::{Component, DenseVecStorage},
};

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
///An empty component that marks a player and ties it in to the control system from keyboard
pub struct NPC {
    pub(crate) role: Role
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Role {
    ENEMY,
    NPC(&'static str),
}

impl NPC {
    pub(crate) fn new_NPC(dialog: &'static str) -> NPC {
        NPC { role: Role::NPC(dialog)}
    }
}