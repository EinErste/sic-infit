use amethyst::{
    derive::SystemDesc,
    ecs::{Join,Entity, Read, System, SystemData, WriteStorage, ReadStorage, ReadExpect,Entities},
    input::{InputHandler, StringBindings},
    core::math::Vector3
};

use crate::components::{PhysicsBodyDescription, SimpleAnimation, StateAnimation, Player, CollisionGroupType};
use amethyst_physics::servers::PhysicsWorld;
use amethyst_physics::objects::{PhysicsHandle, CollisionGroup};
use amethyst_physics::prelude::PhysicsRigidBodyTag;

///This system controls the character control
#[derive(SystemDesc,Default)]
pub struct PlayerSystem {
}


const FORCE_MULTIPLIER: f32 = 1000000.0;
const IMPULSE_JUMP: f32 =  10000000. * 1.3;
const IMPULSE_JUMP_DEFEAT_ENEMY: f32 =  100000000. * 0.5;
const IMPULSE_MOVE: f32 =  500000. ;

#[allow(dead_code)]
impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, PhysicsBodyDescription>,
        WriteStorage<'s, SimpleAnimation>,
        Read<'s, InputHandler<StringBindings>>,

        ReadExpect<'s,PhysicsWorld<f32>>,
        ReadStorage<'s, PhysicsHandle<PhysicsRigidBodyTag>>,
        ReadStorage<'s, Player>,
        Entities<'s>,
    );

    fn run(&mut self, (mut descs, mut animations, input, physics_world, rigid_body_tags, player,entities): Self::SystemData) {
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

            //if let Some(jump) = input.action_is("Jump"){
            if let Some(jump) = input.action_is_down("Jump") {
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
                if almost::zero_with(1. - contact_event.normal.y, 0.01){
                    is_on_ground = true;
                    break;
                }
            }

            //Contacts
            for &contact_event in &events {
                let belongs_to = body_server.belong_to(contact_event.other_body);
                for collision_group in belongs_to {

                    let collision_group = CollisionGroupType::from(collision_group.get());

                    match collision_group{
                        //TODO how to delete shape ? -_-
                        CollisionGroupType::Collectable =>{
                            entities.delete(contact_event.other_entity.unwrap());
                            dbg!("+1 COIN");
                        }
                        CollisionGroupType::Enemy =>{
                            if almost::zero_with(1. - contact_event.normal.y, 0.01){
                                body_server.set_belong_to(
                                    contact_event.other_body,
                                    vec![CollisionGroup::new(CollisionGroupType::Deletable.into()),]
                                );
                                body_server.set_collide_with(
                                    contact_event.other_body,
                                    vec![CollisionGroup::new(CollisionGroupType::DeleteArea.into()),]
                                );
                                body_server.set_linear_velocity(
                                    contact_event.other_body,
                                    &Vector3::new(0.,0.,0.)
                                );
                                body_server.apply_impulse(
                                    contact_event.other_body,
                                    &Vector3::new(0.,IMPULSE_JUMP_DEFEAT_ENEMY,0.));
                                dbg!("Enemy dead");
                                //Not sure
                            } else{
                                dbg!("ENEMY COLLIDED");
                            }
                        }
                        CollisionGroupType::Ground =>{
                            if almost::zero_with(1. - contact_event.normal.y, 0.01){
                                let velocity_ground = body_server.linear_velocity(contact_event.other_body);
                                //Check if directions are same
                                let x_direction_determinant = if velocity.x.signum() == velocity_ground.x.signum() {1.} else {-1.};
                                //If player is moving

                                //TODO
                                if p_description.velocity_direction().x != 0. {
                                    if x_direction_determinant == 1.{
                                        body_server.set_linear_velocity(
                                            p_body_tag.get(),
                                            &Vector3::new(
                                                p_description.velocity_max()*p_description.velocity_direction().x + velocity_ground.x,
                                                velocity.y + velocity_ground.y,
                                                0.));
                                    }
                                    else {
                                        body_server.set_linear_velocity(
                                            p_body_tag.get(),
                                            &Vector3::new(
                                                p_description.velocity_max()*p_description.velocity_direction().x,
                                                velocity.y + velocity_ground.y,
                                                0.));
                                    }

                                } else{
                                    body_server.set_linear_velocity(
                                        p_body_tag.get(),
                                        &Vector3::new(velocity_ground.x,velocity_ground.y,0.));
                                }
                            }
                        }
                        _ => {}
                    }
                }

            }


            let mut velocity = body_server.linear_velocity(p_body_tag.get());
            if p_description.velocity_direction().y != 0. && is_on_ground{
                //Kinda crutch?
                body_server.set_linear_velocity(
                    p_body_tag.get(),
                    &Vector3::new(velocity.x,0.,0.));

                body_server.apply_impulse(
                    p_body_tag.get(),
                    &Vector3::new(0.,IMPULSE_JUMP,0.));
            }

            if velocity.x.abs() <=  p_description.velocity_max() || velocity.x.signum()!= p_description.velocity_direction().x.signum(){
                body_server.apply_impulse(
                    p_body_tag.get(),
                    &Vector3::new(IMPULSE_MOVE * p_description.velocity_direction().x,0.,0.));
            }


            //just in case (only 1 player entity exists)
            break;
        }
    }
}