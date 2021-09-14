use crate::colliders::is_point_in_sphere;
use crate::colliders::lines::closest_point_to_segment_3d;
use crate::{Collider, Collision};
use bevy::prelude::{GlobalTransform, Vec3};

#[derive(Debug, Clone)]
pub struct CapsuleCollider {
    pub start: Vec3,
    pub end: Vec3,
    pub radius: f32,
}

impl Collider for CapsuleCollider {
    fn is_within(&self, self_transform: &GlobalTransform, point: Vec3) -> Option<Collision> {
        let pos = self_transform.translation;
        let projected_point =
            closest_point_to_segment_3d(point, (pos + self.start, pos + self.end));
        if is_point_in_sphere(point, projected_point, self.radius) {
            // TODO: investigate 0 normal cases
            let normal = (point - projected_point).normalize_or_zero();
            Some(Collision {
                normal,
                target_point: projected_point + normal * self.radius,
            })
        } else {
            None
        }
    }
}
