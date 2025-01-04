use crate::{
    components::{VerletLocked, VerletPoint},
    config::VerletConfig,
};
use bevy::prelude::*;

fn update_point(
    transform: &mut Transform,
    point: &mut VerletPoint,
    acceleration: Vec3,
    friction: f32,
    dt: f32,
) {
    let position = transform.translation;
    let velocity = point.old_position.map_or(Vec3::ZERO, |pos| position - pos);
    transform.translation += velocity * friction + (acceleration / point.mass) * friction * dt * dt;
    point.old_position = Some(position);
}

#[allow(clippy::needless_pass_by_value, clippy::cast_possible_truncation)]
pub fn update_points(
    mut points_query: Query<(&mut Transform, &mut VerletPoint), Without<VerletLocked>>,
    time: Res<Time>,
    config: Res<VerletConfig>,
) {
    let gravity = config.gravity;
    let friction = config.friction_coefficient();

    if config.parallel_processing {
        points_query
            .par_iter_mut()
            .for_each(|(mut transform, mut point)| {
                update_point(
                    &mut transform,
                    &mut point,
                    gravity,
                    friction,
                    time.delta_secs(),
                );
            });
    } else {
        for (mut transform, mut point) in &mut points_query {
            update_point(
                &mut transform,
                &mut point,
                gravity,
                friction,
                time.delta_secs(),
            );
        }
    }
}
