use crate::{Collider, Collision};
use bevy::prelude::{GlobalTransform, Vec3};
pub use {capsule_collider::*, sphere_collider::*};

mod capsule_collider;
mod sphere_collider;

pub enum Collider3d {
    SphereCollider(SphereCollider),
    CapsuleCollider(CapsuleCollider),
    CustomCollider(Box<dyn Collider + Send + Sync>),
}

impl Collider for Collider3d {
    fn is_within(&self, self_transform: &GlobalTransform, point: Vec3) -> Option<Collision> {
        match self {
            Collider3d::SphereCollider(c) => c.is_within(self_transform, point),
            Collider3d::CapsuleCollider(c) => c.is_within(self_transform, point),
            Collider3d::CustomCollider(c) => c.is_within(self_transform, point),
        }
    }
}
