mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

use bevy::prelude::*;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NumberOfEnemiesLeft>()
            .add_systems(Startup, spawn_enemies)
            .add_systems(Update, enemy_hit_laser);
    }
}
