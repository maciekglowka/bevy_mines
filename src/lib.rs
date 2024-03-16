use bevy::{
    prelude::*,
    window::{ApplicationLifetime, WindowMode},
};

mod assets;
mod main_menu;
mod states;

#[bevy_main]
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.75, 0.25, 0.5)))
        .add_plugins(DefaultPlugins.set(
                ImagePlugin::default_nearest()
            )
            // .set(WindowPlugin {
            //     primary_window: Some(Window {
            //         resizable: false,
            //         mode: WindowMode::BorderlessFullscreen,
            //         ..Default::default()
            //     }),
            //     ..Default::default()
            // })
        )
        .insert_resource(Msaa::Off)
        .insert_state(states::MainState::MainMenu)
        .add_systems(Startup, setup)
        .add_systems(Startup, assets::load_assets)
        .add_plugins(main_menu::MainMenuPlugin)
        // .add_systems(Update, frames)
        .run();
}

// fn frames(
//     time: Res<Time>,
//     mut timer_query: Query<&mut FrameTimer>,
//     mut atlas_query: Query<(&mut TextureAtlas, &mut FrameData)>
// ) {
//     if let Ok(mut timer) = timer_query.get_single_mut() {
//         timer.0.tick(time.delta());
//         if timer.0.just_finished() {
//             for (mut sprite, mut frame) in atlas_query.iter_mut() {
//                 frame.0 = (frame.0 + 1) % frame.1;
//                 sprite.index = frame.0;
//             }
//         }
//     }
// }

fn setup(mut commands: Commands, mut state: ResMut<NextState<states::MainState>>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(FrameTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));
    // state.set(states::MainState::MainMenu);
}

#[derive(Component)]
struct FrameTimer(Timer);

#[derive(Component)]
struct FrameData(usize, usize);
