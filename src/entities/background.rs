use amethyst::prelude::{World, WorldExt, Builder};
use amethyst::core::transform::Transform;
use amethyst::renderer::{SpriteRender};
use amethyst::ecs::Entity;
use crate::resources::{SpriteSheetList, AssetType};
use amethyst_physics::prelude::{ShapeDesc, RigidBodyDesc, BodyMode};
use amethyst_physics::servers::PhysicsWorld;
use amethyst::core::math::Vector3;
use crate::entities::AdjustToDistance;
use amethyst_physics::objects::CollisionGroup;
use crate::components::CollisionGroupType;

pub fn load_forest_path(world: &mut World){
    let sprite_sheet_handle = {
        let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
        sprite_sheet_list.get(AssetType::BackgroundForest).unwrap().clone()
    };

    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 6,
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(960., 60., 0.);

    let shape = {
        let desc = ShapeDesc::Cube {half_extents: Vector3::new(960.,5.,10.)};
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.shape_server().create(&desc)
    };

    let rb = {
        let mut rb_desc = RigidBodyDesc::default();
        rb_desc.mode = BodyMode::Static;
        rb_desc.friction = 0.5;
        rb_desc.bounciness = 0.00;
        rb_desc.belong_to = vec![CollisionGroup::new(CollisionGroupType::Ground.into())];
        rb_desc.collide_with = vec![CollisionGroup::new(CollisionGroupType::Player.into()),
                                    CollisionGroup::new(CollisionGroupType::NPC.into()),
                                    CollisionGroup::new(CollisionGroupType::Enemy.into()),];
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };

    world
        .create_entity()
        .with(sprite.clone())
        .with(transform)
        .with(shape)
        .with(rb)
        .build();
}


pub fn load_forest(world: &mut World){
    let distances:Vec<f32> = vec![-1500.,-1000.,-500.,-100.,-40.,-10.,0.0,10.];
    let sprite_sheet_handle = {
        let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
        sprite_sheet_list.get(AssetType::BackgroundForest).unwrap().clone()
    };

    for i in 0..8 {
        if i == 6 {continue};

        let sprite = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: i,
        };
        let mut transform = Transform::default();
        transform.adjust_to_distance(distances[i], 1920.,360.);
        transform.set_translation_xyz(960., 180., distances[i]);

        world
            .create_entity()
            .with(sprite.clone())
            .with(transform)
            .build();

    }
    load_forest_path(world);
}

pub fn load_intro(world: &mut World) -> Entity{

    let sprite_sheet_handle = {
        let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
        sprite_sheet_list.get(AssetType::Intro).unwrap().clone()
    };
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };
    let mut transform = Transform::default();
    transform.set_translation_xyz(420., 180., 3.);
    world
        .create_entity()
        .with(sprite.clone())
        .with(transform)
        .build()

}