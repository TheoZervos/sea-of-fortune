use bevy::prelude::*;

//setting window constants
pub const WIN_W: f32 = 1280.;
pub const WIN_H: f32 = 720.;

//setting level constants
pub const TILE_SIZE: u32 = 32;

//REMEMBER TO CHANGE THIS WHEN WE CHANGE MAP SIZE
pub const OCEAN_LEVEL_H: f32 = 32000.;
pub const OCEAN_LEVEL_W: f32 = 32000.;
pub const SAND_LEVEL_H: f32 = 3000.;
pub const SAND_LEVEL_W: f32 = 3000.;

//for boat (change later, should not have bounds determined
//in different ways for different entities)
pub const BOUNDS: Vec2 = Vec2::new(OCEAN_LEVEL_W, OCEAN_LEVEL_H);
