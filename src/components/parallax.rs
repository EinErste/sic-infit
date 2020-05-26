use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage}
};
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Parallax {
    pub velocity_ratio: Vector2<f32>,
}

impl Parallax{
    pub fn new(x: f32,y: f32)-> Self {
        Parallax{
            velocity_ratio: Vector2::new(x,y)
        }
    }
}

impl Default for Parallax{
    fn default() -> Self {
        Parallax::new(0.,0.)
    }
}

