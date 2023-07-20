use crate::{VerletPoint, VerletStick};
use bevy::prelude::*;

#[allow(clippy::needless_pass_by_value)]
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
