use crate::{VerletPoint, VerletStick};
use bevy::log;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

macro_rules! get_point_debug {
    ($res:expr) => {
        match $res {
            Ok(p) => p,
            Err(e) => {
                log::warn!("Could not find point entity to draw debug stick: {}", e);
                return None;
            }
        }
    };
}

fn draw_stick(
    stick: &VerletStick,
    points_query: &Query<&Transform, With<VerletPoint>>,
) -> Option<(Vec3, Vec3)> {
    let transform_a = get_point_debug!(points_query.get(stick.point_a_entity));
    let transform_b = get_point_debug!(points_query.get(stick.point_b_entity));
    Some((transform_a.translation, transform_b.translation))
}

#[allow(clippy::needless_pass_by_value)]
pub fn debug_draw_sticks(
    mut lines: ResMut<DebugLines>,
    sticks_query: Query<&VerletStick>,
    points_query: Query<&Transform, With<VerletPoint>>,
) {
    for stick in sticks_query.iter() {
        if let Some((a, b)) = draw_stick(stick, &points_query) {
            lines.line(a, b, 0.);
        }
    }
}
