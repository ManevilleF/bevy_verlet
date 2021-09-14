use bevy::math::{Vec2, Vec3};
use bevy::prelude::GlobalTransform;

pub struct Collision {
    pub normal: Vec3,
    pub target_point: Vec3,
}

pub struct Collision2D {
    pub normal: Vec2,
    pub target_point: Vec2,
}

pub trait Collider {
    fn is_within(&self, self_transform: &GlobalTransform, point: Vec3) -> Option<Collision>;
}

pub trait Collider2D {
    fn is_within(&self, self_transform: &GlobalTransform, point: Vec2) -> Option<Collision2D>;
}
