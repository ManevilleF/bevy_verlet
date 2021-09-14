use crate::colliders::is_point_in_circle;
use crate::{Collider2D, Collision2D};
use bevy::prelude::{GlobalTransform, Vec2};

#[derive(Debug, Clone)]
pub struct CircleCollider {
    pub radius: f32,
}

impl Collider2D for CircleCollider {
    fn is_within(&self, self_transform: &GlobalTransform, point: Vec2) -> Option<Collision2D> {
        let pos = Vec2::new(self_transform.translation.x, self_transform.translation.y);
        if is_point_in_circle(point, pos, self.radius) {
            // TODO: investigate 0 normal cases
            let normal = (point - pos).normalize_or_zero();
            Some(Collision2D {
                normal,
                target_point: pos + normal * self.radius,
            })
        } else {
            None
        }
    }
}
