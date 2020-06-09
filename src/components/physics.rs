use amethyst::{
    core::math::Vector3,
    ecs::{Component, DenseVecStorage},
};
#[repr(u8)]
pub enum CollisionGroupType {
    Ground = 1,
    Player = 2,
    NPC = 3,
    Enemy = 4,
}

impl Into<u8> for CollisionGroupType{
    fn into(self) -> u8 {
        self as u8
    }
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct PhysicsBodyDescription {
    velocity_direction: Vector3<f32>,
    velocity_max: f32,
    acceleration_time: f32,
    mass: f32,
}

impl Default for PhysicsBodyDescription {
    fn default() -> Self {
        PhysicsBodyDescription { velocity_direction: Vector3::new(0., 0., 0.), mass: 1.,acceleration_time: 2., velocity_max: 10.}
    }
}

impl PhysicsBodyDescription {
    pub fn new(mass: f32, vel_max: f32, acceleration_time: f32) -> Self {
        let mut desc = Self::default();
        desc.mass = mass;
        desc.velocity_max = vel_max;
        desc.acceleration_time = acceleration_time;
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

    pub fn acceleration_time(&self) ->  f32{
        self.acceleration_time
    }

    pub fn set_acceleration_time(&mut self, acceleration_time: f32){
        self.acceleration_time = acceleration_time;
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