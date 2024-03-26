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
    if !mouse_buttons.just_pressed(MouseButton::Left) { return };
    for touch in touches.iter() {
        if let Some(tile) = touch_to_tile(touch, &camera_query) {
            ev.send(BoardClickEvent(tile));
        }
    }
    if let Some(tile) = mouse_to_tile(&camera_query, &window_query) {
        ev.send(BoardClickEvent(tile));
    }
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

fn touch_to_tile(
    touch: &bevy::input::touch::Touch,
    camera_query: &Query<(&Camera, &GlobalTransform)>
) -> Option<IVec2> {
    let (camera, transform) = camera_query.single();
    let w = camera.viewport_to_world_2d(transform, touch.position())?;
    Some(world_to_tile(w))
}
