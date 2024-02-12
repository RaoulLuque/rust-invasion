mod player;
mod systems;

use player::*;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, spawn_camera)
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .run();
}
