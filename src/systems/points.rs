use crate::components::{VerletLocked, VerletPoint};
use crate::resources::VerletConfig;
use bevy::prelude::*;

pub fn update_points(
    mut points_query: Query<(&mut Transform, &mut VerletPoint), Without<VerletLocked>>,
    time: Res<Time>,
    config: Option<Res<VerletConfig>>,
) {
    let config = config.map(|g| *g).unwrap_or_default();
    let delta_time = time.delta_seconds();
    let down_force = config.gravity * delta_time;
    for (mut transform, mut point) in points_query.iter_mut() {
        let position = transform.translation;
        let velocity = if let Some(pos) = point.old_position { position - pos } else { Vec3::ZERO };
        transform.translation += velocity * config.friction_coefficient() + down_force;
        point.old_position = Some(position);
    }
}
