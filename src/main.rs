pub mod enemies;
pub mod laser;
mod player;
mod systems;

use enemies::EnemiesPlugin;
use laser::*;
use player::*;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, constrain_ship_movement)
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemiesPlugin)
        .add_plugins(LaserPlugin)
        .run();
}
