pub mod components;
pub mod resources;
pub mod systems;

use systems::*;

use bevy::prelude::*;

use self::resources::LaserDelayTimer;

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LaserDelayTimer>()
            .add_systems(Startup, start_stopwatch)
            .add_systems(Update, tick_laser_delay_timer)
            .add_systems(Update, update_laser_position)
            .add_systems(Update, delete_laser_if_out_of_screen);
    }
}
