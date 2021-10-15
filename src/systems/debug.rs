use crate::{VerletConfig, VerletPoint, VerletStick};
use bevy::log;
use bevy::prelude::*;
use bevy::tasks::ComputeTaskPool;
use bevy_prototype_debug_lines::DebugLines;
use std::sync::RwLock;

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

#[cfg(feature = "debug")]
fn draw_stick(
    stick: &VerletStick,
    points_query: &Query<&Transform, With<VerletPoint>>,
) -> Option<(Vec3, Vec3)> {
    let transform_a = get_point_debug!(points_query.get(stick.point_a_entity));
    let transform_b = get_point_debug!(points_query.get(stick.point_b_entity));
    Some((transform_a.translation, transform_b.translation))
}

#[cfg(feature = "debug")]
pub fn debug_draw_sticks(
    pool: Res<ComputeTaskPool>,
    mut lines: ResMut<DebugLines>,
    sticks_query: Query<&VerletStick>,
    points_query: Query<&Transform, With<VerletPoint>>,
    config: Option<Res<VerletConfig>>,
) {
    let config = config.map(|g| *g).unwrap_or_default();
    if let Some(batch_size) = config.parallel_processing_batch_size {
        let coords = RwLock::new(Vec::new());
        sticks_query.par_for_each(&pool, batch_size, |stick| {
            if let Some(coord) = draw_stick(stick, &points_query) {
                let mut lock = coords.write().unwrap();
                lock.push(coord)
            }
        });
        let lock = coords.read().unwrap();
        for (a, b) in lock.iter() {
            lines.line(*a, *b, 0.);
        }
    } else {
        for stick in sticks_query.iter() {
            if let Some((a, b)) = draw_stick(stick, &points_query) {
                lines.line(a, b, 0.);
            }
        }
    }
}
