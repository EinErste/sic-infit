use amethyst::{
    ecs::{Component, DenseVecStorage},
};

#[derive(Component, Default, Debug)]
#[storage(DenseVecStorage)]
///An empty component that marks a player and ties it in to the control system from keyboard
pub struct NPC {
    pub(crate) role: Role
}
#[derive(Debug, PartialOrd, PartialEq)]
pub enum Role {
    ENEMY, NPC
}

impl Default for Role{
    fn default() -> Self {
        Role::NPC
    }
}