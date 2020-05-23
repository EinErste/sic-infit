use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage},
};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Motion {
    pub velocity: Vector2<f32>,
}

impl Default for Motion {
    fn default() -> Self {
        Motion { velocity: Vector2::new(0., 0.) }
    }
}

impl Motion {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_velocity(
        &mut self,
        (x, y): (f32, f32),
    ) {
        self.velocity.x = x;
        self.velocity.y = y;
    }
}