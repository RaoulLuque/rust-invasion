use super::components::Direction;
use super::{components::*, resources::LaserDelayTimer};
use crate::player::components::Player;
use crate::player::systems::{PLAYER_HEIGHT, PLAYER_WIDTH};

use bevy::{prelude::*, window::PrimaryWindow};

pub const LASER_HEIGHT: f32 = 37.0;
pub const LASER_SPEED: f32 = 700.0;

pub fn update_laser_position(mut laser_query: Query<(&mut Transform, &Laser)>, time: Res<Time>) {
    for (mut laser_transform, laser_direction) in laser_query.iter_mut() {
        laser_transform.translation += match laser_direction.direction {
            Direction::Up => Vec3::new(0.0, 1.0, 0.0) * LASER_SPEED * time.delta_seconds(),
            Direction::Down => Vec3::new(0.0, -1.0, 0.0) * LASER_SPEED * time.delta_seconds(),
        }
    }
}

pub fn delete_laser_if_out_of_screen(
    mut commands: Commands,
    mut laser_query: Query<(Entity, &Transform), With<Laser>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query
        .get_single()
        .expect("Primary window should exist");

    for (laser_entity, laser_transform) in laser_query.iter_mut() {
        if laser_transform.translation.y > (window.height() / 2.0) + (LASER_HEIGHT / 2.0)
            || laser_transform.translation.y < (-window.height() / 2.0) - (LASER_HEIGHT / 2.0)
        {
            commands.entity(laser_entity).despawn();
        }
    }
}

pub fn tick_laser_delay_timer(mut laser_delay_timer: ResMut<LaserDelayTimer>, time: Res<Time>) {
    laser_delay_timer.timer.tick(time.delta());
}

pub fn start_stopwatch(mut laser_delay_timer: ResMut<LaserDelayTimer>) {
    laser_delay_timer.timer.reset();
}

#[allow(clippy::type_complexity)]
pub fn laser_hit_player(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    laser_query: Query<(Entity, &Transform), (With<Laser>, With<Foe>)>,
) {
    for (player_entity, player_transform) in player_query.iter() {
        for (laser_entity, laser_transform) in laser_query.iter() {
            let player_translation = player_transform.translation;
            // Figure out hit box of player
            let x_min = player_translation.x - (PLAYER_WIDTH / 2.0);
            let x_max = player_translation.x + (PLAYER_WIDTH / 2.0);
            let y_min = player_translation.y - (PLAYER_HEIGHT / 2.0 * 0.9);
            let y_max = player_translation.y + (PLAYER_HEIGHT / 2.0 * 0.9);

            if laser_transform.translation.x > x_min
                && laser_transform.translation.x < x_max
                && laser_transform.translation.y - (LASER_HEIGHT / 2.0) > y_min
                && laser_transform.translation.y - (LASER_HEIGHT / 2.0) < y_max
            {
                debug!("The Player was just hit by a laser");
                commands.entity(player_entity).despawn();
                commands.entity(laser_entity).despawn();
            }
        }
    }
}
