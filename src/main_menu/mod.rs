use bevy::prelude::*;

use crate::states::MainState;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::MainMenu), draw_dummy_text)
            .add_systems(Update, (
                handle_input
            ).run_if(in_state(MainState::MainMenu)))
            .add_systems(OnExit(MainState::MainMenu), clear_dummy_text);
    }
}

fn draw_dummy_text(
    mut commands: Commands,
    assets: Res<crate::assets::GraphicsAssets>
) {
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

fn handle_input(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut state: ResMut<NextState<MainState>>
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        state.set(MainState::Game);
    }
}

fn clear_dummy_text(
    mut commands: Commands,
    query: Query<Entity, With<Text>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
