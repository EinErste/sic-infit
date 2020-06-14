use amethyst::{
    ecs::{Component, DenseVecStorage}
};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Direction{
    pub(crate) dir: Directions
}

pub enum Directions {
    Left, Right
}