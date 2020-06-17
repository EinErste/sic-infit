use amethyst::{
    ecs::{Join, ReadStorage, WriteStorage, ReadExpect, System, SystemData, Entities},
    derive::SystemDesc,
    core::Transform,
    core::math::Vector3
};

use crate::components::{PhysicsBodyDescription, Player, CollisionGroupType, group_belongs_to, Direction, Directions};
use amethyst_physics::servers::PhysicsWorld;
use amethyst_physics::PhysicsTime;
use amethyst_physics::objects::{PhysicsHandle, CollisionGroup};
use amethyst_physics::prelude::{PhysicsRigidBodyTag, RigidBodyDesc, ContactEvent, RBodyPhysicsServerTrait, PhysicsAreaTag, OverlapEvent};
use amethyst::prelude::World;
use std::io::Read;
use std::convert::TryInto;


pub const FORCE_MULTIPLIER: f32 = 1000000.0;
pub const ACCELERATION_G: f32 = 10.;
pub const FORCE_GRAVITY: f32 = 1000.;
pub const IMPULSE_JUMP: f32 =  1000000.;
///This system controls the character control
pub struct PhysicsSystem {
    init: bool,
    time_world_from_start: f32,
}

impl Default for PhysicsSystem{
    fn default() -> Self {
        PhysicsSystem{init: true, time_world_from_start:0.}
    }
}

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        ReadExpect<'s,PhysicsWorld<f32>>,
        ReadStorage<'s, PhysicsHandle<PhysicsRigidBodyTag>>,
        ReadStorage<'s, PhysicsHandle<PhysicsAreaTag>>,
        WriteStorage<'s, PhysicsBodyDescription>,
        Entities<'s>,
        ReadExpect<'s, PhysicsTime>,
    );

    fn run(&mut self, (physics_world, rigid_body_tags, area_body_tags, mut body_descs, entities,physics_time): Self::SystemData) {
        self.time_world_from_start+=physics_time.delta_seconds();
        if self.time_world_from_start == 1./0. {
            self.time_world_from_start = 0.;
        }
        let body_server = physics_world.rigid_body_server();
        let area_server = physics_world.area_server();
        for (area_body_tag) in (&area_body_tags).join() {

            let area_belong_groups = area_server.belong_to(area_body_tag.get());
            for &area_group_body in &area_belong_groups {
                let area_group_type_body = CollisionGroupType::from(area_group_body.get());
                let mut overlap_events = area_server.overlap_events(area_body_tag.get());
                for &event in &overlap_events {
                    match area_group_type_body {
                        CollisionGroupType::InvisibleArea =>{
                            match event {
                                OverlapEvent::Enter(tag,entity) => {
                                    let body_desc = body_descs.get_mut(entity.unwrap()).unwrap();
                                    let (time,_) = body_desc.last_collision();
                                    //Prevent stuck
                                    if (self.time_world_from_start-time).abs() > 0.3{
                                        body_desc.set_last_collision(self.time_world_from_start,area_group_type_body);
                                        body_desc.set_velocity_direction_x(-body_desc.velocity_direction().x);
                                        body_desc.set_velocity_direction_y(-body_desc.velocity_direction().y);
                                    }
                                }
                                _ =>{}
                            }
                        }
                        CollisionGroupType::DeleteArea =>{
                            match event {
                                OverlapEvent::Enter(tag,entity) => {
                                    match entity {
                                        Some(entity) => {entities.delete(entity);},
                                        None => {dbg!("Error unwrapping overlap entity");}
                                    }
                                }
                                _=>{}
                            }
                        }
                        _=>{}
                    }

                }
            }
        }

        for(body_desc, body_tag) in (&mut body_descs,&rigid_body_tags).join() {
            let body_belongs_to = body_server.belong_to(body_tag.get());
            //body_server.set_contacts_to_report(body_tag.get(),5);
            for &group_type_body in &body_belongs_to {
                let group_type_body = CollisionGroupType::from(group_type_body.get());
                match group_type_body {
                    CollisionGroupType::LinearMovable => {
                        self.move_body(body_server, &body_tag, &body_desc);
                    }
                    _ => {}
                }
            }
        }

    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        world.fetch_mut::<PhysicsWorld<f32>>().world_server().set_gravity(&Vector3::new(0.,-FORCE_GRAVITY,0.));
    }
}

impl PhysicsSystem{
    fn move_body(&mut self,body_server: &dyn RBodyPhysicsServerTrait<f32>, body_tag: &PhysicsHandle<PhysicsRigidBodyTag>, body_desc: &PhysicsBodyDescription){
        let vel = body_server.linear_velocity(body_tag.get());
        let belongs_to = body_server.belong_to(body_tag.get());
        if group_belongs_to(CollisionGroupType::Enemy, &belongs_to) {
            body_server.set_linear_velocity(
                body_tag.get(),
                &Vector3::new(body_desc.velocity_max()*body_desc.velocity_direction().x,vel.y,0.));
        }
        if group_belongs_to(CollisionGroupType::Ground, &belongs_to){
            body_server.set_linear_velocity(
                body_tag.get(),
                &Vector3::new(body_desc.velocity_max()*body_desc.velocity_direction().x,body_desc.velocity_max()*body_desc.velocity_direction().y,0.));
        }

    }
}
