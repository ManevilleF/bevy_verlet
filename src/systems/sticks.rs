use crate::components::{VerletLocked, VerletPoint, VerletStick};
use crate::{VerletConfig, VerletStickMaxTension};
use bevy::log;
use bevy::prelude::*;
use bevy::tasks::ComputeTaskPool;
use std::sync::RwLock;

macro_rules! get_point {
    ($res:expr) => {
        match $res {
            Ok(p) => p,
            Err(e) => {
                log::error!("Could not find point entity for stick: {}", e);
                continue;
            }
        }
    };
}

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
    for _ in 0..=config.sticks_computation_depth {
        for stick in sticks_query.iter() {
            let (point_a, point_a_locked) = get_point!(points_query.q0().get(stick.point_a_entity));
            let (point_b, point_b_locked) = get_point!(points_query.q0().get(stick.point_b_entity));

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

pub fn handle_stick_constraints(
    pool: Res<ComputeTaskPool>,
    mut commands: Commands,
    sticks_query: Query<(Entity, &VerletStick, &VerletStickMaxTension)>,
    points_query: Query<&Transform, With<VerletPoint>>,
    config: Option<Res<VerletConfig>>,
) {
    let config = config.map(|g| *g).unwrap_or_default();
    if let Some(batch_size) = config.parallel_processing_batch_size {
        let sticks_to_destroy = RwLock::new(Vec::new());
        sticks_query.par_for_each(&pool, batch_size, |(entity, stick, max_tension)| {
            if let Some(entity) = handle_stick_constraint(entity, stick, max_tension, &points_query)
            {
                let mut lock = sticks_to_destroy.write().unwrap();
                lock.push(entity);
            }
        });
        let lock = sticks_to_destroy.read().unwrap();
        for entity in lock.iter() {
            commands.entity(*entity).despawn_recursive();
        }
    } else {
        for (entity, stick, max_tension) in sticks_query.iter() {
            if let Some(entity) = handle_stick_constraint(entity, stick, max_tension, &points_query)
            {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
