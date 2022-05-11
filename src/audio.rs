use bevy::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(play_intro_music)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_intro_music);
    }
}

fn play_intro_music(music: Res<IntroMusic>, audio: Res<Audio>) {
    audio.play(music.0.clone());
}
pub struct IntroMusic(Handle<AudioSource>);

fn load_intro_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    let music = asset_server.load("sounds/me.ogg");
    commands.insert_resource(IntroMusic(music));
}
