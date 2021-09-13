use crate::components::{VerletLocked, VerletPoint, VerletStick};
use bevy::log;
use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_prototype_debug_lines::DebugLines;

pub fn update_sticks(
    sticks_query: Query<&VerletStick>,
    mut query: QuerySet<(
        Query<(&Transform, Option<&VerletLocked>), With<VerletPoint>>,
        Query<&mut Transform, With<VerletPoint>>,
    )>,
) {
    for stick in sticks_query.iter() {
        let (point_a, point_a_locked) = match query.q0().get(stick.point_a_entity) {
            Ok(p) => p,
            Err(e) => {
                log::error!("Could not find point_a entity for stick: {}", e);
                continue;
            }
        };
        let (point_b, point_b_locked) = match query.q0().get(stick.point_b_entity) {
            Ok(p) => p,
            Err(e) => {
                log::error!("Could not find point_b entity for stick: {}", e);
                continue;
            }
        };
        let point_a_locked = point_a_locked.is_some();
        let point_b_locked = point_b_locked.is_some();
        if point_a_locked && point_b_locked {
            continue;
        }
        let center: Vec3 = (point_a.translation + point_b.translation) / 2.;
        let direction: Vec3 = (point_a.translation - point_b.translation).normalize();
        if !point_a_locked {
            let mut point_a_transform = query.q1_mut().get_mut(stick.point_a_entity).unwrap();
            point_a_transform.translation = center + direction * stick.length / 2.;
        }
        if !point_b_locked {
            let mut point_b_transform = query.q1_mut().get_mut(stick.point_b_entity).unwrap();
            point_b_transform.translation = center - direction * stick.length / 2.;
        }
    }
}

#[cfg(feature = "debug")]
pub fn debug_draw_sticks(
    mut lines: ResMut<DebugLines>,
    sticks_query: Query<&VerletStick>,
    points_query: Query<&Transform, With<VerletPoint>>,
) {
    for stick in sticks_query.iter() {
        let transform_a = match points_query.get(stick.point_a_entity) {
            Ok(p) => p,
            Err(e) => {
                log::error!("Could not find point_a transform for stick: {}", e);
                continue;
            }
        };
        let transform_b = match points_query.get(stick.point_b_entity) {
            Ok(p) => p,
            Err(e) => {
                log::error!("Could not find point_b transform for stick: {}", e);
                continue;
            }
        };
        lines.line(transform_a.translation, transform_b.translation, 0.);
    }
}
