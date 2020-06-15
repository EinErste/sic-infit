//! A set of useful functions for loading the spritess
use amethyst::prelude::{World, WorldExt, Builder};
use amethyst::core::transform::Transform;
use amethyst::renderer::{SpriteRender};
use amethyst::ecs::Entity;
use crate::resources::{SpriteSheetList, AssetType};
use amethyst_physics::prelude::{ShapeDesc, RigidBodyDesc, BodyMode};
use amethyst_physics::servers::{PhysicsWorld, AreaDesc};
use amethyst::core::math::Vector3;
use crate::entities::{AdjustToDistance, load_enemy, load_npc};
use amethyst_physics::objects::CollisionGroup;
use crate::components::{CollisionGroupType, PhysicsBodyDescription, create_cube};


//All x and y parameters stands for left bottom point
pub fn load_forest_path(world: &mut World){

    //Main path
    let cube = create_cube(0.,40.,0.,3840.,20.,100.,world);

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

    //Delete area

    let cube = create_cube(0.,-500.,0.,3840.,200.,100.,world);

    let rb = {
        let mut rb_desc = AreaDesc::default();
        rb_desc.belong_to = vec![CollisionGroup::new(CollisionGroupType::DeleteArea.into())];
        rb_desc.collide_with = vec![
            CollisionGroup::new(CollisionGroupType::Deletable.into()),
        ];
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.area_server().create(&rb_desc)
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


pub fn load_world_forest(world: &mut World){
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
                transform.adjust_to_distance(-distances[i], 3840.,360.);
                transform.set_translation_xyz(1920., 180., distances[i]);
            } else{
                //Hardcoded as fuck due to imprecision of adjust_to_distance()
                transform.set_translation_xyz(1920., 230., distances[i]);
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
    let column_width = 50 as f32;
    let column_height = 360 as f32;
    let platform_height = 12 as f32;
    let platform_init_width = 70 as f32;
    let sprite_sheet_handle = {
        let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
        sprite_sheet_list.get(AssetType::Obstacles).unwrap().clone()
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
    transform.set_translation_xyz(init_x + column_width + platform_width/2., init_y-platform_height/2., -10.);

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
    transform.set_translation_xyz(init_x, -(column_height-init_y), -9.);

    world
        .create_entity()
        .with(sprite.clone())
        .with(transform)
        .build();

    let mut transform = Transform::default();
    transform.set_translation_xyz(init_x+platform_width+column_width, -(column_height-init_y), -9.);

    world
        .create_entity()
        .with(sprite.clone())
        .with(transform)
        .build();

    //-------------------------------
    //Invisible walls

    let wall_width = 2.;
    let wall_height = 5.;
    //LEFT TURN AREA
    load_invisible_area(init_x - wall_width, init_y,wall_width,wall_height,world);
    //RIGHT TURN AREA
    load_invisible_area(init_x + column_width*2. + platform_width, init_y,wall_width,wall_height,world);

}

fn load_invisible_area(init_x: f32, init_y: f32, wall_width: f32, wall_height: f32, world: &mut World){
    let cube = create_cube(init_x,init_y,0.,wall_width,wall_height,100.,world);

    let rb = {
        let mut rb_desc = AreaDesc::default();
        rb_desc.belong_to = vec![CollisionGroup::new(CollisionGroupType::InvisibleArea.into())];
        rb_desc.collide_with = vec![
            CollisionGroup::new(CollisionGroupType::Enemy.into()),
            CollisionGroup::new(CollisionGroupType::Ground.into()),
            CollisionGroup::new(CollisionGroupType::LinearMovable.into()),
        ];
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.area_server().create(&rb_desc)
    };

    world
        .create_entity()
        .with(cube.0)
        .with(cube.1)
        .with(rb)
        .build();
}

fn load_support_ground(init_x:f32, init_y:f32, init_z:f32, width:f32,height:f32,depth:f32,world: &mut World){
    let cube = create_cube(init_x,init_y,init_z,width,height,depth,world);

    let rb = {
        let mut rb_desc = RigidBodyDesc::default();
        rb_desc.mode = BodyMode::Static;
        rb_desc.friction = 0.0;
        rb_desc.bounciness = 0.00;
        rb_desc.mass = 100.;
        rb_desc.belong_to = vec![
            CollisionGroup::new(CollisionGroupType::SupportGround.into()),
        ];
        rb_desc.collide_with = vec![
            CollisionGroup::new(CollisionGroupType::Ground.into()),
        ];
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };

    world
        .create_entity()
        .with(rb)
        .with(cube.0)
        .with(cube.1)
        .build();
}


fn load_moving_platform(init_x: f32, init_y: f32, speed: f32, init_directions: (f32,f32), world: &mut World) -> (f32,f32){
    let platform_width = 153 as f32;
    let platform_height = 28 as f32;
    let sprite_sheet_handle = {
        let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
        sprite_sheet_list.get(AssetType::Obstacles).unwrap().clone()
    };

    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 2,
    };

    let cube = create_cube(init_x,init_y,-1.,platform_width, platform_height,80.,world);

    let rb = {
        let mut rb_desc = RigidBodyDesc::default();
        rb_desc.mode = BodyMode::Dynamic;
        rb_desc.friction = 1.0;
        rb_desc.bounciness = 0.00;
        rb_desc.mass = 100.;
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
            CollisionGroup::new(CollisionGroupType::InvisibleArea.into()),
            CollisionGroup::new(CollisionGroupType::Enemy.into()),
            CollisionGroup::new(CollisionGroupType::Collectable.into()),
            CollisionGroup::new(CollisionGroupType::WorldWall.into()),
            CollisionGroup::new(CollisionGroupType::SupportGround.into()),
        ];
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };


    let mut desc = PhysicsBodyDescription::new(100.,speed);
    desc.set_velocity_direction_x(init_directions.0);
    desc.set_velocity_direction_y(init_directions.1);
    world
        .create_entity()
        .with(rb)
        .with(cube.0)
        .with(cube.1)
        .with(sprite.clone())
        .with(desc)
        .build();
    (platform_width,platform_height)
}

fn load_moving_platform_x(init_x: f32, init_y: f32, distance: f32, speed: f32, world: &mut World){
    let (platform_width,platform_height) = load_moving_platform(init_x,init_y,speed,(-1.,0.),world);

    //LEFT TURN AREA
    load_invisible_area(init_x - 50.,init_y,50.,platform_height,world);
    //RIGHT TURN AREA
    load_invisible_area(init_x + distance,init_y,50.,platform_height,world);
    //BOTTOM SUPPORT
    load_support_ground(init_x,init_y-platform_height,0.,platform_width+distance,10.,100.,world);
}

fn load_moving_platform_y(init_x: f32, init_y: f32, distance: f32, speed: f32, world: &mut World){

    let (platform_width,platform_height) = load_moving_platform(init_x,init_y,speed,(0.,1.),world);

    //UP TURN AREA
    load_invisible_area(init_x,init_y + platform_height + distance,platform_width,platform_height,world);
    //DOWN TURN AREA
    load_invisible_area(init_x,init_y - platform_height,platform_width,platform_height,world);

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
    load_enemy(420.,Height::Low.into(),world);
    // load_enemy(450.,Height::Low.into(),world);
    // load_enemy(500.,Height::Low.into(),world);
    // load_enemy(600.,Height::Low.into(),world);
    load_platform(400.,Height::Low.into(),70.,world);
    load_coin(450.,Height::Low.into(),world);
    load_platform(600.,Height::Mid.into(),120.,world);
    load_platform(800.,Height::High.into(),400.,world);
    load_moving_platform_x(900.,Height::Low.into(),500.,200., world);
    load_moving_platform_y(1500.,Height::Low.into(),1000.,200., world);
    load_npc(world); //todo seyoha pls do it properly

}
