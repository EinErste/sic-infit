use amethyst::prelude::{World, Builder, WithNamed, WorldExt};
use amethyst::core::transform::Transform;
use amethyst::ecs::Entity;
use amethyst::renderer::camera::Camera;
///Function that creates a camera which follows player
pub fn init_camera(world: &mut World) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(640., 360., 550.);
    world
        .create_entity()
        .with(Camera::standard_3d(640., 360.))
        .with(transform)
        .named("camera")
        .build()
}
