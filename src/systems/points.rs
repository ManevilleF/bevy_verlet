use crate::components::VerletLocked;
use crate::resources::VerletConfig;
#[cfg(feature = "2D")]
use crate::{VerletPoint2D, VerletPoint2Influence};
#[cfg(feature = "3D")]
use crate::{VerletPoint3D, VerletPoint3Influence};
use bevy::prelude::*;

#[cfg(feature = "2D")]
pub fn update_2d_points(
    mut points_query: Query<(
        &mut Transform,
        &mut VerletPoint2D,
        &mut VerletPoint2Influence,
        Option<&VerletLocked>,
    )>,
    time: Res<Time>,
    config: Option<Res<VerletConfig>>,
) {
    let config = config.map(|g| *g).unwrap_or_default();
    let delta_time = time.delta_seconds();
    let down_force = config.gravity_2d() * delta_time;
    for (mut transform, mut point, mut influence, locked) in points_query.iter_mut() {
        if locked.is_none() {
            if let Some(new_pos) = influence.new_position {
                point.current_position = new_pos;
                influence.new_position = None;
            }
            let position = point.current_position;
            let velocity: Vec2 = point.current_position - point.old_position;
            point.current_position += velocity * config.friction_coefficient() + down_force;
            point.old_position = position;
        }
        transform.translation.x = point.current_position.x;
        transform.translation.y = point.current_position.y;
    }
}

#[cfg(feature = "3D")]
pub fn update_3d_points(
    mut points_query: Query<(
        &mut Transform,
        &mut VerletPoint3D,
        &mut VerletPoint3Influence,
        Option<&VerletLocked>,
    )>,
    time: Res<Time>,
    config: Option<Res<VerletConfig>>,
) {
    let config = config.map(|g| *g).unwrap_or_default();
    let delta_time = time.delta_seconds();
    let down_force = config.gravity * delta_time;
    for (mut transform, mut point, mut influence, locked) in points_query.iter_mut() {
        if locked.is_none() {
            if let Some(new_pos) = influence.new_position {
                point.current_position = new_pos;
                influence.new_position = None;
            }
            let position = point.current_position;
            let velocity: Vec3 = point.current_position - point.old_position;
            point.current_position += velocity * config.friction_coefficient() + down_force;
            point.old_position = position;
        }
        transform.translation = point.current_position;
    }
}
