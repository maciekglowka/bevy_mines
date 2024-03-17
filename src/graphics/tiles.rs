use bevy::prelude::*;

use crate::{
    board::components::{Covered, Position, Tile, Trap},
    globals::{TILE_SIZE, SPRITE_GRID_SIZE},
};

pub(crate) fn spawn_tile_sprites(
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
                            super::tile_to_world(position.0)
                        )
                        .with_scale(Vec3::new(size, size, 1.)),
                    ..Default::default()
                },
            );

    }
}

pub(crate) fn uncover_tile_sprites(
    mut removals: RemovedComponents<Covered>,
    mut tile_query: Query<&mut TextureAtlas>
) -> Vec<Entity> {
    let mut output = Vec::new();
    for entity in removals.read() {
        if let Ok(mut atlas) = tile_query.get_mut(entity) {
            atlas.index = 1;
            output.push(entity);
        }
    }
    output
}

pub(crate) fn draw_tile_bodies(
    In(entities): In<Vec<Entity>>,
    mut commands: Commands,
    assets: Res<crate::assets::GraphicsAssets>,
    board: Res<crate::board::Board>,
    query: Query<(&Position, Option<&Trap>), With<Tile>>
) {
    let style = TextStyle {
        font: assets.font.clone(),
        font_size: 8. * TILE_SIZE,
        color: Color::BLACK
    };

    for entity in entities {
        let Ok((position, trap)) = query.get(entity) else { continue };
        let t = if trap.is_some() {
            "#".to_string()
        } else {
            let n = crate::board::get_surrounding_trap_count(position.0, &board, &query);
            if n == 0 { continue };
            format!("{}", n)
        };
        let inner = commands.spawn(Text2dBundle {
                text: Text::from_section(
                    t,
                    style.clone()
                ),
                transform: Transform::from_scale(Vec3::new(1./64., 1./64., 1.))
                    .with_translation(Vec3::new(0., 0., 1.)),
                ..Default::default()
            })
            .id();
        commands.entity(entity).add_child(inner);
    }
}


