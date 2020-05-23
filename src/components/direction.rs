use amethyst::{
    ecs::Component
};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub enum Direction {
    Left, Right
}

