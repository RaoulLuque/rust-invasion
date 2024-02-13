use bevy::{prelude::*, window::PrimaryWindow};

use crate::{enemies::components::Enemy, player::components::Player};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((SpriteBundle {
        transform: Transform::from_xyz(0.0, 0.0, -10.0),
        texture: asset_server.load("background/Background.png"),
        ..default()
    },));
}

pub fn constrain_ship_movement(
    mut ship_query: Query<&mut Transform, Or<(&Player, &Enemy)>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query
        .get_single()
        .expect("Primary window should exist");

    for mut ship_transform in ship_query.iter_mut() {
        let x_min = -window.width() / 2.0;
        let x_max = -x_min;

        let ship_translation_x_value: &mut f32 = &mut ship_transform.translation.x;

        // Let ship appear on right side of screen
        if *ship_translation_x_value < x_min {
            *ship_translation_x_value += window.width();
        } else if *ship_translation_x_value > x_max {
            *ship_translation_x_value -= window.width();
        }
    }
}
