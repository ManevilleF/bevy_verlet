use crate::colliders::is_point_in_circle;
use crate::colliders::lines::closest_point_to_segment_2d;
use crate::{Collider2D, Collision2D};
use bevy::prelude::{GlobalTransform, Vec2};

#[derive(Debug, Clone)]
pub struct StadiumCollider {
    pub start: Vec2,
    pub end: Vec2,
    pub radius: f32,
}

impl Collider2D for StadiumCollider {
    fn is_within(&self, self_transform: &GlobalTransform, point: Vec2) -> Option<Collision2D> {
        let pos = Vec2::new(self_transform.translation.x, self_transform.translation.y);
        let projected_point =
            closest_point_to_segment_2d(point, (pos + self.start, pos + self.end));
        if is_point_in_circle(point, projected_point, self.radius) {
            // TODO: investigate 0 normal cases
            let normal = (point - projected_point).normalize_or_zero();
            Some(Collision2D {
                normal,
                target_point: projected_point + normal * self.radius,
            })
        } else {
            None
        }
    }
}
