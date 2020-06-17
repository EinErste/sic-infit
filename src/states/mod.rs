//!Mod that stores all of the states of the game we have
mod gameplay;
mod pause;
mod loading;
mod starting;
mod finish;
pub use self::{
    finish::FinishState,
    starting::StartState,
    gameplay::{GameplayState,GameplayStateType,GameplayStateTypes},
    pause::PauseState,
    loading::LoadingState
};