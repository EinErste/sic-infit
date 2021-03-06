use amethyst::{
    derive::SystemDesc,
    ecs::{Join, Entity, Read, System, SystemData, WriteStorage, ReadStorage, ReadExpect, Entities, Write},
    input::{InputHandler, StringBindings},
    core::math::Vector3,
    shrev::EventChannel,
    assets::AssetStorage,
    audio::{output::Output, Source},
};

use crate::components::{PhysicsBodyDescription, SimpleAnimation, StateAnimation, Player, CollisionGroupType, group_belongs_to};
use amethyst_physics::servers::PhysicsWorld;
use amethyst_physics::objects::{PhysicsHandle, CollisionGroup};
use amethyst_physics::prelude::PhysicsRigidBodyTag;
use crate::systems::{CoinPicked, Interact, HpEvent, SoundEffect};
use crate::systems::health::HpEvent::{HpGained, HpLost};
use amethyst_physics::PhysicsTime;
use crate::entities::MAX_COINS;
use crate::audio::{play_damage_sound, Sounds, play_death_sound};
use std::ops::Deref;
use crate::states::{GameplayStateType,GameplayStateTypes};

///This system controls the character control
#[derive(SystemDesc, Default)]
pub struct PlayerSystem {
    time_last_enemy_collide: f32,
    time_last_collectable_collide: f32,
    time_world_from_start: f32,
}


// const IMPULSE_JUMP: f32 = 10000000. * 2.;
const IMPULSE_JUMP: f32 = 10000000. * 1.2;
const IMPULSE_JUMP_DEFEAT_ENEMY: f32 = 40000000.;
const IMPULSE_RESISTANCE_WALL: f32 = 3000000.;
const IMPULSE_RESISTANCE_ENEMY: f32 = 5000000.;
const IMPULSE_MOVE: f32 = 800000.;

#[allow(dead_code)]
impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, PhysicsBodyDescription>,
        WriteStorage<'s, SimpleAnimation>,
        ReadExpect<'s, PhysicsTime>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, PhysicsWorld<f32>>,
        ReadStorage<'s, PhysicsHandle<PhysicsRigidBodyTag>>,
        ReadStorage<'s, Player>,
        Entities<'s>,
        Write<'s, EventChannel<CoinPicked>>,
        Write<'s, EventChannel<HpEvent>>,
        Read<'s, AssetStorage<Source>>,
        Read<'s, Sounds>,
        Option<Read<'s, Output>>,
        Write<'s, EventChannel<Interact>>,
        Write<'s, EventChannel<SoundEffect>>,
        Write<'s, GameplayStateType>
    );

    fn run(&mut self, (mut descs, mut animations, physics_time, input, physics_world, rigid_body_tags, player, entities,
        mut coinChannel, mut hpChannel, sources, sounds, output,
        mut interactChannel,  mut player_sound_effect_channel,
        mut gameplay_current_state,): Self::SystemData) {
        self.time_world_from_start += physics_time.delta_seconds();
        if self.time_world_from_start == 1. / 0. {
            self.time_world_from_start = 0.;
            self.time_last_enemy_collide = 0.;
            self.time_last_collectable_collide = 0.;
        }
        let body_server = physics_world.rigid_body_server();
        for (p_description, animation, p_body_tag, player) in (&mut descs, &mut animations, &rigid_body_tags, &player).join() {
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

            if let Some(action) = input.action_is_down("Action") {
                if action {
                    interactChannel.single_write(Interact());
                }
            }


            //--------------------
            // physics


            body_server.set_contacts_to_report(p_body_tag.get(), 8);
            let mut events = vec![];
            body_server.contact_events(p_body_tag.get(), &mut events);


            let mut velocity = body_server.linear_velocity(p_body_tag.get());
            //Check if able to jump
            let mut is_on_ground = false;
            for &contact_event in &events {
                if almost::zero_with(1. - contact_event.normal.y, 0.01) {
                    is_on_ground = true;
                    break;
                }
            }

            let mut sound_run_sent = false;
            //Contacts
            for &contact_event in &events {
                let belongs_to = body_server.belong_to(contact_event.other_body);
                for &collision_group in &belongs_to {
                    let collision_group = CollisionGroupType::from(collision_group.get());

                    match collision_group {
                        //TODO how to delete shape ? -_-
                        CollisionGroupType::Collectable => {
                            ///add coin
                            let time_between_collides = physics_time.delta_seconds() * 2.;
                            if self.time_world_from_start - self.time_last_collectable_collide > time_between_collides {
                                self.time_last_collectable_collide = self.time_world_from_start;
                                entities.delete(contact_event.other_entity.unwrap());
                                coinChannel.single_write(CoinPicked());
                            }
                        }
                        CollisionGroupType::Enemy => {
                            if almost::zero_with(1. - contact_event.normal.y, 0.01) {
                                //set entity to delete
                                body_server.set_belong_to(
                                    contact_event.other_body,
                                    vec![CollisionGroup::new(CollisionGroupType::Deletable.into()), ],
                                );
                                body_server.set_collide_with(
                                    contact_event.other_body,
                                    vec![CollisionGroup::new(CollisionGroupType::DeleteArea.into()), ],
                                );
                                play_death_sound(&*sounds, &sources, output.as_ref().map(|o| o.deref()));


                                //stop entity
                                body_server.set_linear_velocity(
                                    contact_event.other_body,
                                    &Vector3::new(0., 0., 0.),
                                );
                                //Entity last jump mericCat
                                body_server.apply_impulse(
                                    contact_event.other_body,
                                    &Vector3::new(0., IMPULSE_JUMP_DEFEAT_ENEMY, 0.));

                                //Player resistance jump
                                body_server.apply_impulse(
                                    p_body_tag.get(),
                                    &Vector3::new(0., IMPULSE_JUMP*1.3, 0.));
                            } else {
                                let time_between_collides = physics_time.delta_seconds()*30.;
                                if self.time_world_from_start - self.time_last_enemy_collide > time_between_collides {
                                    self.time_last_enemy_collide = self.time_world_from_start;
                                    //Player resistance impulse
                                    body_server.set_linear_velocity(
                                        p_body_tag.get(),
                                        &Vector3::new(0., 0., 0.));
                                    body_server.apply_impulse(
                                        p_body_tag.get(),
                                        &Vector3::new(IMPULSE_RESISTANCE_ENEMY * contact_event.normal.x, IMPULSE_RESISTANCE_ENEMY * 2., 0.));
                                    hpChannel.single_write(HpLost);
                                    if player.hp == 0 {
                                        play_death_sound(&*sounds, &sources, output.as_ref().map(|o| o.deref()));
                                        gameplay_current_state.state = GameplayStateTypes::Inactice;
                                    }
                                }
                            }
                        }
                        CollisionGroupType::Ground => {
                            if contact_event.normal.x != 0. {
                                body_server.apply_impulse(
                                    p_body_tag.get(),
                                    &Vector3::new(IMPULSE_RESISTANCE_WALL * contact_event.normal.x, 0., 0.));
                            }
                            if almost::zero_with(1. - contact_event.normal.y, 0.01) && p_description.velocity_direction().x != 0. {
                                player_sound_effect_channel.single_write(SoundEffect::Running);
                            } else {
                                player_sound_effect_channel.single_write(SoundEffect::Idle);
                            }
                            sound_run_sent = true;
                            let velocity_ground = body_server.linear_velocity(contact_event.other_body);
                            if almost::zero_with(1. - contact_event.normal.y, 0.01) && velocity_ground.x != 0. {
                                //Check if directions are same
                                //If player is moving
                                let x_direction_determinant = if velocity.x.signum() == velocity_ground.x.signum() { 1. } else { -1. };
                                if p_description.velocity_direction().x != 0. {
                                    player_sound_effect_channel.single_write(SoundEffect::Running);
                                    if x_direction_determinant == 1. {
                                        body_server.set_linear_velocity(
                                            p_body_tag.get(),
                                            &Vector3::new(
                                                p_description.velocity_max() * p_description.velocity_direction().x + velocity_ground.x,
                                                velocity.y + velocity_ground.y,
                                                0.));
                                    } else {
                                        body_server.set_linear_velocity(
                                            p_body_tag.get(),
                                            &Vector3::new(
                                                p_description.velocity_max() * p_description.velocity_direction().x,
                                                velocity.y + velocity_ground.y,
                                                0.));
                                    }
                                } else {
                                    body_server.set_linear_velocity(
                                        p_body_tag.get(),
                                        &Vector3::new(velocity_ground.x, velocity_ground.y, 0.));
                                }
                            }
                            //Prevent stuck by moving platform
                            if group_belongs_to(CollisionGroupType::LinearMovable, &belongs_to) {
                                if contact_event.normal.y == -1. && is_on_ground {
                                    body_server.apply_impulse(
                                        contact_event.other_body,
                                        &Vector3::new(
                                            0.,
                                            IMPULSE_JUMP * 10.,
                                            0.));
                                }

                                //todo
                                // if contact_event.normal.x != 0. && !is_on_ground{
                                //     body_server.apply_impulse(
                                //         contact_event.other_body,
                                //         &Vector3::new(
                                //             0.,
                                //             0.,
                                //             0.));
                                // }
                            }
                        }
                        CollisionGroupType::Exit => {
                            if player.coins == MAX_COINS {
                                gameplay_current_state.state = GameplayStateTypes::Inactice;
                            } else {
                                body_server.apply_impulse(
                                    p_body_tag.get(),
                                    &Vector3::new(-IMPULSE_JUMP, 0., 0.));
                            }
                        }
                        _ => {}
                    }
                }
            }

            if !sound_run_sent {
                player_sound_effect_channel.single_write(SoundEffect::Idle);
            }

            let mut velocity = body_server.linear_velocity(p_body_tag.get());
            //Jump
            if p_description.velocity_direction().y != 0. && is_on_ground {
                //Set y velocity to zero
                body_server.set_linear_velocity(
                    p_body_tag.get(),
                    &Vector3::new(velocity.x, 0., 0.));

                body_server.apply_impulse(
                    p_body_tag.get(),
                    &Vector3::new(0., IMPULSE_JUMP, 0.));
            }

            //Move
            if velocity.x.abs() <= p_description.velocity_max() || velocity.x.signum() != p_description.velocity_direction().x.signum() {
                body_server.apply_impulse(
                    p_body_tag.get(),
                    &Vector3::new(IMPULSE_MOVE * p_description.velocity_direction().x, 0., 0.));
            }

            //just in case (only 1 player entity exists)
            break;
        }
    }
}