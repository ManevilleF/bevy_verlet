use crate::colliders::is_point_in_sphere;
use crate::{Collider, Collision};
use bevy::prelude::{GlobalTransform, Vec3};

#[derive(Debug, Clone)]
pub struct SphereCollider {
    pub radius: f32,
}

impl Collider for SphereCollider {
    fn is_within(&self, self_transform: &GlobalTransform, point: Vec3) -> Option<Collision> {
        let pos = self_transform.translation;
        if is_point_in_sphere(point, pos, self.radius) {
            // TODO: investigate 0 normal cases
            let normal = (point - pos).normalize_or_zero();
            Some(Collision {
                normal,
                target_point: pos + normal * self.radius,
            })
        } else {
            None
        }
    }
}
