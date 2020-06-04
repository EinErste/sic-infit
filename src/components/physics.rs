use amethyst::{
    core::math::Vector3,
    ecs::{Component, DenseVecStorage},
};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct PhysicsBodyDescription {
    velocity_direction: Vector3<f32>,
    velocity_max: f32,
    mass: f32
}

impl Default for PhysicsBodyDescription {
    fn default() -> Self {
        PhysicsBodyDescription { velocity_direction: Vector3::new(0., 0., 0.), mass: 1., velocity_max: 10.}
    }
}

impl PhysicsBodyDescription {
    pub fn new(mass: f32, vel_max: f32) -> Self {
        let mut desc = Self::default();
        desc.mass = mass;
        desc.velocity_max = vel_max;
        desc
    }


    pub fn velocity_max(&self) ->  f32{
        self.velocity_max
    }

    pub fn set_velocity_max(&mut self, velocity_max: f32){
        self.velocity_max = velocity_max;
    }

    pub fn mass(&self) ->  f32{
        self.mass
    }

    pub fn set_mass(&mut self, mass: f32){
        self.mass = mass;
    }


    pub fn velocity_direction(&self) ->  Vector3<f32>{
        self.velocity_direction.clone()
    }

    pub fn set_velocity_direction(&mut self, vec: &Vector3<f32>) {
        self.velocity_direction = vec.clone();
    }
    pub fn set_velocity_direction_x(&mut self, x: f32) {
        self.velocity_direction.x = x;
    }
    pub fn set_velocity_direction_y(&mut self, y: f32) {
        self.velocity_direction.y = y;
    }
    pub fn set_velocity_direction_z(&mut self, z: f32) {
        self.velocity_direction.z = z;
    }

}