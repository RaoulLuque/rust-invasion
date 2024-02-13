mod components;
mod systems;

use bevy::prelude::*;

use self::systems::update_meteorite_position;

pub struct MeteoritePlugin;

impl Plugin for MeteoritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_meteorite_position);
    }
}
