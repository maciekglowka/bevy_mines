use bevy::prelude::*;

#[derive(Resource)]
pub struct GraphicsAssets {
    pub font: Handle<Font>,
    pub tile_texture: Handle<Image>,
    pub tile_atlas: Handle<TextureAtlasLayout>
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>
) {
    let font = asset_server.load("ui/04B_03.TTF");
    let tile_texture = asset_server.load("sprites/tiles.png");

    let layout = TextureAtlasLayout::from_grid(
        Vec2::splat(16.),
        4,
        4,
        None,
        None
    );
    let tile_atlas = texture_atlas_layout.add(layout);
    commands.insert_resource(GraphicsAssets {
        font,
        tile_atlas,
        tile_texture
    });
}
