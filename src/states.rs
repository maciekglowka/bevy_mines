use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MainState {
    Init,
    MainMenu,
    Game
}
