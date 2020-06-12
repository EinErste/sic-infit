use amethyst::{
    prelude::{World, WorldExt, Builder},
    core::transform::Transform,
    renderer::{SpriteRender},
    ecs::{Entity},
    core::math::{Vector3}
};
use crate::{
    resources::{SpriteSheetList, AssetType},
    components::{Direction, SimpleAnimation, Directions, StateAnimation, Player}
};
use enum_map::{enum_map};
use amethyst_physics::{
    prelude::{ShapeDesc, RigidBodyDesc, BodyMode},
    servers::PhysicsWorld,
    objects::PhysicsHandle
};
use crate::components::{PhysicsBodyDescription, CollisionGroupType};
use amethyst_physics::objects::CollisionGroup;
use amethyst_physics::prelude::PhysicsShapeTag;


pub fn load_player(world: &mut World) -> Entity{
    let sprite_sheet_handle = {
        let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
        sprite_sheet_list.get(AssetType::Character).unwrap().clone()
    };
    let transform =
        Transform::default().set_translation_xyz(360., 240., 1.).to_owned();
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };
    let shape = {
        let desc = ShapeDesc::Cube {half_extents: Vector3::new(20.,32.,5.)};
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.shape_server().create(&desc)
    };

    let rb = {
        let mut rb_desc = RigidBodyDesc::default();
        rb_desc.lock_translation_z = true;
        rb_desc.lock_rotation_x = true;
        rb_desc.lock_rotation_y = true;
        rb_desc.lock_rotation_z = true;
        rb_desc.friction = 0.5;
        rb_desc.bounciness = 0.00;
        rb_desc.mass = 10.;
        rb_desc.belong_to = vec![CollisionGroup::new(CollisionGroupType::Player.into())];
        rb_desc.collide_with = vec![CollisionGroup::new(CollisionGroupType::Ground.into()),
                                    CollisionGroup::new(CollisionGroupType::NPC.into()),
                                    CollisionGroup::new(CollisionGroupType::Enemy.into())];
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };
    world
        .create_entity()
        .with(sprite)
        .with(transform)
        .with(PhysicsBodyDescription::new(10.,150.,20.))
        .with(Direction{dir: Directions::Right})
        .with(Player{})
        .with(shape)
        .with(rb)
        .with(SimpleAnimation::new(StateAnimation::Idle,enum_map!(
            StateAnimation::Run => (2,10,0.1),
            StateAnimation::Idle => (0,2,0.8),
            _ => (0,1,0.1)
        )))
        .build()
}

pub fn load_lion(world: &mut World){
    let sprite_sheet_handle = {
        let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
        sprite_sheet_list.get(AssetType::Character).unwrap().clone()
    };
    let transform =
        Transform::default().set_translation_xyz(600., 300., 1.).to_owned();
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };
    let shape: PhysicsHandle<PhysicsShapeTag> = {
        let desc = ShapeDesc::Cube {half_extents: Vector3::new(24.,32.,5.)};
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.shape_server().create(&desc)
    };


    let rb = {
        let mut rb_desc = RigidBodyDesc::default();
        rb_desc.lock_translation_z = true;
        rb_desc.lock_rotation_x = true;
        rb_desc.lock_rotation_y = true;
        rb_desc.lock_rotation_z = true;
        rb_desc.friction = 0.5;
        rb_desc.bounciness = 0.05;
        rb_desc.mass = 10.;
        rb_desc.mode = BodyMode::Dynamic;
        rb_desc.belong_to = vec![CollisionGroup::new(CollisionGroupType::Enemy.into())];
        rb_desc.collide_with = vec![CollisionGroup::new(CollisionGroupType::Ground.into()),
                                    CollisionGroup::new(CollisionGroupType::NPC.into()),
                                    CollisionGroup::new(CollisionGroupType::Player.into()),
                                    CollisionGroup::new(CollisionGroupType::Wall.into()),];
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };
    world
        .create_entity()
        .with(sprite)
        .with(transform)
        .with(shape)
        .with(rb)
        .with(PhysicsBodyDescription::new(10.,120.,1.))
        .with(Direction{dir: Directions::Left})
        .build();
}