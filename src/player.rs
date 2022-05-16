use crate::ascii::{self, AsciiSheet};
use crate::tilemap::TileCollider;
use crate::TILE_SIZE;
use bevy::prelude::*;
use bevy::sprite::collide_aabb;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable)]
pub struct Player {
    pub speed: f32,
}
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(camera_follow.after("movement"))
            .add_system(player_movement.label("movement"));
    }
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player = ascii::spawn_ascii_sprite(
        &mut commands,
        &ascii,
        1,
        Color::rgb(0.3, 0.3, 0.9),
        Vec3::new(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
    );

    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player { speed: 3.0 });

    let background = ascii::spawn_ascii_sprite(
        &mut commands,
        &ascii,
        0,
        Color::rgb(0.5, 0.5, 0.5),
        Vec3::new(0.0, 0.0, -1.0),
    );

    commands.entity(background).insert(Name::new("Background"));
    commands.entity(player).push_children(&[background]);
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera.single_mut();
    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn player_movement(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    // crash if 0 or > 1 match, use get_single_mut for a Result instead.
    let (mut player, mut transform) = player_query.single_mut();

    let mut y_delta = 0.0;
    if keyboard.pressed(KeyCode::Z) || keyboard.pressed(KeyCode::Up) {
        y_delta += player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down) {
        y_delta -= player.speed * TILE_SIZE * time.delta_seconds();
    }

    let mut x_delta = 0.0;
    if keyboard.pressed(KeyCode::Q) || keyboard.pressed(KeyCode::Left) {
        x_delta -= player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
        x_delta += player.speed * TILE_SIZE * time.delta_seconds();
    }

    let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
    if wall_collision_check(target, &wall_query) {
        transform.translation = target;
    }

    let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    if wall_collision_check(target, &wall_query) {
        transform.translation = target;
    }

    if keyboard.just_pressed(KeyCode::Space) {
        player.speed *= 2.0;
    }

    if keyboard.just_released(KeyCode::Space) {
        player.speed /= 2.0;
    }

    if keyboard.pressed(KeyCode::R) {
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
    }
}

fn wall_collision_check(
    target_player_pos: Vec3,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>,
) -> bool {
    for wall_transform in wall_query.iter() {
        let collision = collide_aabb::collide(
            target_player_pos,
            Vec2::splat(TILE_SIZE * 0.9), //TODO: WTF change this ugliness
            wall_transform.translation,
            Vec2::splat(TILE_SIZE * 1.0),
        );
        if collision.is_some() {
            return false;
        }
    }
    true
}
