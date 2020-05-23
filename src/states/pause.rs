use amethyst::{input::{get_key, is_close_requested, is_key_down, VirtualKeyCode}, SimpleState, StateData, GameData, StateEvent, SimpleTrans, Trans};


pub struct PauseState;
impl SimpleState for PauseState {
    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::P) {
                return Trans::Pop;
            }
        }
        Trans::None
    }
}
