use super::{components::*, resources::LaserDelayTimer};

use bevy::{prelude::*, window::PrimaryWindow};

pub const LASER_HEIGHT: f32 = 37.0;
pub const LASER_SPEED: f32 = 700.0;

pub fn update_laser_position(mut laser_query: Query<&mut Transform, With<Laser>>, time: Res<Time>) {
    for mut laser in laser_query.iter_mut() {
        laser.translation += Vec3::new(0.0, 1.0, 0.0) * LASER_SPEED * time.delta_seconds();
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
        if laser_transform.translation.y > (window.height() / 2.0) + (LASER_HEIGHT / 2.0) {
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
