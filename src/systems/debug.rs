use crate::{VerletPoint, VerletStick};
use bevy::prelude::*;

pub fn debug_draw_sticks(
    mut gizmos: Gizmos,
    sticks_query: Query<&VerletStick>,
    points_query: Query<&GlobalTransform, With<VerletPoint>>,
) {
    for stick in sticks_query.iter() {
        if let Ok([a, b]) = points_query.get_many(stick.entities()) {
            gizmos.line(a.translation(), b.translation(), Color::WHITE);
        }
    }
}
