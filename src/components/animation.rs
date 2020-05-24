use amethyst::{
    ecs::{Component, DenseVecStorage}
};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct SimpleAnimation{
    pub start_sprite_index: usize,
    pub frames: usize,
    pub current_frame: usize,
    pub time_per_frame: f32,
    pub elapsed_time: f32
}


impl SimpleAnimation {
    pub fn new(start_sprite_index: usize, frames: usize, time_per_frame: f32) -> SimpleAnimation
    {
        SimpleAnimation {
            start_sprite_index,
            frames,
            current_frame: 0,
            time_per_frame,
            elapsed_time: 0.0,
        }
    }
}