use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Entity, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
///This system controls the camera and ties it to a character at all times
pub struct CameraSystem {
    pub(crate) character: Entity,
    pub(crate) camera: Entity,
}

impl<'s> System<'s> for CameraSystem {
    type SystemData = WriteStorage<'s, Transform>;

    fn run(&mut self, mut transforms: Self::SystemData) {
        let char_translation = transforms.get(self.character).unwrap().translation().clone();
        let camera_transform = transforms.get_mut(self.camera).unwrap();
        //TODO
        camera_transform.set_translation_xyz(char_translation[0]+250 as f32,char_translation[1]+80 as f32,300.);
        //camera_transform.translation_mut().copy_from(&char_translation);
        //camera_transform.set_translation_z(300.);
    }
}