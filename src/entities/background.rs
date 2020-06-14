//! A set of useful functions for loading the spritess
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
use crate::components::{CollisionGroupType, PhysicsBodyDescription, create_cube};


//All x and y parameters stands for left bottom point
pub fn load_forest_path(world: &mut World){

    //Main path
    let cube = create_cube(0.,40.,0.,1920.,20.,100.,world);

    let rb = {
        let mut rb_desc = RigidBodyDesc::default();
        rb_desc.mode = BodyMode::Static;
        rb_desc.friction = 0.5;
        rb_desc.bounciness = 0.00;
        rb_desc.belong_to = vec![CollisionGroup::new(CollisionGroupType::Ground.into())];
        rb_desc.collide_with = vec![CollisionGroup::new(CollisionGroupType::Player.into()),
                                    CollisionGroup::new(CollisionGroupType::NPC.into()),
                                    CollisionGroup::new(CollisionGroupType::Collectable.into()),
                                    CollisionGroup::new(CollisionGroupType::Enemy.into()),];
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };

    world
        .create_entity()
        .with(cube.0)
        .with(cube.1)
        .with(rb)
        .build();


    //Left wall

    let cube = create_cube(0.,0.,0.,320.,1000.,100.,world);

    let rb = {
        let mut rb_desc = RigidBodyDesc::default();
        rb_desc.mode = BodyMode::Static;
        rb_desc.friction = 0.0;
        rb_desc.bounciness = 1.0;
        rb_desc.belong_to = vec![CollisionGroup::new(CollisionGroupType::WorldWall.into())];
        rb_desc.collide_with = vec![
            CollisionGroup::new(CollisionGroupType::Player.into()),
            CollisionGroup::new(CollisionGroupType::NPC.into()),
            CollisionGroup::new(CollisionGroupType::Collectable.into()),
            CollisionGroup::new(CollisionGroupType::Enemy.into()),
            CollisionGroup::new(CollisionGroupType::LinearMovable.into()),
        ];
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };

    world
        .create_entity()
        .with(cube.0)
        .with(cube.1)
        .with(rb)
        .build();
}


pub fn load_forest(world: &mut World){
    let distances:Vec<f32> = vec![-1500.,-1400.,-1300.,-1000.,-900.,0.0,80.];
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
            if i != 6{
                transform.adjust_to_distance(-distances[i], 1920.,360.);
                transform.set_translation_xyz(960., 180., distances[i]);
            } else{
                //Hardcoded as fuck due to imprecision of adjust_to_distance()
                transform.set_translation_xyz(960., 230., distances[i]);
            }

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
        sprite_sheet_list.get(AssetType::Platforms).unwrap().clone()
    };


    //---------------------------
    //Platform

    //sprite
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


    //hit box
    let cube = create_cube(init_x,init_y - platform_height,0.,platform_width + column_width*2.,platform_height,100.,world);

    let rb = {
        let mut rb_desc = RigidBodyDesc::default();
        rb_desc.mode = BodyMode::Static;
        rb_desc.friction = 0.5;
        rb_desc.bounciness = 0.00;
        rb_desc.belong_to = vec![CollisionGroup::new(CollisionGroupType::Ground.into())];
        rb_desc.collide_with = vec![CollisionGroup::new(CollisionGroupType::Player.into()),
                                    CollisionGroup::new(CollisionGroupType::NPC.into()),
                                    CollisionGroup::new(CollisionGroupType::Collectable.into()),
                                    CollisionGroup::new(CollisionGroupType::Enemy.into()),];
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };

    world
        .create_entity()
        .with(cube.0)
        .with(cube.1)
        .with(rb)
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

    let wall_width = 50.;
    let wall_height = 200_f32;
    //LEFT WALL
    load_invisible_wall(init_x - wall_width, init_y,wall_width,wall_height,world);
    //RIGHT WALL
    load_invisible_wall(init_x + column_width*2. + platform_width, init_y,wall_width,wall_height,world);

}

fn load_invisible_wall(init_x: f32, init_y: f32, wall_width: f32, wall_height: f32, world: &mut World){
    let cube = create_cube(init_x,init_y,0.,wall_width,wall_height,100.,world);

    let rb = {
        let mut rb_desc = RigidBodyDesc::default();
        rb_desc.mode = BodyMode::Static;
        rb_desc.friction = 1.0;
        rb_desc.bounciness = 1.00;
        rb_desc.belong_to = vec![CollisionGroup::new(CollisionGroupType::InvisibleWall.into())];
        rb_desc.collide_with = vec![
            CollisionGroup::new(CollisionGroupType::Enemy.into()),
            CollisionGroup::new(CollisionGroupType::Ground.into()),
            CollisionGroup::new(CollisionGroupType::LinearMovable.into()),
        ];
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };

    world
        .create_entity()
        .with(cube.0)
        .with(cube.1)
        .with(rb)
        .build();
}

fn load_moving_platform(init_x: f32, init_y: f32, distance: f32, speed: f32, world: &mut World){
    let platform_width = 150 as f32;
    let platform_height = 14 as f32;
    let sprite_sheet_handle = {
        let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
        sprite_sheet_list.get(AssetType::Platforms).unwrap().clone()
    };

    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 2,
    };

    let cube = create_cube(init_x,init_y,-1.,platform_width, platform_height,100.,world);

    let rb = {
        let mut rb_desc = RigidBodyDesc::default();
        rb_desc.mode = BodyMode::Dynamic;
        rb_desc.friction = 1.0;
        rb_desc.bounciness = 0.00;
        rb_desc.lock_translation_z = true;
        rb_desc.lock_rotation_x = true;
        rb_desc.lock_rotation_y = true;
        rb_desc.lock_rotation_z = true;
        rb_desc.belong_to = vec![
            CollisionGroup::new(CollisionGroupType::Ground.into()),
            CollisionGroup::new(CollisionGroupType::LinearMovable.into()),
        ];
        rb_desc.collide_with = vec![
            CollisionGroup::new(CollisionGroupType::Player.into()),
            CollisionGroup::new(CollisionGroupType::NPC.into()),
            CollisionGroup::new(CollisionGroupType::InvisibleWall.into()),
            CollisionGroup::new(CollisionGroupType::Enemy.into()),
            CollisionGroup::new(CollisionGroupType::Collectable.into()),
            CollisionGroup::new(CollisionGroupType::WorldWall.into()),
        ];
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };


    let mut desc = PhysicsBodyDescription::new(100.,speed);
    desc.set_velocity_direction_x(-1.);
    world
        .create_entity()
        .with(rb)
        .with(cube.0)
        .with(cube.1)
        .with(sprite.clone())
        .with(desc)
        .build();

    //LEFT
    load_invisible_wall(init_x - 20.,init_y,10.,platform_height,world);
    //RIGHT
    load_invisible_wall(init_x + distance,init_y,10.,platform_height,world);
    //BOTTOM (god forgive me)
    load_invisible_wall(init_x ,init_y - platform_height,platform_height+distance,platform_height,world);
}

fn load_coin(init_x: f32, init_y: f32, world: &mut World){
    let coin_width = 15 as f32;
    let coin_height = 15 as f32;
    let sprite_sheet_handle = {
        let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
        sprite_sheet_list.get(AssetType::Collectables).unwrap().clone()
    };

    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    let cube = create_cube(init_x,init_y,0.,coin_width, coin_height,50.,world);

    let rb = {
        let mut rb_desc = RigidBodyDesc::default();
        rb_desc.mode = BodyMode::Dynamic;
        rb_desc.friction = 1.0;
        rb_desc.bounciness = 0.00;
        rb_desc.lock_translation_z = true;
        rb_desc.lock_rotation_x = true;
        rb_desc.lock_rotation_y = true;
        rb_desc.lock_rotation_z = true;
        rb_desc.belong_to = vec![
            CollisionGroup::new(CollisionGroupType::Collectable.into()),
        ];
        rb_desc.collide_with = vec![
            CollisionGroup::new(CollisionGroupType::Player.into()),
            CollisionGroup::new(CollisionGroupType::Ground.into()),
            //dunno why, just in case
            CollisionGroup::new(CollisionGroupType::WorldWall.into()),
        ];
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };

    world
        .create_entity()
        .with(rb)
        .with(cube.0)
        .with(cube.1)
        .with(sprite.clone())
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
    load_coin(450.,Height::Low.into(),world);
    load_platform(600.,Height::Mid.into(),120.,world);
    load_platform(800.,Height::High.into(),100.,world);
    load_moving_platform(1000.,Height::Low.into(),500.,200., world);
}
