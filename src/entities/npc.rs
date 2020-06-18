//! A set of useful functions for loading the player and associated spritess
use amethyst::{
    prelude::{World, WorldExt, Builder},
    core::transform::{Transform},
    renderer::{SpriteRender},
    ecs::{Entity},
    core::math::{Vector3},
    ui::{UiCreator, UiTransform, Anchor, UiText, TtfFormat, FontAsset},
    assets::{Loader, Handle},
};
use crate::{
    resources::{SpriteSheetList, AssetType},
    components::{Direction, SimpleAnimation, Directions, StateAnimation, Player},
};
use enum_map::{enum_map};
use amethyst_physics::{
    prelude::{ShapeDesc, RigidBodyDesc, BodyMode},
    servers::PhysicsWorld,
    objects::PhysicsHandle,
};
use crate::components::{PhysicsBodyDescription, CollisionGroupType, NPC, create_cube};
use amethyst_physics::objects::CollisionGroup;
use amethyst_physics::prelude::PhysicsShapeTag;
use crate::systems::DirectionSystem;

pub fn load_player(world: &mut World) -> Entity {
    let sprite_sheet_handle = {
        let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
        sprite_sheet_list.get(AssetType::Character).unwrap().clone()
    };
    let transform =
        Transform::default().set_translation_xyz(360., 240., 0.15).to_owned();
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };
    let shape = {
        let desc = ShapeDesc::Cube { half_extents: Vector3::new(20., 28., 5.) };
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
                                    CollisionGroup::new(CollisionGroupType::WorldWall.into()),
                                    CollisionGroup::new(CollisionGroupType::Collectable.into()),
                                    CollisionGroup::new(CollisionGroupType::Enemy.into()),
                                    CollisionGroup::new(CollisionGroupType::Prop.into()),
                                    CollisionGroup::new(CollisionGroupType::Exit.into()),
        ];
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };
    world
        .create_entity()
        .with(sprite)
        .with(transform)
        .with(PhysicsBodyDescription::new(10., 130.))
        .with(Direction { dir: Directions::Right })
        .with(Player::new())
        .with(shape)
        .with(rb)
        .with(SimpleAnimation::new(StateAnimation::Idle, enum_map!(
            StateAnimation::Run => (2,10,0.13),
            StateAnimation::Idle => (0,2,0.8),
            _ => (0,1,0.1)
        )))
        .build()
}

pub fn load_enemy(init_x: f32, init_y: f32,speed: f32, dir: Directions,world: &mut World) {
    let sprite_sheet_handle = {
        let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
        sprite_sheet_list.get(AssetType::Enemy).unwrap().clone()
    };

    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    let cube = create_cube(init_x, init_y, 0.1, 40., 60., 40., world);


    let rb = {
        let mut rb_desc = RigidBodyDesc::default();
        rb_desc.lock_translation_z = true;
        rb_desc.lock_rotation_x = true;
        rb_desc.lock_rotation_y = true;
        rb_desc.lock_rotation_z = true;
        rb_desc.friction = 0.0;
        rb_desc.bounciness = 1.0;
        rb_desc.mass = 1000.;
        rb_desc.mode = BodyMode::Dynamic;
        rb_desc.belong_to = vec![
            CollisionGroup::new(CollisionGroupType::Enemy.into()),
            CollisionGroup::new(CollisionGroupType::LinearMovable.into()),
        ];
        rb_desc.collide_with = vec![
            CollisionGroup::new(CollisionGroupType::Ground.into()),
            CollisionGroup::new(CollisionGroupType::NPC.into()),
            CollisionGroup::new(CollisionGroupType::Player.into()),
            CollisionGroup::new(CollisionGroupType::WorldWall.into()),
            CollisionGroup::new(CollisionGroupType::Prop.into()),
            CollisionGroup::new(CollisionGroupType::InvisibleArea.into()),
        ];
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };


    let mut desc = PhysicsBodyDescription::new(1000., speed);
    let vel_dir = match dir {
        Directions::Left => {-1.}
        Directions::Right => {1.}
    };
    desc.set_velocity_direction_x(vel_dir);
    let entity = world
        .create_entity()
        .with(sprite)
        .with(cube.0)
        .with(cube.1)
        .with(rb)
        .with(desc)
        .with(Direction {dir })
        .build();
}

pub  fn load_npc(init_x: f32, init_y: f32, dir: Directions, asset_type: AssetType, line: &str, world: &mut World) {
    let width = 48.;
    let height = 64.;
    let sprite_sheet_handle = {
        let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
        sprite_sheet_list.get(asset_type).unwrap().clone()
    }; //TODO change asset to real npc
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };
    let cube = create_cube(init_x,init_y,0.1,width,height,20.,world);

    let rb = {
        let mut rb_desc = RigidBodyDesc::default();
        rb_desc.mode = BodyMode::Disabled;
        rb_desc.belong_to = vec![
            CollisionGroup::new(CollisionGroupType::NPC.into())
        ];
        rb_desc.collide_with = vec![
            CollisionGroup::new(CollisionGroupType::Ground.into()),
            CollisionGroup::new(CollisionGroupType::WorldWall.into()),
        ];
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };
    world
        .create_entity()
        .with(sprite)
        .with(cube.0)
        .with(cube.1)
        .with(rb)
        .with(NPC::new(line))
        .with(Direction{dir})
        .build();
}

#[derive(Default)]
pub struct CoinSign(pub Option<Entity>);

fn load_coins(world: &mut World) {
    let font = load_font(&world);

    let transform = UiTransform::new(
        "coins".to_string(), Anchor::TopLeft, Anchor::TopLeft,
        25., 0., 1., 100., 50.,
    );

    let entity = CoinSign(Some(world
        .create_entity()
        .with(transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [0., 0., 0., 1.],
            50.,
        ))
        .build()));

    world.insert(entity)
}

#[derive(Default)]
pub struct HeartsSign(pub Option<Entity>);

fn load_hearts(world: &mut World) {
    let font = load_font(&world);

    let transform = UiTransform::new(
        "hearts".to_string(), Anchor::TopLeft, Anchor::TopLeft,
        25., -50., 1., 100., 50.,
    );

    let entity = HeartsSign(Some(world
        .create_entity()
        .with(transform)
        .with(UiText::new(
            font.clone(),
            "10".to_string(),
            [0., 0., 0., 1.],
            50.,
        ))
        .build()));
    world.insert(entity)
}

fn load_ui_imgs(world: &mut World) {
    world.exec(|mut creator: UiCreator<'_>| {
        Some(creator.create("prefabs/ui/ui_imgs.ron", ()))
    });
}

#[derive(Default)]
pub struct InteractButton(pub Option<Entity>);

fn load_interact_button(world: &mut World) {
    let font = load_font(&world);

    let transform = UiTransform::new(
        "coins".to_string(), Anchor::BottomMiddle, Anchor::BottomMiddle,
        0., 100., 1., 1600., 50.,
    );

    let entity = InteractButton(Some(world
        .create_entity()
        .with(transform)
        .with(UiText::new(
            font.clone(),
            "E".to_string(),
            [0., 0., 0., 1.],
            75.,
        ))
        .build()));
    world.insert(entity);
}

fn load_font(world: &&mut World) -> Handle<FontAsset> {
    world.read_resource::<Loader>().load(
        "font/dalek.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    )
}

pub fn load_ui(world: &mut World) {
    load_coins(world);
    load_hearts(world);
    load_ui_imgs(world);
    load_interact_button(world);
}