use crate::components::Laser;

use super::{components::Enemy, NumberOfEnemiesLeft};
use crate::laser::systems::LASER_HEIGHT;

use bevy::{prelude::*, window::PrimaryWindow};

pub const ENEMY_WIDTH: f32 = 93.0;
pub const ENEMY_HEIGHT: f32 = 84.0;

pub fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut number_of_enemies: ResMut<NumberOfEnemiesLeft>,
) {
    let window = window_query
        .get_single()
        .expect("Primary window should exist");

    let number_of_enemies_that_fit = (window.width() / ENEMY_WIDTH * 0.9) as u32;
    let space_between_enemies = (window.width()
        - (number_of_enemies_that_fit as f32 * ENEMY_WIDTH) as f32)
        / ((number_of_enemies_that_fit + 1) as f32);

    for i in 0..number_of_enemies_that_fit {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    -(window.width() / 2.0)
                        + ((space_between_enemies + ENEMY_WIDTH) * i as f32)
                        + (ENEMY_WIDTH / 2.0),
                    window.height() / 2.0 * 0.9,
                    0.0,
                ),
                texture: asset_server.load("sprites/EnemyShip.png"),
                ..default()
            },
            Enemy {},
        ));
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    -(window.width() / 2.0)
                        + ((space_between_enemies + ENEMY_WIDTH) * i as f32)
                        + (ENEMY_WIDTH / 2.0),
                    (window.height() / 2.0 * 0.9) - (ENEMY_HEIGHT + 0.05 * window.height()),
                    0.0,
                ),
                texture: asset_server.load("sprites/EnemyShip.png"),
                ..default()
            },
            Enemy {},
        ));
    }
}

pub fn enemy_hit_laser(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    laser_query: Query<(Entity, &Transform), With<Laser>>,
) {
    for (enemy_entity, enemy_transform) in enemy_query.iter() {
        for (laser_entity, laser_transform) in laser_query.iter() {
            if enemy_transform
                .translation
                .distance(laser_transform.translation)
                < LASER_HEIGHT / 2.0 + ENEMY_HEIGHT / 2.0
            {
                debug!("An enemy was just hit by a laser");
                commands.entity(enemy_entity).despawn();
                commands.entity(laser_entity).despawn();
            }
        }
    }
}
