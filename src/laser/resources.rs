use bevy::{prelude::*, time::Stopwatch};

#[derive(Resource, Default)]
pub struct LaserDelayTimer {
    pub timer: Stopwatch,
}
