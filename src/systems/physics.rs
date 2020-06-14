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
        ReadStorage<'s, PhysicsHandle<PhysicsRigidBodyTag>>,
        WriteStorage<'s, PhysicsBodyDescription>,
        ReadStorage<'s, Direction>,
    );

    fn run(&mut self, (physics_world, rigid_body_tags, mut body_descs, directions): Self::SystemData) {
        let body_server = physics_world.rigid_body_server();
        for(body_desc, body_tag) in (&mut body_descs,&rigid_body_tags).join(){
            let belong_groups = body_server.belong_to(body_tag.get());
            // if group_belongs_to(CollisionGroupType::Ground, &belong_groups) {
            //     body_server.apply_impulse(
            //         body_tag.get(),
            //         &Vector3::new(0.,body_desc.mass()*FORCE_GRAVITY,0.));
            // }
            if group_belongs_to(CollisionGroupType::LinearMovable, &belong_groups) {
                body_server.set_contacts_to_report(body_tag.get(),5);
                let mut collide_events = vec![];
                body_server.contact_events(body_tag.get(),&mut collide_events);
                for &event in &collide_events {
                    let contact_belongs_to = body_server.belong_to(event.other_body);
                    for group_collide in contact_belongs_to{
                        let group_collide = CollisionGroupType::from(group_collide.get());

                        match group_collide{
                            CollisionGroupType::InvisibleWall =>{
                                if event.normal.y.round() !=1.{
                                    body_desc.set_velocity_direction_x(event.normal.x.round());
                                }
                            }
                            _ => {}
                        }
                    }
                }
                self.move_body(body_server,&body_tag,&body_desc);
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
        body_server.set_linear_velocity(
            body_tag.get(),
            &Vector3::new(body_desc.velocity_max()*body_desc.velocity_direction().x,vel.y,0.));
    }
}

