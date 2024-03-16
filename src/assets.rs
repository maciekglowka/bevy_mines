use bevy::prelude::*;

#[derive(Resource)]
pub struct GraphicsAssets {
    pub font: Handle<Font>
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    let font = asset_server.load("ui/04B_03.TTF");

    commands.insert_resource(GraphicsAssets {
        font
    });

    // let texture = asset_server.load("creatures.png");
    // let layout = TextureAtlasLayout::from_grid(
    //     Vec2::splat(16.),
    //     4,
    //     4,
    //     None,
    //     None
    // );
    // let atlas_layout = layouts.add(layout);
    // commands.spawn((
    //     SpriteSheetBundle {
    //         texture,
    //         atlas: TextureAtlas {
    //             layout: atlas_layout,
    //             index: 0
    //         },
    //         transform: Transform::from_scale(Vec3::splat(8.)),
    //         ..Default::default()
    //     },
    //     FrameData(0, 4)
    // ));
}
