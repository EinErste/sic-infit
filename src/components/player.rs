use amethyst::{
    ecs::{Component, DenseVecStorage},
};


#[derive(Component, Default)]
#[storage(DenseVecStorage)]
///An empty component that marks a player and ties it in to the control system from keyboard
pub struct Player {
    pub hp: u8,
    pub coins: u8,
}

impl Player {
    pub(crate) fn new() -> Player {
        Player { hp: 3, coins: 0 }
    }
}