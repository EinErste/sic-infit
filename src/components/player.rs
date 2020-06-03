use amethyst::{
    ecs::{Component, DenseVecStorage},
};


#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct Player {
}