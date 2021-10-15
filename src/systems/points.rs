use crate::components::{VerletLocked, VerletPoint};
use crate::resources::verlet_time_step::VerletTimeStep;
use crate::resources::VerletConfig;
use bevy::prelude::*;
use bevy::tasks::ComputeTaskPool;
use std::ops::Deref;

fn update_point(
    transform: &mut Transform,
    point: &mut VerletPoint,
    down_force: Vec3,
    friction: f32,
) {
    let position = transform.translation;
    let velocity = if let Some(pos) = point.old_position {
        position - pos
    } else {
        Vec3::ZERO
    };
    transform.translation += velocity * friction + down_force;
    point.old_position = Some(position);
}

pub fn update_points(
    time_step: Res<VerletTimeStep>,
    mut points_query: Query<(&mut Transform, &mut VerletPoint), Without<VerletLocked>>,
    pool: Res<ComputeTaskPool>,
    time: Res<Time>,
    config: Option<Res<VerletConfig>>,
) {
    let config = config.map(|g| *g).unwrap_or_default();
    let delta_time = match time_step.deref() {
        VerletTimeStep::DeltaTime => time.delta_seconds(),
        VerletTimeStep::FixedDeltaTime(dt) => *dt as f32,
    };
    let down_force = config.gravity * delta_time;
    let friction = config.friction_coefficient();
    if let Some(batch_size) = config.parallel_processing_batch_size {
        points_query.par_for_each_mut(&pool, batch_size, |(mut transform, mut point)| {
            update_point(&mut transform, &mut point, down_force, friction);
        });
    } else {
        for (mut transform, mut point) in points_query.iter_mut() {
            update_point(&mut transform, &mut point, down_force, friction);
        }
    }
}
