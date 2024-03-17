use bevy::prelude::*;

use crate::board::BoardClickEvent;
use super::world_to_tile;

pub fn tile_click(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut ev: EventWriter<BoardClickEvent>
) {
    if !mouse_buttons.just_pressed(MouseButton::Left) { return };
    let Some(tile) = mouse_to_tile(&camera_query, &window_query) else { return };
    ev.send(BoardClickEvent(tile));
}

fn mouse_to_world(
    camera_query: &Query<(&Camera, &GlobalTransform)>,
    window_query: &Query<&Window>
) -> Option<Vec2> {
    let (camera, transform) = camera_query.single();
    let cursor = window_query.single().cursor_position()?;
    camera.viewport_to_world_2d(transform, cursor)
}

fn mouse_to_tile(
    camera_query: &Query<(&Camera, &GlobalTransform)>,
    window_query: &Query<&Window> 
) -> Option<IVec2> {
    let w = mouse_to_world(camera_query, window_query)?;
    Some(world_to_tile(w))
}
