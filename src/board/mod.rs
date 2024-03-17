use bevy::prelude::*;
use rand::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::{
    globals::{BOARD_SIZE, TRAPS},
    states::MainState
};

pub mod components;

use components::{Covered, Position, Tile, Trap};

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BoardClickEvent>()
            .add_systems(
                OnEnter(MainState::Game), (spawn_tiles, populate_tiles).chain()
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
                    Position(v),
                    Covered,
                    Tile
                ))
                .id();
            tiles.insert(v, tile);
        }
    }
    commands.insert_resource(Board { tiles });
}

fn populate_tiles(
    mut commands: Commands,
    board: Res<Board>
) {
    let mut rng = thread_rng();
    let entities = board.tiles.values().choose_multiple(&mut rng, TRAPS);
    for entity in entities {
        commands.entity(*entity).insert(Trap(1));
    }
}

fn uncover_tiles(
    mut commands: Commands,
    mut ev_reader: EventReader<BoardClickEvent>,
    board: Res<Board>,
    query: Query<(&Position, Option<&Trap>), With<Tile>>
) {
    let mut to_uncover = ev_reader.read()
        .map(|ev| ev.0)
        .filter(|v| board.tiles.get(v).is_some())
        .collect::<VecDeque<_>>();

    let mut visited = HashSet::new();
    while let Some(v) = to_uncover.pop_front() {
        visited.insert(v);
        let Some(&entity) = board.tiles.get(&v) else { continue };
        commands.entity(entity).remove::<components::Covered>();
        if let Ok((_, t)) = query.get(entity) {
            if t.is_some() { continue };
        }
        if get_surrounding_trap_count(v, &board, &query) == 0 {
            for n in get_neighbouring_vecs(v, &board) {
                if !visited.contains(&n) { to_uncover.push_back(n) }
            }
        }
    }
}

pub fn get_neighbouring_vecs(v: IVec2, board: &Board) -> impl Iterator<Item=IVec2> + '_ {
    (-1..=1).map(move |x| (-1..=1).map(move |y|
            v + IVec2::new(x, y)
        ))
        .flatten()
        .filter(|a| board.tiles.contains_key(a))
}

pub fn get_neighbours(v: IVec2, board: &Board) -> impl Iterator<Item=Entity> + '_ {
    get_neighbouring_vecs(v, board)
        .filter_map(|n| board.tiles.get(&n))
        .copied()
}

pub fn get_surrounding_trap_count(
    v: IVec2,
    board: &Board,
    query: &Query<(&Position, Option<&Trap>), With<Tile>>
) -> usize {
    get_neighbours(v, board)
        .filter_map(|e| query.get(e).ok())
        .filter(|t| t.1.is_some())
        .count()
}
