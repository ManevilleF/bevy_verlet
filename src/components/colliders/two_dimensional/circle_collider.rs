use crate::colliders::is_point_in_circle;
use crate::{Collider, Collision};
use bevy::prelude::{GlobalTransform, Vec2, Vec3};

#[derive(Debug, Clone)]
pub struct CircleCollider {
    pub radius: f32,
}

impl Collider for CircleCollider {
    fn is_within(&self, self_transform: &GlobalTransform, point: Vec3) -> Option<Collision> {
        let pos = self_transform.translation;
        let pos_2d = Vec2::new(pos.x, pos.y);
        let point_2d = Vec2::new(point.x, point.y);
        if is_point_in_circle(point_2d, pos_2d, self.radius) {
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
