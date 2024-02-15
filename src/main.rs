pub mod enemies;
pub mod laser;
mod meteorites;
mod player;
mod systems;

use enemies::EnemiesPlugin;
use laser::LaserPlugin;
use meteorites::MeteoritePlugin;
use player::PlayerPlugin;
use systems::*;

use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowTheme},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        // Add default plugin and tweak ImagePlugin for smoother Animations with SpriteSheets and
        // window plugin
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        title: "Rust Invasion".to_string(),
                        resizable: false,
                        window_theme: Some(WindowTheme::Dark),
                        ..Default::default()
                    }),
                    ..default()
                }),
        )
        // Add Debugging info in game
        .add_plugins(WorldInspectorPlugin::new())
        // Startup Systems
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_background)
        // Systems that affect multiple plugins below
        .add_systems(Update, constrain_ship_movement)
        // Plugins for game
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemiesPlugin)
        .add_plugins(LaserPlugin)
        .add_plugins(MeteoritePlugin)
        .run();
}
