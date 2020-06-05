use amethyst::{
    ecs::{Join, ReadStorage, ReadExpect , System, SystemData,},
    derive::SystemDesc,
    core::math::Vector3
};

use crate::components::{Parallax, Directions, Direction, Player, PhysicsBodyDescription};
use amethyst_physics::servers::PhysicsWorld;
use amethyst_physics::objects::PhysicsHandle;
use amethyst_physics::prelude::PhysicsRigidBodyTag;
use amethyst_physics::prelude::*;
#[derive(SystemDesc,Default)]
pub struct ParallaxSystem {

}

impl<'s> System<'s> for ParallaxSystem {
    type SystemData = (
        ReadStorage<'s, Parallax>,
        ReadStorage<'s, Player>,
        ReadExpect<'s,PhysicsWorld<f32>>,
        ReadStorage<'s, PhysicsHandle<PhysicsRigidBodyTag>>,
        ReadStorage<'s, PhysicsBodyDescription>,
    );

    fn run(&mut self, (parallaxes, players, physics_world , rigid_body_tags, body_descs): Self::SystemData) {

        let server = physics_world.rigid_body_server();
        let mut player_velocity = Vector3::new(0.,0.,0.);
        for (_player, body_tag,body_desc) in (&players, &rigid_body_tags, &body_descs).join() {
            player_velocity = server.linear_velocity(body_tag.get());
            break;
        }

        for (parallax, body_tag) in (&parallaxes, &rigid_body_tags).join() {
            let mut velocity = player_velocity.clone();
            velocity.x *= parallax.velocity_ratio.x;
            //velocity.y *= parallax.velocity_ratio.y;
            velocity.y = 0.;
            server.set_linear_velocity(
                body_tag.get(),
                        &velocity);
        }
    }
}