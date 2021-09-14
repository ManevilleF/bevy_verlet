use crate::components::{VerletLocked, VerletPoint, VerletStick};
use crate::{VerletConfig, VerletStickMaxTension};
use bevy::log;
use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_prototype_debug_lines::DebugLines;
#[cfg(feature = "shuffle")]
use rand::{prelude::SliceRandom, thread_rng};

#[allow(clippy::type_complexity)]
pub fn update_sticks(
    config: Option<Res<VerletConfig>>,
    sticks_query: Query<&VerletStick>,
    mut points_query: QuerySet<(
        Query<(&Transform, Option<&VerletLocked>), With<VerletPoint>>,
        Query<&mut Transform, With<VerletPoint>>,
    )>,
) {
    let config = config.map(|g| *g).unwrap_or_default();
    #[cfg(not(feature = "shuffle"))]
    let iterator = sticks_query;
    #[cfg(feature = "shuffle")]
    let mut iterator: Vec<&VerletStick> = sticks_query.iter().collect();
    for _ in 0..=config.sticks_computation_depth {
        #[cfg(feature = "shuffle")]
        iterator.shuffle(&mut thread_rng());
        for stick in iterator.iter() {
            let (point_a, point_a_locked) = match points_query.q0().get(stick.point_a_entity) {
                Ok(p) => p,
                Err(e) => {
                    log::error!("Could not find point_a entity for stick: {}", e);
                    continue;
                }
            };
            let (point_b, point_b_locked) = match points_query.q0().get(stick.point_b_entity) {
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
                let mut point_a_transform =
                    points_query.q1_mut().get_mut(stick.point_a_entity).unwrap();
                point_a_transform.translation = center + direction * stick.length / 2.;
            }
            if !point_b_locked {
                let mut point_b_transform =
                    points_query.q1_mut().get_mut(stick.point_b_entity).unwrap();
                point_b_transform.translation = center - direction * stick.length / 2.;
            }
        }
    }
}

pub fn handle_stick_constraints(
    mut commands: Commands,
    sticks_query: Query<(Entity, &VerletStick, &VerletStickMaxTension)>,
    points_query: Query<&Transform, With<VerletPoint>>,
) {
    for (entity, stick, max_tension) in sticks_query.iter() {
        let point_a = match points_query.get(stick.point_a_entity) {
            Ok(p) => p,
            Err(e) => {
                log::error!("Could not find point_a entity for stick: {}", e);
                continue;
            }
        };
        let point_b = match points_query.get(stick.point_b_entity) {
            Ok(p) => p,
            Err(e) => {
                log::error!("Could not find point_b entity for stick: {}", e);
                continue;
            }
        };
        let distance = point_a.translation.distance(point_b.translation);
        if distance > stick.length * max_tension.0 {
            commands.entity(entity).despawn_recursive();
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
