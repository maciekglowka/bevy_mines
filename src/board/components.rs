use bevy::prelude::*;

#[derive(Component)]
pub struct Covered;

#[derive(Component)]
pub struct Position(pub IVec2);

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Trap(pub u32);
