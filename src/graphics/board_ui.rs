use bevy::prelude::*;

use crate::board::BoardClickEvent;
use super::world_to_tile;

pub fn tile_click(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut ev: EventWriter<BoardClickEvent>
) {
    if let Some(touch) = touches.first_pressed_position() {
        if let Some(tile) = screen_to_world(touch, &camera_query) {
            ev.send(BoardClickEvent(tile));
        }
    }
    
    if !mouse_buttons.just_pressed(MouseButton::Left) { return };
    if let Some(tile) = mouse_to_tile(&camera_query, &window_query) {
        ev.send(BoardClickEvent(tile));
    }
}

// fn mouse_to_world(
//     camera_query: &Query<(&Camera, &GlobalTransform)>,
//     window_query: &Query<&Window>
// ) -> Option<Vec2> {
//     let (camera, transform) = camera_query.single();
//     let cursor = window_query.single().cursor_position()?;
//     camera.viewport_to_world_2d(transform, cursor)
// }

fn mouse_to_tile(
    camera_query: &Query<(&Camera, &GlobalTransform)>,
    window_query: &Query<&Window> 
) -> Option<IVec2> {
    let cursor = window_query.single().cursor_position()?;
    screen_to_world(cursor, camera_query)
}

fn screen_to_world(
    v: Vec2,
    camera_query: &Query<(&Camera, &GlobalTransform)>
) -> Option<IVec2> {
    let (camera, transform) = camera_query.single();
    info!("BEVY: v {:?}", v);
    let w = camera.viewport_to_world_2d(transform, v)?;
    info!("BEVY: w {:?}", w);
    Some(world_to_tile(w))
}
