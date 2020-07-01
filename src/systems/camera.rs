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
        let char = transforms.get(self.character).unwrap().translation().clone();
        let cam = transforms.get_mut(self.camera).unwrap();
        if char.x<600. {
            cam.translation_mut().x = 600.;
        } else if char.x>3240.{
            cam.translation_mut().x = 3240.;
        }
        else{
            cam.translation_mut().x = char.x;
        }
        if char.y < 302.{
            cam.translation_mut().y = 302.;
        } else{
            cam.translation_mut().y = char.y;
        }
    }
}