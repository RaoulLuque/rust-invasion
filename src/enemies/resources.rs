use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct NumberOfEnemiesLeft {
    pub value: u32,
}
