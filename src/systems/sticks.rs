use crate::components::{VerletLocked, VerletStick};
#[cfg(feature = "3D")]
use crate::components::{VerletPoint2D, VerletPoint2Influence};
#[cfg(feature = "3D")]
use crate::components::{VerletPoint3D, VerletPoint3Influence};
use bevy::log;
use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_prototype_debug_lines::DebugLines;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[cfg(feature = "2D")]
pub fn update_2d_sticks(
    sticks_query: Query<&VerletStick>,
    points_query: Query<(&VerletPoint2D, Option<&VerletLocked>)>,
    mut points_influence_query: Query<&mut VerletPoint2Influence>,
) {
    let mut sticks: Vec<&VerletStick> = sticks_query.iter().collect();
    sticks.shuffle(&mut thread_rng());
    for stick in sticks.iter() {
        let (point_a, point_a_locked) = match points_query.get(stick.point_a_entity) {
            Ok(p) => p,
            Err(e) => {
                log::error!("Could not find 2D point_a entity for stick: {}", e);
                continue;
            }
        };
        let (point_b, point_b_locked) = match points_query.get(stick.point_b_entity) {
            Ok(p) => p,
            Err(e) => {
                log::error!("Could not find 2D point_b entity for stick: {}", e);
                continue;
            }
        };
        let point_a_locked = point_a_locked.is_some();
        let point_b_locked = point_b_locked.is_some();
        if point_a_locked && point_b_locked {
            continue;
        }
        let center: Vec2 = (point_a.current_position + point_b.current_position) / 2.;
        let direction: Vec2 = (point_a.current_position - point_b.current_position).normalize();
        if !point_a_locked {
            let mut point_a_influence = points_influence_query
                .get_mut(stick.point_a_entity)
                .unwrap();
            point_a_influence.apply_new_position(center + direction * stick.length / 2.);
        }
        if !point_b_locked {
            let mut point_b_influence = points_influence_query
                .get_mut(stick.point_b_entity)
                .unwrap();
            point_b_influence.apply_new_position(center - direction * stick.length / 2.);
        }
    }
}

#[cfg(feature = "3D")]
pub fn update_3d_sticks(
    sticks_query: Query<&VerletStick>,
    points_query: Query<(&VerletPoint3D, Option<&VerletLocked>)>,
    mut points_influence_query: Query<&mut VerletPoint3Influence>,
) {
    let mut sticks: Vec<&VerletStick> = sticks_query.iter().collect();
    sticks.shuffle(&mut thread_rng());
    for stick in sticks.iter() {
        let (point_a, point_a_locked) = match points_query.get(stick.point_a_entity) {
            Ok(p) => p,
            Err(e) => {
                log::error!("Could not 3D find point_a entity for stick: {}", e);
                continue;
            }
        };
        let (point_b, point_b_locked) = match points_query.get(stick.point_b_entity) {
            Ok(p) => p,
            Err(e) => {
                log::error!("Could not 3D find point_b entity for stick: {}", e);
                continue;
            }
        };
        let point_a_locked = point_a_locked.is_some();
        let point_b_locked = point_b_locked.is_some();
        if point_a_locked && point_b_locked {
            continue;
        }
        let center = (point_a.current_position + point_b.current_position) / 2.;
        let direction = (point_a.current_position - point_b.current_position).normalize();
        if !point_a_locked {
            let mut point_a_influence = points_influence_query
                .get_mut(stick.point_a_entity)
                .unwrap();
            point_a_influence.apply_new_position(center + direction * stick.length / 2.);
        }
        if !point_b_locked {
            let mut point_b_influence = points_influence_query
                .get_mut(stick.point_b_entity)
                .unwrap();
            point_b_influence.apply_new_position(center - direction * stick.length / 2.);
        }
    }
}

#[cfg(all(feature = "2D", feature = "debug"))]
pub fn debug_draw_2d_sticks(
    mut lines: ResMut<DebugLines>,
    sticks_query: Query<&VerletStick>,
    points_query: Query<&VerletPoint2D>,
) {
    for stick in sticks_query.iter() {
        let point_a = match points_query.get(stick.point_a_entity) {
            Ok(p) => p,
            Err(e) => {
                log::error!("Could not find 2D point_a entity for stick: {}", e);
                continue;
            }
        };
        let point_b = match points_query.get(stick.point_b_entity) {
            Ok(p) => p,
            Err(e) => {
                log::error!("Could not find 2D point_b entity for stick: {}", e);
                continue;
            }
        };
        lines.line(point_a.vec3_position(), point_b.vec3_position(), 0.);
    }
}

#[cfg(all(feature = "3D", feature = "debug"))]
pub fn debug_draw_3d_sticks(
    mut lines: ResMut<DebugLines>,
    sticks_query: Query<&VerletStick>,
    points_query: Query<&VerletPoint3D>,
) {
    for stick in sticks_query.iter() {
        let point_a = match points_query.get(stick.point_a_entity) {
            Ok(p) => p,
            Err(e) => {
                log::error!("Could not find 3D point_a entity for stick: {}", e);
                continue;
            }
        };
        let point_b = match points_query.get(stick.point_b_entity) {
            Ok(p) => p,
            Err(e) => {
                log::error!("Could not find 3D point_b entity for stick: {}", e);
                continue;
            }
        };
        lines.line(point_a.current_position, point_b.current_position, 0.);
    }
}
