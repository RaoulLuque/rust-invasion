use crate::player::components::Player;
use bevy::{input::keyboard, prelude::*, transform, window::PrimaryWindow};

pub const PLAYER_HEIGHT: f32 = 75.0;
pub const PLAYER_WIDTH: f32 = 112.0;
pub const PLAYER_SPEED: f32 = 500.0;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query
        .get_single()
        .expect("Primary window should exist");

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, -(window.height() / 2.0) + PLAYER_HEIGHT, 0.0),
            texture: asset_server.load("sprites/PlayerShip.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        } else if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn constrain_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query
        .get_single()
        .expect("Primary window should exist");

    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let x_min = -window.width() / 2.0;
        let x_max = -x_min;

        let player_translation_x_value: &mut f32 = &mut player_transform.translation.x;

        // Let player appear on right side of screen
        if *player_translation_x_value < x_min {
            *player_translation_x_value += window.width();
        } else if *player_translation_x_value > x_max {
            *player_translation_x_value -= window.width();
        }
    }
}
