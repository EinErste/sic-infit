use amethyst::{
    ecs::{Component, DenseVecStorage}
};

#[derive(Component)]
#[storage(DenseVecStorage)]
///Component responsible for direction. Rotates sprite depending on where the entity is headed
pub struct Direction{
    pub(crate) dir: Directions
}

pub enum Directions {
    Left, Right
}