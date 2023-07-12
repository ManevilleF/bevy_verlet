use crate::components::{VerletLocked, VerletPoint};
use crate::resources::verlet_time_step::VerletTimeStep;
use crate::resources::VerletConfig;
use bevy::prelude::*;

fn update_point(
    transform: &mut Transform,
    point: &mut VerletPoint,
    acceleration: Vec3,
    friction: f32,
) {
    let position = transform.translation;
    let velocity = point.old_position.map_or(Vec3::ZERO, |pos| position - pos);
    transform.translation += velocity * friction + acceleration;
    point.old_position = Some(position);
}

#[allow(clippy::needless_pass_by_value, clippy::cast_possible_truncation)]
pub fn update_points(
    time_step: Res<VerletTimeStep>,
    mut points_query: Query<(&mut Transform, &mut VerletPoint), Without<VerletLocked>>,
    time: Res<Time>,
    config: Res<VerletConfig>,
) {
    let delta_time = match &*time_step {
        VerletTimeStep::DeltaTime => time.delta_seconds(),
        VerletTimeStep::FixedDeltaTime(dt) => (*dt * *dt) as f32,
    };
    let gravity = config.gravity * delta_time;
    let friction = config.friction_coefficient();

    if config.parallel_processing {
        points_query
            .par_iter_mut()
            .for_each_mut(|(mut transform, mut point)| {
                update_point(&mut transform, &mut point, gravity, friction);
            });
    } else {
        for (mut transform, mut point) in points_query.iter_mut() {
            update_point(&mut transform, &mut point, gravity, friction);
        }
    }
}
