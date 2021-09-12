use crate::components::{VerletLocked, VerletPoint2, VerletPoint2Influence, VerletStick};
use bevy::log;
use bevy::prelude::*;

#[cfg(feature = "2D")]
pub fn update_2d_sticks(
    sticks_query: Query<&VerletStick>,
    points_query: Query<(&VerletPoint2, Option<&VerletLocked>)>,
    mut points_influence_query: Query<&mut VerletPoint2Influence>,
) {
    for stick in sticks_query.iter() {
        let (point_a, point_a_locked) = match points_query.get(stick.point_a_entity) {
            Ok(p) => p,
            Err(e) => {
                log::error!("Could not find point_a entity for stick: {}", e);
                continue;
            }
        };
        let (point_b, point_b_locked) = match points_query.get(stick.point_b_entity) {
            Ok(p) => p,
            Err(e) => {
                log::error!("Could not find point_b entity for stick: {}", e);
                continue;
            }
        };
        let center: Vec2 = (point_a.current_position + point_b.current_position) / 2.;
        let direction: Vec2 = (point_a.current_position - point_b.current_position).normalize();
        if point_a_locked.is_none() {
            let mut point_a_influence = points_influence_query
                .get_mut(stick.point_a_entity)
                .unwrap();
            point_a_influence.apply_new_position(center + (direction * stick.length / 2.));
        }
        if point_b_locked.is_none() {
            let mut point_b_influence = points_influence_query
                .get_mut(stick.point_b_entity)
                .unwrap();
            point_b_influence.apply_new_position(center + (direction * stick.length / 2.));
        }
    }
}
