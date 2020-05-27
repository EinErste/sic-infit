use amethyst::prelude::{World, WorldExt, Builder, WithNamed};
use amethyst::core::transform::Transform;
use amethyst::renderer::SpriteRender;
use amethyst::ecs::Entity;
use crate::resources::{SpriteSheetList, AssetType};
use crate::components::{Motion, Direction, SimpleAnimation, Directions, StateAnimation};
use enum_map::{enum_map};

pub fn load_character(world: &mut World) -> Entity{
    let sprite_sheet_handle = {
        let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
        sprite_sheet_list.get(AssetType::Character).unwrap().clone()
    };
    let transform =
        Transform::default().set_translation_xyz(320., 100., 1.).to_owned();
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };
    world
        .create_entity()
        .with(sprite)
        .with(transform)
        .with(Motion::new())
        .with(Direction{dir: Directions::Right})
        .named("character")
        .with(SimpleAnimation::new(StateAnimation::Idle,enum_map!(
            StateAnimation::Run => (2,10,0.1),
            StateAnimation::Idle => (0,2,0.8),
            _ => (0,1,0.1)
        )))
        .build()
}