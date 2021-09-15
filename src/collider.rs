use bevy::math::Vec3;
use bevy::prelude::GlobalTransform;

#[derive(Debug, Clone)]
pub struct Collision {
    pub normal: Vec3,
    pub target_point: Vec3,
}

pub trait Collider {
    fn is_within(&self, self_transform: &GlobalTransform, point: Vec3) -> Option<Collision>;
}
