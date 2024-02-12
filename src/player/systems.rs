use crate::player::components::Player;
use bevy::prelude::*;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/PlayerShip.png"),
            ..default()
        },
        Player {},
    ));
}
