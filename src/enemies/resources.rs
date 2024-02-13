use bevy::prelude::*;

pub const STAR_SPAWN_TIME: f32 = 0.07;

#[derive(Resource, Default)]
pub struct NumberOfEnemiesLeft {
    pub value: u32,
}

#[derive(Resource)]
pub struct EnemyLaserTimer {
    pub timer: Timer,
}

impl Default for EnemyLaserTimer {
    fn default() -> EnemyLaserTimer {
        EnemyLaserTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
