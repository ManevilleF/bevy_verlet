use crate::colliders::is_point_in_triangle;
use crate::colliders::lines::closest_point_to_segment_2d;
use crate::{Collider, Collision};
use bevy::prelude::{GlobalTransform, Vec2, Vec3};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct TriangleCollider {
    pub a: Vec2,
    pub b: Vec2,
    pub c: Vec2,
}

impl Collider for TriangleCollider {
    fn is_within(&self, self_transform: &GlobalTransform, point: Vec3) -> Option<Collision> {
        let pos = self_transform.translation;
        let pos_2d = Vec2::new(pos.x, pos.y);
        let point_2d = Vec2::new(point.x, point.y);
        // TODO: apply transform rotation
        let a = pos_2d + self.a;
        let b = pos_2d + self.b;
        let c = pos_2d + self.c;
        if is_point_in_triangle(point_2d, (a, b, c)) {
            let normal_points = vec![
                closest_point_to_segment_2d(point_2d, (a, b)),
                closest_point_to_segment_2d(point_2d, (b, c)),
                closest_point_to_segment_2d(point_2d, (c, a)),
            ];
            let (target_point, distance) = normal_points
                .into_iter()
                .map(|p| (p, point_2d.distance_squared(p)))
                .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap_or(Ordering::Equal))
                .unwrap();
            let normal = target_point - point_2d;
            let normal = Vec3::new(normal.x, normal.y, 0.).normalize_or_zero();
            Some(Collision {
                normal,
                target_point: Vec3::new(target_point.x, target_point.y, point.z)
                    + (normal * distance.sqrt()),
            })
        } else {
            None
        }
    }
}
