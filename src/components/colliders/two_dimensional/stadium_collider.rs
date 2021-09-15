use crate::colliders::is_point_in_circle;
use crate::colliders::lines::closest_point_to_segment_2d;
use crate::{Collider, Collision};
use bevy::prelude::{GlobalTransform, Vec2, Vec3};

#[derive(Debug, Clone)]
pub struct StadiumCollider {
    pub start: Vec2,
    pub end: Vec2,
    pub radius: f32,
}

impl Collider for StadiumCollider {
    fn is_within(&self, self_transform: &GlobalTransform, point: Vec3) -> Option<Collision> {
        let pos = self_transform.translation;
        let pos_2d = Vec2::new(pos.x, pos.y);
        let point_2d = Vec2::new(point.x, point.y);
        let projected_point_2d =
            closest_point_to_segment_2d(point_2d, (pos_2d + self.start, pos_2d + self.end));
        if is_point_in_circle(point_2d, projected_point_2d, self.radius) {
            // TODO: investigate 0 normal cases
            let normal = point_2d - pos_2d;
            let normal = Vec3::new(normal.x, normal.y, 0.).normalize_or_zero();
            Some(Collision {
                normal,
                target_point: pos + normal * self.radius,
            })
        } else {
            None
        }
    }
}
