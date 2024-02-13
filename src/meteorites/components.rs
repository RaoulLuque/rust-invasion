use bevy::{ecs::component::Component, math::Vec3};

#[derive(Component)]
pub struct Meteorite {
    pub direction: Vec3,
    pub speed: f32,
}
