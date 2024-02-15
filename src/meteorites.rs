mod components;
mod resources;
mod systems;

use bevy::prelude::*;

use systems::*;

use self::resources::MeteoriteSpawnTimer;

pub struct MeteoritePlugin;

impl Plugin for MeteoritePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MeteoriteSpawnTimer>()
            .add_systems(Update, update_meteorite_position)
            .add_systems(Update, spawn_meteorites_over_time)
            .add_systems(Update, despawn_meteorites_out_of_screen)
            .add_systems(Update, animate_sprite)
            .add_systems(Update, tick_meteorite_spawn_timer);
    }
}
