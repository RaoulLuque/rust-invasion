use bevy::{ecs::component::Component, math::Vec3, time::Timer};

#[derive(Component)]
pub struct Meteorite {
    pub direction: Vec3,
    pub speed: f32,
}

#[derive(Component)]
pub struct RotationAnimation {
    pub length_of_animation: usize,
    pub animation_timer: Timer,
}