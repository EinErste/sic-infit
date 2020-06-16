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
use crate::entities::background::Latitude::WorldStart;


//All x and y parameters stands for left bottom point
pub fn load_forest_path(init_x: f32, init_y: f32, ground_width: f32,ground_height: f32, ground_depth: f32,world: &mut World){

    //Main path
    let cube = create_cube(init_x,init_y - ground_height,0.,ground_width,ground_height,ground_depth,world);

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

    let cube = create_cube(init_x,init_y - ground_height -500.,0.,ground_width,ground_height*3.,ground_depth,world);

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

}

fn load_world_wall(init_x: f32, init_y: f32,world: &mut World) {

    let cube = create_cube(init_x,init_y,0.,320.,2000.,100.,world);

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
    //Main sprites
    let width = 3840.;
    let height = 360.;
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
                transform.adjust_to_distance(-distances[i], width,height);
                transform.set_translation_xyz(width/2., height/2., distances[i]);
            } else{
                //Hardcoded as fuck due to imprecision of adjust_to_distance()
                transform.set_translation_xyz(width/2., 230., distances[i]);
            }

            world
                .create_entity()
                .with(sprite.clone())
                .with(transform)
                .build();

        }
    }
    load_forest_path(Latitude::WorldStart.into(),Altitude::Ground.into(),width,65.,100.,world);
    load_world_wall(Latitude::WorldStart.into(),Altitude::Zero.into(),world);
    load_world_wall(Latitude::WorldEnd as u32 as f32 - 48.,Altitude::Zero.into(),world);
    load_obstacles(world);
}



fn load_platform(init_x: f32, init_y: f32, init_z: f32, platform_width: f32, world: &mut World){
    let column_width = 50 as f32;
    let column_height = 1080 as f32;
    let platform_height = 12 as f32;
    let platform_init_width = 70 as f32;
    let sprite_sheet_handle = {
        let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
        sprite_sheet_list.get(AssetType::Obstacles).unwrap().clone()
    };


    //---------------------------
    //Platform


    //hit box
    let cube = create_cube(init_x,init_y - platform_height*2.,0.,platform_width + column_width*2.,platform_height,100.,world);

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

    //sprite
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 1,
    };
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(platform_width/platform_init_width,1.,1.));
    transform.set_translation_xyz(init_x + column_width + platform_width/2., init_y-platform_height/2. - platform_height, -0.3 + init_z);
    world
        .create_entity()
        .with(sprite.clone())
        .with(transform)
        .build();



    //---------------------------
    //Columns

    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };
    let mut transform = Transform::default();
    transform.set_translation_xyz(init_x, -(column_height-init_y + platform_height), -0.1 + init_z);

    world
        .create_entity()
        .with(sprite.clone())
        .with(transform)
        .build();

    let mut transform = Transform::default();
    transform.set_translation_xyz(init_x+platform_width+column_width, -(column_height-init_y +platform_height), -0.1 + init_z);

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
    load_invisible_area(init_x - wall_width, init_y - platform_height,wall_width,wall_height,world);
    //RIGHT TURN AREA
    load_invisible_area(init_x + column_width*2. + platform_width, init_y - platform_height,wall_width,wall_height,world);

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

    let cube = create_cube(init_x,init_y-platform_height,0.08,platform_width, platform_height,80.,world);

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
    load_invisible_area(init_x - 50.,init_y-platform_height,5.,platform_height,world);
    //RIGHT TURN AREA
    load_invisible_area(init_x + distance,init_y - platform_height,5.,platform_height,world);
    //BOTTOM SUPPORT
    //width +platform_width?
    load_support_ground(init_x,init_y-platform_height *2.,0.,distance,10.,100.,world);
}

fn load_moving_platform_y(init_x: f32, init_y: f32, distance: f32, speed: f32, world: &mut World){

    let (platform_width,platform_height) = load_moving_platform(init_x,init_y,speed,(0.,1.),world);

    //UP TURN AREA
    load_invisible_area(init_x,init_y + platform_height + distance,platform_width,5.,world);
    //DOWN TURN AREA
    load_invisible_area(init_x,init_y - platform_height*2.,platform_width,5.,world);

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

    let cube = create_cube(init_x,init_y,0.09,coin_width, coin_height,50.,world);

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

fn  load_exit(world: &mut World) {
    let cave_width = 320.;
    let cave_height = 540.;
    let sprite_sheet_handle = {
        let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
        sprite_sheet_list.get(AssetType::EndStructure).unwrap().clone()
    };

    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    let cube = create_cube(Latitude::WorldEnd as u32 as f32 - cave_width,Altitude::Ground as u32 as f32 - 60.,0.13,cave_width,cave_height,100.,world);

    let rb = {
        let mut rb_desc = RigidBodyDesc::default();
        rb_desc.mode = BodyMode::Static;
        rb_desc.friction = 0.0;
        rb_desc.bounciness = 0.00;
        rb_desc.belong_to = vec![
            CollisionGroup::new(CollisionGroupType::Exit.into()),
        ];
        rb_desc.collide_with = vec![
            CollisionGroup::new(CollisionGroupType::Player.into()),
        ];
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };

    world
        .create_entity()
        .with(cube.0)
        .with(cube.1)
        .with(sprite.clone())
        .with(rb)
        .build();
}

//For ground entities - top point
//For other entities - bottom point
#[repr(u32)]
pub enum Altitude {
    Zero = 0,
    Ground = 65,
    Low = 160,
    Mid = 240,
    High = 320,
}

#[repr(u32)]
pub enum Latitude {
    WorldStart = 0,
    WorldEnd = 3840
}

impl Into<f32> for Altitude{
    fn into(self) -> f32 {
        self as u32 as f32
    }
}

impl Into<f32> for Latitude{
    fn into(self) -> f32 {
        self as u32 as f32
    }
}


fn load_obstacles(world: &mut World){
    let (d1,d2,d3,d4,d5,d6) = (-0.3,-0.6,-0.9,-1.2,-1.5,-1.8);
    let ground: f32 = Altitude::Ground.into();
    let low: f32 = Altitude::Low.into();
    let mid: f32 = Altitude::Mid.into();
    let high: f32 = Altitude::High.into();

    //1
    load_platform(380.,mid*3.,d6,100.,world);
    //15
    load_moving_platform_y(650.,low,mid * 3. - low,150., world);
    //4
    load_platform(850.,high,d3,150.,world);
    //2
    load_platform(1000.,low,d1,100.,world);
    //6
    load_platform(1250.,high*2.,d6,350.,world);
    //3
    load_platform(1500.,mid,d2,200.,world);
    //17
    load_platform(1850.,low,d1,100.,world);
    //8
    load_platform(1900.,high,d3,400.,world);
    //20
    load_platform(2150.,mid*2. - ground,d4,50.,world);
    //7
    load_platform(1300.,mid*2.,d4,700.,world);
    //14
    load_moving_platform_y(2500.,low,mid * 3. - low,100., world);
    //9
    load_platform(2800.,mid*2.,d4,200.,world);
    //10
    load_platform(2700.,mid*3.,d6,400.,world);
    //13
    load_moving_platform_x(1200.,high,300.,100., world);
    //21
    load_moving_platform_x(900.,mid*2.,325.,100., world);
    //22
    load_moving_platform_x(900.,high*2.,300.,100., world);
    //25
    load_moving_platform_y(1700.,mid*2. + ground,high*2. - mid*2.,100., world);
    //12
    load_moving_platform_x(1950.,high*2.,500.,100., world);

    //load_enemy(620.,Altitude::Low.into(),world);
    //load_coin(650.,Altitude::Low.into(),world);
    load_npc(400.,Altitude::Ground.into(),world);

    load_exit(world);
}
