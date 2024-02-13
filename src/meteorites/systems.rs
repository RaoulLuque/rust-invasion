use bevy::prelude::*;

use super::components::Meteorite;

pub fn update_meteorite_position(
    mut meteorite_query: Query<(&Meteorite, &mut Transform)>,
    time: Res<Time>,
) {
    for (meteorite, mut meteorite_transform) in meteorite_query.iter_mut() {
        meteorite_transform.translation +=
            meteorite.direction * meteorite.speed * time.delta_seconds();
    }
}
