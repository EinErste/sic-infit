//! Mod for storing utility functions
#[derive(PartialEq)]
pub enum CurrentState {
    Running,
    Paused,
}
impl Default for CurrentState {
    fn default() -> Self {
        CurrentState::Paused
    }
}