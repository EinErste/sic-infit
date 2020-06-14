use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Entity, System, SystemData, WriteStorage, Read, Write, World, ReaderId},
    shrev::EventChannel,
    ui::UiText
};

use crate::entities::CoinSign;
///This system controls the camera and ties it to a character at al
pub struct CoinPickupSystem {
    reader_id: ReaderId<CoinPicked>,
}

impl CoinPickupSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let reader_id = world.fetch_mut::<EventChannel<CoinPicked>>().register_reader();
        Self { reader_id }
    }
}

pub struct CoinPicked();

impl<'s> System<'s> for CoinPickupSystem {
    type SystemData = (
        Read<'s, EventChannel<CoinPicked>>,
        Write<'s, CoinSign>,
        WriteStorage<'s, UiText>,
    );

    fn run(&mut self, (coinChannel, coinSign, mut uiText): Self::SystemData) {
        for coinEvent in coinChannel.read(&mut self.reader_id) {
            let entity = coinSign.0.unwrap();

            if let Some(text) = uiText.get_mut(entity) {
                text.text = (text.text.parse::<i32>().unwrap() + 1).to_string();//TODO fix doubling
            }

        }
    }
}