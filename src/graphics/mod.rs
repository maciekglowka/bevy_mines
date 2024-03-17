use bevy::prelude::*;

use crate::{
    board::components::{Covered, Position, Tile},
    globals::{BOARD_SIZE, TILE_SIZE, SPRITE_GRID_SIZE},
    states::MainState
};

mod board_ui;
mod tiles;

const BOARD_OFFSET: f32 = -0.5 * TILE_SIZE * BOARD_SIZE as f32 + 0.5 * TILE_SIZE;

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
                PostUpdate, (
                    tiles::spawn_tile_sprites,
                    tiles::uncover_tile_sprites.pipe(tiles::draw_tile_bodies)
                ).run_if(in_state(MainState::Game))
            )
            .add_systems(
                Update, (
                    board_ui::tile_click
                ).run_if(in_state(MainState::Game))
            );
    }
}

pub fn tile_to_world(v: IVec2) -> Vec3 {
    Vec3::new(
        v.x as f32 * TILE_SIZE + BOARD_OFFSET,
        v.y as f32 * TILE_SIZE + BOARD_OFFSET,
        0.
    )
}

pub fn world_to_tile(v: Vec2) -> IVec2 {
    IVec2::new(
        ((v.x - BOARD_OFFSET) / TILE_SIZE).round() as i32,
        ((v.y - BOARD_OFFSET) / TILE_SIZE).round() as i32,
    )
}
