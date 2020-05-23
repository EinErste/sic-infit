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
        Self::new()
    }
}

impl Motion {
    pub fn new() -> Self {
        Motion {
            velocity: Vector2::new(0., 0.),
        }
    }

    pub fn update_velocity(
        &mut self,
        acceleration: Vector2<f32>,
    ) {
        self.velocity.x += acceleration.x;
        self.velocity.y += acceleration.y;
    }
}