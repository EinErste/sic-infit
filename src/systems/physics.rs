use amethyst::{
    ecs::{Join, ReadStorage, ReadExpect, System, SystemData},
    derive::SystemDesc,
    core::Transform,
    core::math::Vector3
};

use crate::components::{Motion};
use amethyst_physics::servers::PhysicsWorld;
use amethyst_physics::PhysicsTime;
use amethyst_physics::objects::PhysicsHandle;
use amethyst_physics::prelude::{PhysicsRigidBodyTag};


const FORCE_MULTIPLIER: f32 = 1000000.0;
const FORCE_GRAVITY: f32 = 5000000.;
const IMPULSE_JUMP: f32 =  2000000.;


///This system controls the character control
#[derive(SystemDesc,Default)]
pub struct PhysicsSystem {}

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        ReadExpect<'s,PhysicsWorld<f32>>,
        ReadExpect<'s, PhysicsTime>,
        ReadStorage<'s, PhysicsHandle<PhysicsRigidBodyTag>>,
        ReadStorage<'s, Motion>,
    );

    fn run(&mut self, (physics_world,physics_time, rigid_body_tags, motions): Self::SystemData) {
        let server = physics_world.rigid_body_server();
        for(motion, body_tag) in (&motions,&rigid_body_tags).join(){

            let mut velocity = server.linear_velocity(body_tag.get());

            let is_in_air = {
              !almost::zero_with(velocity.y,0.2)
            };

            if motion.velocity.y != 0. && !is_in_air{
                server.apply_impulse(
                        body_tag.get(),
                        &Vector3::new(0.,IMPULSE_JUMP,0.));
            }

           if is_in_air {
               //Gravity
               server.apply_force(
                   body_tag.get(),
                   &Vector3::new(0.,-FORCE_GRAVITY,0.));
           }

            server.apply_force(
                body_tag.get(),
                &Vector3::new(motion.velocity.x * FORCE_MULTIPLIER,0.,0.));


            // let mut velocity = server.linear_velocity(body_tag.get());
            // velocity.x = motion.velocity.x * FORCE_MULTIPLIER;
            // //Push
            // server.set_linear_velocity(
            //     body_tag.get(),
            //             &velocity);

            dbg!(server.linear_velocity(body_tag.get()));

        }
    }
}