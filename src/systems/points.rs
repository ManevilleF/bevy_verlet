use crate::components::{VerletLocked, VerletPoint2, VerletPoint2Influence};
use crate::resources::VerletGravity;
use bevy::prelude::*;

#[cfg(feature = "2D")]
pub fn update_2d_points(
    mut points_query: Query<
        (&mut Transform, &mut VerletPoint2, &mut VerletPoint2Influence),
        Without<VerletLocked>,
    >,
    time: Res<Time>,
    gravity: Option<Res<VerletGravity>>,
) {
    let gravity = gravity.map(|g| *g).unwrap_or_default();
    for (mut transform, mut point, mut influence) in points_query.iter_mut() {
        if let Some(new_pos) = influence.new_position {
            point.current_position = new_pos;
            influence.new_position = None;
        }
        let position = point.current_position;
        let velocity = point.velocity();
        point.current_position +=
            velocity + (Vec2::new(0., -1.) * gravity.0 * time.delta_seconds());
        point.old_position = position;
        transform.translation.x = point.current_position.x;
        transform.translation.y = point.current_position.y;
    }
}
