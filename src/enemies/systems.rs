use crate::laser::components::{Direction, Foe, Friend, Laser};

use super::{
    components::{Enemy, EnemyDirection},
    EnemyLaserTimer, NumberOfEnemiesLeft,
};
use crate::laser::systems::LASER_HEIGHT;

use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

pub const ENEMY_WIDTH: f32 = 93.0;
pub const ENEMY_HEIGHT: f32 = 84.0;
pub const ENEMY_SHOOT_PROBABILITY: f64 = 0.01;
pub const ENEMY_SPEED: f32 = 250.0;

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
    number_of_enemies.value = number_of_enemies_that_fit * 2;
    let space_between_enemies = (window.width()
        - (number_of_enemies_that_fit as f32 * ENEMY_WIDTH) as f32)
        / ((number_of_enemies_that_fit) as f32);

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
            Enemy {
                direction: EnemyDirection::Left,
            },
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
            Enemy {
                direction: EnemyDirection::Right,
            },
        ));
    }
}

pub fn enemy_hit_laser(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    laser_query: Query<(Entity, &Transform), (With<Laser>, With<Friend>)>,
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

pub fn tick_enemy_laser_timer(mut enemy_laser_timer: ResMut<EnemyLaserTimer>, time: Res<Time>) {
    enemy_laser_timer.timer.tick(time.delta());
}

pub fn enemy_shoots_laser(
    mut commands: Commands,
    enemy_query: Query<&Transform, With<Enemy>>,
    enemy_laser_timer: Res<EnemyLaserTimer>,
    asset_server: Res<AssetServer>,
) {
    if enemy_laser_timer.timer.finished() {
        let mut random = rand::thread_rng();

        for enemy_transform in enemy_query.iter() {
            if random.gen_bool(ENEMY_SHOOT_PROBABILITY) {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            enemy_transform.translation.x,
                            enemy_transform.translation.y - (ENEMY_HEIGHT / 2.0),
                            0.0,
                        ),
                        texture: asset_server.load("sprites/RedLaser.png"),
                        ..default()
                    },
                    Laser {
                        direction: Direction::Down,
                    },
                    Foe {},
                ));
            }
        }
    }
}

pub fn move_enemies(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut enemy_transform, enemy) in enemy_query.iter_mut() {
        enemy_transform.translation += match enemy.direction {
            EnemyDirection::Left => Vec3::new(-1.0, 0.0, 0.0) * ENEMY_SPEED * time.delta_seconds(),
            EnemyDirection::Right => Vec3::new(1.0, 0.0, 0.0) * ENEMY_SPEED * time.delta_seconds(),
        }
    }
}
