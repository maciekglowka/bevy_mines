use bevy::prelude::*;

use crate::{
    globals::BOARD_SIZE,
    states::MainState
};

pub mod components;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
                OnEnter(MainState::Game), spawn_tiles
            );
    }
}

fn spawn_tiles(
    mut commands: Commands,
) {
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            let v = IVec2::new(x as i32, y as i32);
            commands.spawn((
                components::Position(v),
                components::Covered,
                components::Tile
            ));
        }
    }
}
