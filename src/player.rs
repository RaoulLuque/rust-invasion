pub mod components;
pub mod systems;

use crate::systems::constrain_ship_movement;
use systems::*;

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
struct AfterMovement;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement.before(constrain_ship_movement))
            .add_systems(Update, shoot_laser);
    }
}
