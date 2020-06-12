use amethyst::{
    ecs::{Join, ReadStorage, WriteStorage, ReadExpect, System, SystemData},
    derive::SystemDesc,
    core::Transform,
    core::math::Vector3
};

use crate::components::{PhysicsBodyDescription, Player, CollisionGroupType, group_belongs_to, Direction, Directions};
use amethyst_physics::servers::PhysicsWorld;
use amethyst_physics::PhysicsTime;
use amethyst_physics::objects::{PhysicsHandle, CollisionGroup};
use amethyst_physics::prelude::{PhysicsRigidBodyTag, RigidBodyDesc, ContactEvent, RBodyPhysicsServerTrait};
use amethyst::prelude::World;


pub const FORCE_MULTIPLIER: f32 = 1000000.0;
pub const ACCELERATION_G: f32 = 10.;
pub const FORCE_GRAVITY: f32 = 1000.;
pub const IMPULSE_JUMP: f32 =  1000000.;

///This system controls the character control
pub struct PhysicsSystem {
    init: bool
}

impl Default for PhysicsSystem{
    fn default() -> Self {
        PhysicsSystem{init: true}
    }
}

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        ReadExpect<'s,PhysicsWorld<f32>>,
        ReadExpect<'s, PhysicsTime>,
        ReadStorage<'s, PhysicsHandle<PhysicsRigidBodyTag>>,
        WriteStorage<'s, PhysicsBodyDescription>,
        ReadStorage<'s, Direction>,
    );

    fn run(&mut self, (physics_world,physics_time, rigid_body_tags, mut body_descs, directions): Self::SystemData) {
        let body_server = physics_world.rigid_body_server();
        for(body_desc, body_tag, dir) in (&mut body_descs,&rigid_body_tags,&directions).join(){
            let belong_groups = body_server.belong_to(body_tag.get());

            if group_belongs_to(CollisionGroupType::Enemy, &belong_groups){
                if self.init{
                    body_server.set_contacts_to_report(body_tag.get(),5);
                    body_desc.set_velocity_direction_x(-1.);
                    self.init = false;
                }
                let mut collide_events = vec![];
                body_server.contact_events(body_tag.get(),&mut collide_events);

                for event in collide_events {
                    let contact_belongs_to = body_server.belong_to(event.other_body);

                    for group_collide in contact_belongs_to{
                        let group_collide = CollisionGroupType::from(group_collide.get());
                        match group_collide{
                            CollisionGroupType::Wall =>{
                                body_desc.set_velocity_direction_x(-body_desc.velocity_direction().x);
                            }
                            _ => {}
                        }
                    }
                }
                self.move_body(body_server,&body_tag,&body_desc,dir);
            }

        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        world.fetch_mut::<PhysicsWorld<f32>>().world_server().set_gravity(&Vector3::new(0.,-FORCE_GRAVITY,0.));
    }
}

impl PhysicsSystem{
    fn move_body(&mut self,body_server: &dyn RBodyPhysicsServerTrait<f32>, body_tag: &PhysicsHandle<PhysicsRigidBodyTag>, body_desc: &PhysicsBodyDescription, dir: &Direction){
        let vel = body_server.linear_velocity(body_tag.get());
        match dir.dir {
            Directions::Right =>{
                body_server.set_linear_velocity(
                    body_tag.get(),
                    &Vector3::new(body_desc.velocity_max(),vel.y,0.)
                );
            }
            Directions::Left =>{
                body_server.set_linear_velocity(
                    body_tag.get(),
                    &Vector3::new(-body_desc.velocity_max(),vel.y,0.)
                );
            }
        }
    }
}