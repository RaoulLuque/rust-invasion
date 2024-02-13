use bevy::ecs::component::Component;

pub enum Direction {
    Up,
    Down,
}

#[derive(Component)]
pub struct Laser {
    pub direction: Direction,
}

#[derive(Component)]
pub struct Friend {}

#[derive(Component)]
pub struct Foe {}
