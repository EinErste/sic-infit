use amethyst::{
    derive::SystemDesc,
    ecs::{Join,Entity, Read, System, SystemData, WriteStorage, ReadStorage, ReadExpect},
    input::{InputHandler, StringBindings},
    core::math::Vector3
};

use crate::components::{PhysicsBodyDescription, SimpleAnimation, StateAnimation, Player, CollisionGroupType};
use amethyst_physics::servers::PhysicsWorld;
use amethyst_physics::objects::PhysicsHandle;
use amethyst_physics::prelude::PhysicsRigidBodyTag;

///This system controls the character control
#[derive(SystemDesc,Default)]
pub struct PlayerSystem {
}


const FORCE_MULTIPLIER: f32 = 1000000.0;
const ACCELERATION_G: f32 = 10.;
const FORCE_GRAVITY: f32 = 1000.;
const IMPULSE_JUMP: f32 =  1000000.;

#[allow(dead_code)]
impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, PhysicsBodyDescription>,
        WriteStorage<'s, SimpleAnimation>,
        Read<'s, InputHandler<StringBindings>>,

        ReadExpect<'s,PhysicsWorld<f32>>,
        ReadStorage<'s, PhysicsHandle<PhysicsRigidBodyTag>>,
        ReadStorage<'s, Player>
    );

    fn run(&mut self, (mut descs, mut animations, input, physics_world, rigid_body_tags, player): Self::SystemData) {
        let body_server = physics_world.rigid_body_server();

        for (p_description, animation, p_body_tag, _player) in  (&mut descs, &mut animations, &rigid_body_tags, &player).join(){

            if let Some(x) = input.axis_value("x-axis") {
                if x == 0. {
                    p_description.set_velocity_direction_x(0.);
                    animation.change_state(StateAnimation::Idle);
                } else {
                    p_description.set_velocity_direction_x(x);
                    animation.change_state(StateAnimation::Run);
                }
            }

            if let Some(jump) = input.action_is_down("Jump"){
                if jump {
                    p_description.set_velocity_direction_y(1.);
                } else {
                    p_description.set_velocity_direction_y(0.);
                }
            }


            //--------------------
            // physics


            let mut velocity = body_server.linear_velocity(p_body_tag.get());
            let is_in_air = !almost::zero_with(velocity.y,0.2);

            if p_description.velocity_direction().y != 0. && !is_in_air{
                body_server.apply_impulse(
                    p_body_tag.get(),
                    &Vector3::new(0.,p_description.mass()*IMPULSE_JUMP*1.5,0.));
            }

            let mut velocity = body_server.linear_velocity(p_body_tag.get());
            if velocity.x.abs() <= p_description.velocity_max() {
                body_server.apply_impulse(
                    p_body_tag.get(),
                    &Vector3::new(p_description.mass() * IMPULSE_JUMP/10. * p_description.velocity_direction().x,0.,0.));
                // &Vector3::new(body_desc.mass() * body_desc.velocity_max()/body_desc.acceleration_time() * body_desc.velocity_direction().x,0.,0.));
            }
            //dbg!(body_server.linear_velocity(body_tag.get()));


            //Test contacts

            body_server.set_contacts_to_report(p_body_tag.get(),3);
            let mut events = vec![];
            body_server.contact_events(p_body_tag.get(),&mut events);
            for i in events{
                let belongs_to = body_server.belong_to(i.other_body);
                for j in belongs_to {
                    let enemy: u8 = CollisionGroupType::Enemy.into();
                    if j.get() == enemy{
                        dbg!("ENEMY COLLIDED");
                    }
                }
            }


            //just in case (only 1 player entity exists)
            break;
        }






        // let player_desc = descs.get_mut(self.player).unwrap();
        // let player_anim = animations.get_mut(self.player).unwrap();

        //----------when jump anim? never------------
        // if player_anim.current_state == jump {
        //     let (i,j,time_step) = player_anim.states[StateAnimation::Jump];
        //     if player_anim.time_elapsed == time_step*(i-j){
        //         player_anim.change_state(//Sosat);
        //     }
        // }
    }
}