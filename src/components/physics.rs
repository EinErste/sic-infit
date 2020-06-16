use amethyst::{
    core::math::Vector3,
    ecs::{Component, DenseVecStorage},
};
use crate::components::physics::CollisionGroupType::{Ground, Player, NPC, Enemy, Undefined, WorldWall, LinearMovable, Collectable, DeleteArea, Deletable, InvisibleArea, SupportGround, Exit};
use amethyst_physics::objects::{CollisionGroup, PhysicsHandle};
use amethyst_physics::prelude::{PhysicsShapeTag, ShapeDesc};
use amethyst_physics::servers::PhysicsWorld;
use amethyst::prelude::World;
use amethyst::core::transform::Transform;


#[derive(PartialEq,Debug,Copy,Clone)]
#[repr(u8)]
pub enum CollisionGroupType {
    Undefined = 0,
    Ground = 1,
    Player = 2,
    NPC = 3,
    Enemy = 4,
    InvisibleArea = 5,
    WorldWall = 6,
    LinearMovable = 7,
    Collectable = 8,
    DeleteArea = 9,
    Deletable = 10,
    SupportGround = 11,
    Exit,
}

impl From<u8> for CollisionGroupType{
    fn from(group : u8) -> Self {
        match group {
            1 => Ground,
            2 => Player,
            3 => NPC,
            4 => Enemy,
            5 => InvisibleArea,
            6 => WorldWall,
            7 => LinearMovable,
            8 => Collectable,
            9 => DeleteArea,
            10 => Deletable,
            11 => SupportGround,
            12 => Exit,
            _ => Undefined,
        }
    }
}

pub fn group_belongs_to(group: CollisionGroupType,vec: &Vec<CollisionGroup>)->bool{
    let group: u8 = group.into();
    for &i in vec{
        if group == i.get() {
            return true;
        }
    }
    return false
}

impl Into<u8> for CollisionGroupType{
    fn into(self) -> u8 {
        self as u8
    }
}

#[derive(Component)]
#[storage(DenseVecStorage)]
///Component that describes a physical body
pub struct PhysicsBodyDescription {
    velocity_direction: Vector3<f32>,
    velocity_max: f32,
    mass: f32,
    last_collision_time: f32,
    last_collision_group: CollisionGroupType,
}

impl Default for PhysicsBodyDescription {
    fn default() -> Self {
        PhysicsBodyDescription { velocity_direction: Vector3::new(0., 0., 0.), mass: 1., velocity_max: 10., last_collision_time: 0.,last_collision_group: CollisionGroupType::Undefined}
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

    pub fn set_last_collision(&mut self, last_collision_time: f32, last_collision_group: CollisionGroupType,){
        self.last_collision_time = last_collision_time;
        self.last_collision_group = last_collision_group;
    }

    pub fn last_collision(&mut self) -> (f32, CollisionGroupType){
        (self.last_collision_time,self.last_collision_group)
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

pub fn create_cube(init_x:f32, init_y:f32, init_z:f32, width:f32,height:f32,depth:f32,world: &mut World) -> (PhysicsHandle<PhysicsShapeTag>,Transform){
    let mut transform = Transform::default();
    transform.set_translation_xyz(init_x + width/2., init_y + height/2., init_z);

    let shape = {
        let desc = ShapeDesc::Cube {half_extents: Vector3::new(width/2.,height/2.,depth/2.)};
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.shape_server().create(&desc)
    };
    (shape,transform)
}