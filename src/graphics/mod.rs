use bevy::prelude::*;

use crate::{
    board::components::{Position, Tile},
    globals::{TILE_SIZE, SPRITE_GRID_SIZE},
    states::MainState
};

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate, (
                spawn_tile_sprites
            ).run_if(in_state(MainState::Game))
        );
    }
}

pub fn tile_to_world(v: IVec2) -> Vec3 {
    Vec3::new(
        v.x as f32 * TILE_SIZE,
        v.y as f32 * TILE_SIZE,
        0.
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
                        index: 1
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
