use crate::components::{VerletLocked, VerletPoint, VerletStick};
use crate::{VerletConfig, VerletStickMaxTension};
use bevy::log;
use bevy::prelude::*;

macro_rules! get_point {
    ($res:expr) => {
        match $res {
            Ok((p, locked)) => (p.translation, locked.is_some()),
            Err(e) => {
                log::error!("Could not find point entity for stick: {}", e);
                continue;
            }
        }
    };
}

#[allow(
    clippy::type_complexity,
    clippy::needless_pass_by_value,
    clippy::similar_names
)]
pub fn update_sticks(
    config: Option<Res<VerletConfig>>,
    sticks_query: Query<&VerletStick>,
    mut points_query: QuerySet<(
        QueryState<(&Transform, Option<&VerletLocked>), With<VerletPoint>>,
        QueryState<&mut Transform, With<VerletPoint>>,
    )>,
) {
    let config = config.map(|g| *g).unwrap_or_default();
    for _ in 0..=config.sticks_computation_depth {
        for stick in sticks_query.iter() {
            let (coords_a, a_locked) = get_point!(points_query.q0().get(stick.point_a_entity));
            let (coords_b, b_locked) = get_point!(points_query.q0().get(stick.point_b_entity));

            if a_locked && b_locked {
                continue;
            }
            let center: Vec3 = (coords_a + coords_b) / 2.;
            let direction: Vec3 = (coords_a - coords_b).normalize();
            let mut q1 = points_query.q1();
            if !a_locked {
                let mut point_a_transform = q1.get_mut(stick.point_a_entity).unwrap();
                point_a_transform.translation = center + direction * stick.length / 2.;
            }
            if !b_locked {
                let mut point_b_transform = q1.get_mut(stick.point_b_entity).unwrap();
                point_b_transform.translation = center - direction * stick.length / 2.;
            }
        }
    }
}

fn handle_stick_constraint(
    entity: Entity,
    stick: &VerletStick,
    max_tension: &VerletStickMaxTension,
    points_query: &Query<&Transform, With<VerletPoint>>,
) -> Option<Entity> {
    let point_a = match points_query.get(stick.point_a_entity) {
        Ok(p) => p,
        Err(e) => {
            log::error!("Could not find point_a entity for stick: {}", e);
            return None;
        }
    };
    let point_b = match points_query.get(stick.point_b_entity) {
        Ok(p) => p,
        Err(e) => {
            log::error!("Could not find point_b entity for stick: {}", e);
            return None;
        }
    };
    let distance = point_a.translation.distance(point_b.translation);
    if distance > stick.length * max_tension.0 {
        Some(entity)
    } else {
        None
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn handle_stick_constraints(
    mut commands: Commands,
    sticks_query: Query<(Entity, &VerletStick, &VerletStickMaxTension)>,
    points_query: Query<&Transform, With<VerletPoint>>,
) {
    for (entity, stick, max_tension) in sticks_query.iter() {
        if let Some(entity) = handle_stick_constraint(entity, stick, max_tension, &points_query) {
            commands.entity(entity).despawn_recursive();
        }
    }
}
