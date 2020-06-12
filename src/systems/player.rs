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
const IMPULSE_JUMP: f32 =  10000000. * 1.3;
const IMPULSE_MOVE: f32 =  500000. ;

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


            body_server.set_contacts_to_report(p_body_tag.get(),5);
            let mut events = vec![];
            body_server.contact_events(p_body_tag.get(),&mut events);


            let mut velocity = body_server.linear_velocity(p_body_tag.get());
            //Check if able to jump
            let mut is_on_ground = false;
            for &contact_event in &events{

                //THIS SHIT DOESNT WORK PROPERLY! WHY? HAS I EVER?
                if almost::zero_with(1. - contact_event.normal.y, 0.01) && almost::zero_with(velocity.y,0.1){
                    is_on_ground = true;
                    //dbg!(contact_event.normal);
                    break;
                }
            }


            //Contacts
            for &contact_event in &events {
                let belongs_to = body_server.belong_to(contact_event.other_body);
                for collision_group in belongs_to {
                    let enemy: u8 = CollisionGroupType::Enemy.into();
                    if collision_group.get() == enemy {
                        if almost::zero_with(1. - contact_event.normal.y, 0.01){
                            // dbg!(contact_event.normal);
                            body_server.apply_impulse(
                                p_body_tag.get(),
                                &Vector3::new(0.,IMPULSE_JUMP,0.));
                        }
                        dbg!("ENEMY COLLIDED");
                    }
                }

            }



            if p_description.velocity_direction().y != 0. && is_on_ground{
                body_server.apply_impulse(
                    p_body_tag.get(),
                    &Vector3::new(0.,IMPULSE_JUMP,0.));
            }

            let mut velocity = body_server.linear_velocity(p_body_tag.get());
            if velocity.x.abs() <= p_description.velocity_max() || velocity.x.signum()!= p_description.velocity_direction().x.signum(){
                body_server.apply_impulse(
                    p_body_tag.get(),
                    &Vector3::new(IMPULSE_MOVE * p_description.velocity_direction().x,0.,0.));
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