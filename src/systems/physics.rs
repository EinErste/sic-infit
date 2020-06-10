use amethyst::{
    ecs::{Join, ReadStorage, ReadExpect, System, SystemData},
    derive::SystemDesc,
    core::Transform,
    core::math::Vector3
};

use crate::components::{PhysicsBodyDescription};
use amethyst_physics::servers::PhysicsWorld;
use amethyst_physics::PhysicsTime;
use amethyst_physics::objects::{PhysicsHandle, CollisionGroup};
use amethyst_physics::prelude::{PhysicsRigidBodyTag, RigidBodyDesc};


const FORCE_MULTIPLIER: f32 = 1000000.0;
const ACCELERATION_G: f32 = 10.;
const FORCE_GRAVITY: f32 = 1000.;
const IMPULSE_JUMP: f32 =  1000000.;

///This system controls the character control
#[derive(SystemDesc,Default)]
///System responsible for handling all of the physics: collisions, running, falling etc.
pub struct PhysicsSystem {}

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        ReadExpect<'s,PhysicsWorld<f32>>,
        ReadExpect<'s, PhysicsTime>,
        ReadStorage<'s, PhysicsHandle<PhysicsRigidBodyTag>>,
        ReadStorage<'s, PhysicsBodyDescription>,
    );

    fn run(&mut self, (physics_world,physics_time, rigid_body_tags, body_descs): Self::SystemData) {
        let body_server = physics_world.rigid_body_server();
        //TODO move this line somewhere out of system
        physics_world.world_server().set_gravity(&Vector3::new(0.,-FORCE_GRAVITY,0.));
        //physics_world.world_server().set_gravity(&Vector3::new(0.,0.,0.));
        for(body_desc, body_tag) in (&body_descs,&rigid_body_tags).join(){

            let mut velocity = body_server.linear_velocity(body_tag.get());
            let is_in_air = !almost::zero_with(velocity.y,0.2);

            if body_desc.velocity_direction().y != 0. && !is_in_air{
                body_server.apply_impulse(
                        body_tag.get(),
                        &Vector3::new(0.,body_desc.mass()*IMPULSE_JUMP,0.));
            }

            let mut velocity = body_server.linear_velocity(body_tag.get());
            if velocity.x.abs() <= body_desc.velocity_max() {
                body_server.apply_impulse(
                    body_tag.get(),
                    &Vector3::new(body_desc.mass() * IMPULSE_JUMP/10. * body_desc.velocity_direction().x,0.,0.));
                    // &Vector3::new(body_desc.mass() * body_desc.velocity_max()/body_desc.acceleration_time() * body_desc.velocity_direction().x,0.,0.));
            }


            //dbg!(body_server.linear_velocity(body_tag.get()));

        }
    }
}