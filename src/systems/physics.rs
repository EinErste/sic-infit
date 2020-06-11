use amethyst::{
    ecs::{Join, ReadStorage, ReadExpect, System, SystemData},
    derive::SystemDesc,
    core::Transform,
    core::math::Vector3
};

use crate::components::{PhysicsBodyDescription, Player, CollisionGroupType};
use amethyst_physics::servers::PhysicsWorld;
use amethyst_physics::PhysicsTime;
use amethyst_physics::objects::{PhysicsHandle, CollisionGroup};
use amethyst_physics::prelude::{PhysicsRigidBodyTag, RigidBodyDesc, ContactEvent};


const FORCE_MULTIPLIER: f32 = 1000000.0;
const ACCELERATION_G: f32 = 10.;
const FORCE_GRAVITY: f32 = 1000.;
const IMPULSE_JUMP: f32 =  1000000.;

///This system controls the character control
#[derive(SystemDesc,Default)]
pub struct PhysicsSystem {}

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        ReadExpect<'s,PhysicsWorld<f32>>,
        ReadExpect<'s, PhysicsTime>,
        ReadStorage<'s, PhysicsHandle<PhysicsRigidBodyTag>>,
        ReadStorage<'s, PhysicsBodyDescription>,
        ReadStorage<'s, Player>
    );

    fn run(&mut self, (physics_world,physics_time, rigid_body_tags, body_descs, player): Self::SystemData) {
        let body_server = physics_world.rigid_body_server();
        //TODO move this line somewhere out of system
        physics_world.world_server().set_gravity(&Vector3::new(0.,-FORCE_GRAVITY,0.));
        for(body_desc, body_tag) in (&body_descs,&rigid_body_tags).join(){
        }
    }
}