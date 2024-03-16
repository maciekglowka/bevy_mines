use bevy::prelude::*;

use crate::states::MainState;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::MainMenu), draw_dummy_text);
    }
}

fn draw_dummy_text(
    mut commands: Commands,
    assets: Res<crate::assets::GraphicsAssets>
) {
    println!("Text");
    let style = TextStyle {
        font: assets.font.clone(),
        font_size: 32.,
        color: Color::WHITE
    };
    commands.spawn(
        Text2dBundle {
            text: Text::from_section(
                "Mines",
                style
            ),
            // transform: Transform::from_translation(Vec::splat(0.)),
            ..Default::default()
        }
    );
}
