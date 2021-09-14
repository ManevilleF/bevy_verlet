use bevy::math::{Vec2, Vec3};

mod capsule_collider;
mod circle_collider;
mod sphere_collider;
mod stadium_collider;

mod lines {
    use super::*;

    pub fn point_to_line_projection_2d(point: Vec2, (a, b): (Vec2, Vec2)) -> f32 {
        let ab = b - a;
        Vec2::dot(point - a, ab) / Vec2::dot(ab, ab)
    }

    pub fn point_to_line_projection_3d(point: Vec3, (a, b): (Vec3, Vec3)) -> f32 {
        let ab = b - a;
        Vec3::dot(point - a, ab) / Vec3::dot(ab, ab)
    }

    pub fn closest_point_to_segment_2d(point: Vec2, (a, b): (Vec2, Vec2)) -> Vec2 {
        a + (b - a) * point_to_line_projection_2d(point, (a, b)).clamp(0., 1.)
    }

    pub fn closest_point_to_segment_3d(point: Vec3, (a, b): (Vec3, Vec3)) -> Vec3 {
        a + (b - a) * point_to_line_projection_3d(point, (a, b)).clamp(0., 1.)
    }
}

pub fn is_point_in_circle(point: Vec2, center: Vec2, radius: f32) -> bool {
    point.distance_squared(center) < radius * radius
}

pub fn is_point_in_sphere(point: Vec3, center: Vec3, radius: f32) -> bool {
    point.distance_squared(center) < radius * radius
}
