use bevy::prelude::*;

// Interval for checking if meteorites should spawn in seconds
const METEORITE_SPAWN_TIME: f32 = 0.4;

#[derive(Resource)]
pub struct MeteoriteSpawnTimer {
    pub timer: Timer,
}

impl Default for MeteoriteSpawnTimer {
    fn default() -> MeteoriteSpawnTimer {
        MeteoriteSpawnTimer {
            timer: Timer::from_seconds(METEORITE_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
