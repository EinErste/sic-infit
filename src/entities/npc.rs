use amethyst::prelude::{World, WorldExt, Builder};
use amethyst::core::transform::Transform;
use amethyst::renderer::SpriteRender;
use amethyst::ecs::Entity;
use crate::resources::{SpriteSheetList, AssetType};
use crate::components::{Motion, Direction, SimpleAnimation, Directions, StateAnimation, Player};
use enum_map::{enum_map};
use amethyst_physics::prelude::{ShapeDesc, RigidBodyDesc, BodyMode};
use amethyst_physics::servers::PhysicsWorld;
use amethyst::core::math::Vector3;

pub fn load_player(world: &mut World) -> Entity{
    let sprite_sheet_handle = {
        let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
        sprite_sheet_list.get(AssetType::Character).unwrap().clone()
    };
    let transform =
        Transform::default().set_translation_xyz(320., 300., 1.).to_owned();
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };
    let shape = {
        let desc = ShapeDesc::Cube {half_extents: Vector3::new(24.,32.,1.)};
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.shape_server().create(&desc)
    };

    let rb = {
        let mut rb_desc = RigidBodyDesc::default();
        rb_desc.lock_translation_z = true;
        rb_desc.friction = 0.01;
        rb_desc.bounciness = 0.01;
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };
    world
        .create_entity()
        .with(sprite)
        .with(transform)
        .with(Motion::new())
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