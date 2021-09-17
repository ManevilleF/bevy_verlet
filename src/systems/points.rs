use crate::components::{VerletLocked, VerletPoint};
use crate::resources::VerletConfig;
use crate::verlet_time_step::VerletTimeStep;
use bevy::prelude::*;
use std::ops::Deref;

pub fn update_points(
    time_step: Res<VerletTimeStep>,
    mut points_query: Query<
        (&mut Transform, &GlobalTransform, &mut VerletPoint),
        Without<VerletLocked>,
    >,
    time: Res<Time>,
    config: Option<Res<VerletConfig>>,
) {
    let config = config.map(|g| *g).unwrap_or_default();
    let delta_time = match time_step.deref() {
        VerletTimeStep::DeltaTime => time.delta_seconds(),
        VerletTimeStep::FixedDeltaTime(dt) => *dt as f32,
    };
    let down_force = config.gravity * delta_time;
    for (mut transform, g_transform, mut point) in points_query.iter_mut() {
        let position = g_transform.translation;
        let velocity = if let Some(pos) = point.old_position {
            position - pos
        } else {
            Vec3::ZERO
        };
        transform.translation += velocity * config.friction_coefficient() + down_force;
        point.old_position = Some(position);
    }
}
