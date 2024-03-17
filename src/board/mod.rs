use bevy::prelude::*;
use std::collections::HashMap;

use crate::{
    globals::BOARD_SIZE,
    states::MainState
};

pub mod components;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BoardClickEvent>()
            .add_systems(
                OnEnter(MainState::Game), spawn_tiles
            )
            .add_systems(Update, uncover_tiles.run_if(in_state(MainState::Game)));
    }
}

#[derive(Event)]
pub struct BoardClickEvent(pub IVec2);

#[derive(Resource)]
pub struct Board {
    pub tiles: HashMap<IVec2, Entity>
}

fn spawn_tiles(
    mut commands: Commands,
) {
    let mut tiles = HashMap::new();
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            let v = IVec2::new(x as i32, y as i32);
            let tile = commands.spawn((
                    components::Position(v),
                    components::Covered,
                    components::Tile
                ))
                .id();
            tiles.insert(v, tile);
        }
    }
    commands.insert_resource(Board { tiles });
}

fn uncover_tiles(
    mut commands: Commands,
    mut ev_reader: EventReader<BoardClickEvent>,
    board: Res<Board>,
) {
    for ev in ev_reader.read() {
        let Some(&entity) = board.tiles.get(&ev.0) else { continue };
        commands.entity(entity).remove::<components::Covered>();
    }
}
