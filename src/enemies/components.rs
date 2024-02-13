use bevy::ecs::component::Component;

pub enum EnemyDirection {
    Left,
    Right,
}

#[derive(Component)]
pub struct Enemy {
    pub direction: EnemyDirection,
}
