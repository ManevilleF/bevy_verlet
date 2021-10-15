use crate::{Collider, Collision};
use bevy::prelude::{GlobalTransform, Vec3};
pub use {circle_collider::*, rectangle_collider::*, stadium_collider::*, triangle_collider::*};

mod circle_collider;
mod rectangle_collider;
mod stadium_collider;
mod triangle_collider;

pub enum Collider2d {
    Circle(CircleCollider),
    Stadium(StadiumCollider),
    Triangle(TriangleCollider),
    Custom(Box<dyn Collider + Send + Sync>),
}

impl Collider for Collider2d {
    fn is_within(&self, self_transform: &GlobalTransform, point: Vec3) -> Option<Collision> {
        match self {
            Collider2d::Circle(c) => c.is_within(self_transform, point),
            Collider2d::Stadium(c) => c.is_within(self_transform, point),
            Collider2d::Triangle(c) => c.is_within(self_transform, point),
            Collider2d::Custom(c) => c.is_within(self_transform, point),
        }
    }
}