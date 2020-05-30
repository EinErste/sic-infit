use amethyst::{
    ecs::{Join, ReadStorage, ReadExpect, System, SystemData, WriteStorage},
    derive::SystemDesc,
    core::Transform,
    core::math::Vector3
};

use crate::components::{Motion, Directions, Direction};
use amethyst_physics::servers::PhysicsWorld;
use amethyst_physics::PhysicsTime;
use amethyst_physics::objects::PhysicsHandle;
use amethyst_physics::prelude::PhysicsRigidBodyTag;

///This system controls the character control
#[derive(SystemDesc)]
pub struct MotionSystem {}

impl<'s> System<'s> for MotionSystem {
    type SystemData = (
        ReadExpect<'s,PhysicsWorld<f32>>,
        ReadExpect<'s, PhysicsTime>,
        ReadStorage<'s, PhysicsHandle<PhysicsRigidBodyTag>>,
        ReadStorage<'s, Motion>,
        WriteStorage<'s, Direction>,
    );

    fn run(&mut self, (physics_world,physics_time, rigid_body_tags, motions, mut dirs): Self::SystemData) {
        for (motion, dir) in (&motions, &mut dirs,).join() {
            if motion.velocity.x < 0. {
                dir.dir = Directions::Left;
            } else if motion.velocity.x > 0.{
                dir.dir = Directions::Right;
            }
        }
        let force_mult = 1000000 as f32;
        for(motion, body_tag) in (&motions,&rigid_body_tags).join(){
            //Push
            physics_world.rigid_body_server().apply_force(
                body_tag.get(),
                &Vector3::new(motion.velocity.x*force_mult,0.,0.));

            //Break
            let velocity = physics_world.rigid_body_server().linear_velocity(body_tag.get());
            let break_force = (velocity / physics_time.delta_seconds()) * -1.0;
            physics_world.rigid_body_server().apply_force(
                body_tag.get(),
                &break_force);
        }
    }
}