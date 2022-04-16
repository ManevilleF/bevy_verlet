use crate::components::{VerletLocked, VerletPoint, VerletStick};
use crate::{VerletConfig, VerletStickMaxTension};
use bevy::log;
use bevy::prelude::*;

#[allow(
    clippy::type_complexity,
    clippy::needless_pass_by_value,
    clippy::similar_names
)]
pub fn update_sticks(
    config: Option<Res<VerletConfig>>,
    sticks_query: Query<&VerletStick>,
    mut points_query: Query<(&mut Transform, Option<&VerletLocked>), With<VerletPoint>>,
) {
    let config = config.map(|g| *g).unwrap_or_default();
    for _ in 0..=config.sticks_computation_depth {
        for stick in sticks_query.iter() {
            let [(mut transform_a, a_locked), (mut transform_b, b_locked)] =
                match points_query.get_many_mut([stick.point_a_entity, stick.point_b_entity]) {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("Could not find point entity for stick: {}", e);
                        continue;
                    }
                };
            if a_locked.is_some() && b_locked.is_some() {
                continue;
            }
            let (coords_a, coords_b) = (transform_a.translation, transform_b.translation);
            let center: Vec3 = (coords_a + coords_b) / 2.;
            let direction: Vec3 = (coords_a - coords_b).normalize() * stick.length / 2.;
            if a_locked.is_none() {
                transform_a.translation = center + direction;
            }
            if b_locked.is_none() {
                transform_b.translation = center - direction;
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
