use amethyst::{
    ecs::{Component, DenseVecStorage},
};


#[derive(Component, Default)]
#[storage(DenseVecStorage)]
///An component that marks a player
pub struct Player {
    pub hp: u8,
    pub coins: u8,
}

impl Player {
    pub(crate) fn new() -> Player {
        Player { hp: 100, coins: 0 }
    }

}