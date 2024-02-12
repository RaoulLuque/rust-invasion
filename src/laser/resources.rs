use bevy::{prelude::*, time::Stopwatch};

pub const LASER_DELAY: f32 = 0.5;

#[derive(Resource, Default)]
pub struct LaserDelayTimer {
    pub timer: Stopwatch,
}