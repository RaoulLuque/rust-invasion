use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use super::components::{Meteorite, RotationAnimation};
use super::resources::MeteoriteSpawnTimer;

// Chance of spawning a meteorite every super::resource::METEORITE_SPAWN_TIME seconds
pub const CHANCE_OF_SPAWNING_METEORITE: f64 = 0.5;
pub const MAX_METEOR_WIDTH: f32 = 120.0;
pub const MAX_METEOR_HEIGHT: f32 = 120.0;

pub fn update_meteorite_position(
    mut meteorite_query: Query<(&Meteorite, &mut Transform)>,
    time: Res<Time>,
) {
    for (meteorite, mut meteorite_transform) in meteorite_query.iter_mut() {
        meteorite_transform.translation +=
            meteorite.direction * meteorite.speed * time.delta_seconds();
    }
}

pub fn tick_meteorite_spawn_timer(
    mut meteorite_spawn_timer: ResMut<MeteoriteSpawnTimer>,
    time: Res<Time>,
) {
    meteorite_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_meteorites_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    meteorite_spawn_timer: Res<MeteoriteSpawnTimer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if meteorite_spawn_timer.timer.finished()
        && rand::thread_rng().gen_bool(CHANCE_OF_SPAWNING_METEORITE)
    {
        let window = window_query.get_single().unwrap();

        let direction_of_meteor = Vec3::new(
            (rand::random::<f32>() - 0.5) * 2.0,
            (rand::random::<f32>() - 0.5) * 2.0,
            0.0,
        );
        let mut spawn_point = Vec3::default();

        if direction_of_meteor.y > 0.33 {
            // Meteorite comes from bottom border towards top
            spawn_point.x = window.width() * (rand::random::<f32>() - 0.5) + MAX_METEOR_WIDTH / 2.0;
            spawn_point.y = -window.height() / 2.0 - MAX_METEOR_HEIGHT / 2.0;
        } else if direction_of_meteor.y < -0.33 {
            // Meteorite comes from top border towards bottom
            spawn_point.x = window.width() * (rand::random::<f32>() - 0.5);
            spawn_point.y = window.height() / 2.0 + MAX_METEOR_HEIGHT / 2.0;
        } else if direction_of_meteor.x > 0.0 {
            // Meteorite comes from left border towards right
            spawn_point.x = -window.width() / 2.0 - MAX_METEOR_WIDTH / 2.0;
            spawn_point.y =
                window.height() * (rand::random::<f32>() - 0.5) + MAX_METEOR_HEIGHT / 2.0;
        } else if direction_of_meteor.x < 0.0 {
            // Meteorite comes from right border towards left
            spawn_point.x = window.width() / 2.0 + MAX_METEOR_WIDTH / 2.0;
            spawn_point.y = window.height() * (rand::random::<f32>() - 0.5);
        }

        // // 10 Possible meteorite sprites
        // let meteorite_number = (rand::random::<f32>() * 10.0).floor() as u32;
        // let meteorite_path = format!("sprites/meteorites/meteorite{}.png", meteorite_number);
        let meteorite_path = "sprites/meteorites/meteorite_sprite_sheet.png";

        let texture_handle = asset_server.load(meteorite_path);
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(49.0, 49.0), 6, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        // Use only the subset of sprites in the sheet that make up the run animation
        let animation_struct = RotationAnimation {
            length_of_animation: 6,
            animation_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        };

        commands.spawn((
            SpriteSheetBundle {
                transform: Transform::from_xyz(spawn_point.x, spawn_point.y, -1.0),
                sprite: TextureAtlasSprite::new(0),
                texture_atlas: texture_atlas_handle,
                ..default()
            },
            Meteorite {
                direction: direction_of_meteor.normalize(),
                speed: (rand::random::<f32>() * 2500.0_f32).min(750.0),
            },
            animation_struct,
        ));
    }
}

pub fn despawn_meteorites_out_of_screen(
    mut commands: Commands,
    mut meteorite_query: Query<(Entity, &Transform), With<Meteorite>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query
        .get_single()
        .expect("Primary window should exist");

    for (meteorite_entity, meteorite_transform) in meteorite_query.iter_mut() {
        let y_min: f32 = -(window.height() / 2.0) - MAX_METEOR_HEIGHT;
        let y_max: f32 = -y_min;
        let x_min: f32 = -(window.width() / 2.0) - MAX_METEOR_WIDTH;
        let x_max: f32 = -x_min;

        let meteorite_translation = meteorite_transform.translation;

        if (x_min > meteorite_translation.x
            || x_max < meteorite_translation.x
            || y_min > meteorite_translation.y
            || y_max < meteorite_translation.y)
        {
            // Meteorite out of screen bounds and reached their via entering and leaving the screen,
            // because meteorites move toward screen center when spawning and are spawned within x_min - x_max and y_min - y_max
            commands.entity(meteorite_entity).despawn_recursive();
        }
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut meteorite_query: Query<(&mut RotationAnimation, &mut TextureAtlasSprite)>,
) {
    for (mut rotation_animation, mut sprite) in &mut meteorite_query {
        rotation_animation.animation_timer.tick(time.delta());
        if rotation_animation.animation_timer.just_finished() {
            sprite.index = if sprite.index == rotation_animation.length_of_animation - 1 {
                0
            } else {
                sprite.index + 1
            };
        }
    }
}
