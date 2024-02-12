mod components;
mod systems;

use systems::*;

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
struct AfterMovement;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, constrain_player_movement.in_set(AfterMovement))
            .add_systems(Update, player_movement.before(AfterMovement));
    }
}
