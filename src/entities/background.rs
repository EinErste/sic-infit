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

    //Main path
    let mut transform = Transform::default();
    transform.set_translation_xyz(960. as f32, 40., 0.);

    let shape = {
        let desc = ShapeDesc::Cube {half_extents: Vector3::new(960.,20.,50.)};
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
        .with(transform)
        .with(shape)
        .with(rb)
        .build();


    //Left wall


    let mut transform = Transform::default();
    transform.set_translation_xyz(160., 0., 0.);

    let shape = {
        let desc = ShapeDesc::Cube {half_extents: Vector3::new(160.,1000.,50.)};
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.shape_server().create(&desc)
    };

    let rb = {
        let mut rb_desc = RigidBodyDesc::default();
        rb_desc.mode = BodyMode::Static;
        rb_desc.friction = 0.0;
        rb_desc.bounciness = 1.0;
        rb_desc.belong_to = vec![CollisionGroup::new(CollisionGroupType::WorldWall.into())];
        rb_desc.collide_with = vec![CollisionGroup::new(CollisionGroupType::Player.into()),
                                    CollisionGroup::new(CollisionGroupType::NPC.into()),
                                    CollisionGroup::new(CollisionGroupType::Enemy.into()),];
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };

    world
        .create_entity()
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

    for j in 1..3{
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
    }
    load_forest_path(world);
    load_obstacles(world);
}



fn load_platform(init_x: f32, init_y: f32, platform_width: f32, world: &mut World){
    let column_width = 26 as f32;
    let column_height = 360 as f32;
    let platform_height = 20 as f32;
    let platform_init_width = 26 as f32;
    let sprite_sheet_handle = {
        let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
        sprite_sheet_list.get(AssetType::Obstacles).unwrap().clone()
    };


    //---------------------------
    //Platform

    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 1,
    };
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(platform_width/platform_init_width,1.,1.));
    transform.set_translation_xyz(init_x + column_width + platform_width/2., init_y-platform_height/2., -1.);

    world
        .create_entity()
        .with(sprite.clone())
        .with(transform)
        .build();

    let mut transform = Transform::default();
    transform.set_translation_xyz(init_x + platform_width/2. + column_width, init_y-platform_height/2., -1.);

    let shape = {
        let desc = ShapeDesc::Cube {half_extents: Vector3::new(platform_width/2. + column_width,platform_height/2.,50.)};
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
    transform.set_translation_xyz(init_x+platform_width+column_width, -(column_height-init_y), -1.);

    world
        .create_entity()
        .with(sprite.clone())
        .with(transform)
        .build();

    //-------------------------------
    //Invisible walls


    //LEFT WALl
    let wall_width = column_width;
    let wall_height = 200_f32;
    let mut transform = Transform::default();
    transform.set_translation_xyz(init_x - wall_width, init_y + wall_height/2., 0.);

    let shape = {
        let desc = ShapeDesc::Cube {half_extents: Vector3::new(wall_width/2.,wall_height/2.,50.)};
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.shape_server().create(&desc)
    };

    let rb = {
        let mut rb_desc = RigidBodyDesc::default();
        rb_desc.mode = BodyMode::Static;
        rb_desc.friction = 1.0;
        rb_desc.bounciness = 1.00;
        rb_desc.belong_to = vec![CollisionGroup::new(CollisionGroupType::Wall.into())];
        rb_desc.collide_with = vec![CollisionGroup::new(CollisionGroupType::Enemy.into()),];
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };

    world
        .create_entity()
        .with(rb)
        .with(shape)
        .with(transform)
        .build();


    //RIGHT WALL
    let mut transform = Transform::default();
    transform.set_translation_xyz(init_x + column_width*2. + platform_width + wall_width, init_y + wall_height/2., 0.);

    let shape = {
        let desc = ShapeDesc::Cube {half_extents: Vector3::new(wall_width/2.,wall_height/2.,50.)};
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.shape_server().create(&desc)
    };

    let rb = {
        let mut rb_desc = RigidBodyDesc::default();
        rb_desc.mode = BodyMode::Static;
        rb_desc.friction = 0.5;
        rb_desc.bounciness = 1.00;
        rb_desc.belong_to = vec![CollisionGroup::new(CollisionGroupType::Wall.into())];
        rb_desc.collide_with = vec![CollisionGroup::new(CollisionGroupType::Enemy.into()),];
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };

    world
        .create_entity()
        .with(rb)
        .with(shape)
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
    load_platform(400.,Height::Low.into(),70.,world);
    load_platform(600.,Height::Mid.into(),120.,world);
    load_platform(800.,Height::High.into(),100.,world);
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