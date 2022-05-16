#![allow(clippy::redundant_field_names)]
use bevy::prelude::*;

use proto_game::player::PlayerPlugin;
use proto_game::AsciiPlugin;
use proto_game::AudioPlugin;
use proto_game::CameraPlugin;
use proto_game::DebugPlugin;
use proto_game::TileMapPlugin;
use proto_game::{BACKGROUND_COLOR, RESOLUTION};

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(WindowDescriptor {
            title: "Game Prototype".to_owned(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AsciiPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(TileMapPlugin)
        .add_plugin(DebugPlugin)
        .run();
}
