use amethyst::{
    ecs::{Component, DenseVecStorage}
};
use enum_map::EnumMap;
use enum_map::Enum;

#[derive(Component)]
#[storage(DenseVecStorage)]
///Component responsible for animation. It loops through a set of states through time.
pub struct SimpleAnimation{
    pub current_state: StateAnimation,
    pub time_elapsed: f32,
    pub state_changed: bool,
    pub states: EnumMap<StateAnimation,(usize,usize,f32)>
}

#[derive(Enum,Copy, Clone,PartialEq)]
///Enum for player animation states
pub enum StateAnimation{
    Static,
    Idle,
    Run,
}


impl SimpleAnimation {
    pub fn new(current_state: StateAnimation, states: EnumMap<StateAnimation,(usize,usize,f32)>) -> SimpleAnimation
    {
        SimpleAnimation {
            current_state,
            time_elapsed: 0.0,
            state_changed: false,
            states
        }
    }

    pub fn change_state(&mut self, new_state: StateAnimation){

        if new_state!=self.current_state {
            self.time_elapsed = 0.0;
            self.current_state = new_state;
        }

    }
}

