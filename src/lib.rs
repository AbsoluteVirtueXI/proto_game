use bevy::prelude::*;

pub mod ascii;
pub mod audio;
pub mod camera;
pub mod debug;
pub mod player;
pub mod tilemap;

pub use ascii::AsciiPlugin;
pub use audio::AudioPlugin;
pub use camera::CameraPlugin;
pub use debug::DebugPlugin;
pub use player::PlayerPlugin;
pub use tilemap::TileMapPlugin;

/// Constants
pub const TILE_SIZE: f32 = 0.1;
pub const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
