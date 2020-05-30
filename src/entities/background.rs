use amethyst::prelude::{World, WorldExt, Builder};
use amethyst::core::transform::Transform;
use amethyst::renderer::{SpriteRender};
use amethyst::ecs::Entity;
use crate::resources::{SpriteSheetList, AssetType};
use crate::components::{Motion, Parallax};
use amethyst_physics::prelude::{ShapeDesc, RigidBodyDesc, BodyMode};
use amethyst_physics::servers::PhysicsWorld;
use amethyst::core::math::Vector3;

pub fn load_background_forest(world: &mut World){
    let distances:Vec<f32> = vec![-0.8,-0.7,-0.6,-0.5,-0.4,-0.3,0.0,0.2];
    let speed_ratio:Vec<f32> = vec![0.7,0.6,0.5,0.4,0.3,0.1,0.0,0.1];
    let sprite_sheet_handle = {
        let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
        sprite_sheet_list.get(AssetType::BackgroundForest).unwrap().clone()
    };

    for i in 0..8 {
        let sprite = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: i,
        };
        let mut transform = Transform::default();
        transform.set_translation_xyz(960., 180., distances[i]);

        let shape = {
            let desc = ShapeDesc::Cube {half_extents: Vector3::new(960.,1.,2.)};
            let physics_world = world.fetch::<PhysicsWorld<f32>>();
            physics_world.shape_server().create(&desc)
        };

        let rb = {
            let mut rb_desc = RigidBodyDesc::default();
            if i==6 {
                rb_desc.mode = BodyMode::Static;
            } else{
                rb_desc.mode = BodyMode::Disabled;
            }
            let physics_world = world.fetch::<PhysicsWorld<f32>>();
            physics_world.rigid_body_server().create(&rb_desc)
        };

        world
            .create_entity()
            .with(sprite.clone())
            .with(transform)
            .with(Motion::default())
            .with(Parallax::new(speed_ratio[i],0.))
            .with(shape)
            .with(rb)
            .build();
    }
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