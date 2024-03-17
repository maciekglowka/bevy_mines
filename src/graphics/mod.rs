use bevy::prelude::*;

use crate::{
    board::components::{Covered, Position, Tile},
    globals::{BOARD_SIZE, TILE_SIZE, SPRITE_GRID_SIZE},
    states::MainState
};

mod board_ui;

const BOARD_OFFSET: f32 = -0.5 * TILE_SIZE * BOARD_SIZE as f32 + 0.5 * TILE_SIZE;

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
                PostUpdate, (
                    spawn_tile_sprites,
                    uncover_tile_sprites
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

fn spawn_tile_sprites(
    mut commands: Commands,
    assets: Res<crate::assets::GraphicsAssets>,
    tile_query: Query<(Entity, &Position), (Added<Position>, With<Tile>)>
) {
    let size = TILE_SIZE / SPRITE_GRID_SIZE;

    for (entity, position) in tile_query.iter() {
        commands.entity(entity)
            .insert(
                SpriteSheetBundle {
                    texture: assets.tile_texture.clone(),
                    atlas: TextureAtlas {
                        layout: assets.tile_atlas.clone(),
                        index: 0
                    },
                    transform: Transform::from_translation(
                            tile_to_world(position.0)
                        )
                        .with_scale(Vec3::new(size, size, 1.)),
                    ..Default::default()
                },
            );

    }
}

fn uncover_tile_sprites(
    mut removals: RemovedComponents<Covered>,
    mut tile_query: Query<&mut TextureAtlas>
) {
    for entity in removals.read() {
        if let Ok(mut atlas) = tile_query.get_mut(entity) {
            atlas.index = 1;
        }
    }
}
