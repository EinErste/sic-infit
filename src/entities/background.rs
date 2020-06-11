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
        sprite_number: 5,
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(960., 40., 0.);

    let shape = {
        let desc = ShapeDesc::Cube {half_extents: Vector3::new(960.,20.,10.)};
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
        //.with(sprite.clone())
        .with(transform)
        .with(shape)
        .with(rb)
        .build();
}


pub fn load_forest(world: &mut World){
    let distances:Vec<f32> = vec![-1500.,-1400.,-1300.,-1000.,-900.,0.0,30.];
    //let distances:Vec<f32> = vec![-15.,-12.,-10.,-8.,-5.,0.0,1.];
    let sprite_sheet_handle = {
        let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
        sprite_sheet_list.get(AssetType::BackgroundForest).unwrap().clone()
    };

    for i in 0..7 {
        let sprite = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: i,
        };
        let mut transform = Transform::default();
        transform.adjust_to_distance(-distances[i], 1920.,360.);
        transform.set_translation_xyz(960., 180., distances[i]);

        world
            .create_entity()
            .with(sprite.clone())
            .with(transform)
            .build();

    }
    load_forest_path(world);
    load_obstacles(world);
}



fn load_paddle(init_x: f32, init_y: f32, paddle_width: f32, world: &mut World){
    let column_width = 26 as f32;
    let column_height = 360 as f32;
    let paddle_height = 20 as f32;
    let paddle_inti_width = 26 as f32;
    let sprite_sheet_handle = {
        let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
        sprite_sheet_list.get(AssetType::Obstacles).unwrap().clone()
    };


    //---------------------------
    //Paddle

    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 1,
    };
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(paddle_width/paddle_inti_width,1.,1.));
    transform.set_translation_xyz(init_x + column_width + paddle_width/2., init_y-paddle_height/2., -1.);

    world
        .create_entity()
        .with(sprite.clone())
        .with(transform)
        .build();

    let mut transform = Transform::default();
    transform.set_translation_xyz(init_x + paddle_width/2. + column_width, init_y-paddle_height/2., -1.);

    let shape = {
        let desc = ShapeDesc::Cube {half_extents: Vector3::new(paddle_width/2. + column_width,paddle_height/2.,10.)};
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
        .with(rb)
        .with(shape)
        .with(transform)
        .build();


    //---------------------------
    //Columns

    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };
    let mut transform = Transform::default();
    transform.set_translation_xyz(init_x, -(column_height-init_y), -1.);

    world
        .create_entity()
        .with(sprite.clone())
        .with(transform)
        .build();

    let mut transform = Transform::default();
    transform.set_translation_xyz(init_x+paddle_width+column_width, -(column_height-init_y), -1.);

    world
        .create_entity()
        .with(sprite.clone())
        .with(transform)
        .build();
}

#[repr(u32)]
pub enum Height {
    Low = 160,
    Mid = 250,
    High = 340,
}

impl Into<f32> for Height{
    fn into(self) -> f32 {
        self as u32 as f32
    }
}

fn load_obstacles(world: &mut World){
    load_paddle(200.,Height::Low.into(),70.,world);
    load_paddle(400.,Height::Mid.into(),120.,world);
    load_paddle(600.,Height::High.into(),100.,world);
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