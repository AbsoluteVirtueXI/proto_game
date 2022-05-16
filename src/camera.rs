use crate::RESOLUTION;
use bevy::{prelude::*, render::camera::ScalingMode};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    //TODO: Maybe usefull later, else just remove.
    //let mut camera = OrthographicCameraBundle::new_2d();
    //camera.orthographic_projection.top = 50.0;
    //camera.orthographic_projection.bottom = -50.0;
    //camera.orthographic_projection.right = 50.0; //* RESOLUTION;
    //camera.orthographic_projection.left = -50.0; //* RESOLUTION;
    //camera.orthographic_projection.scaling_mode = ScalingMode::None;
    //commands.spawn_bundle(camera);
}
