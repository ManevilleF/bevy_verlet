use crate::components::{VerletLocked, VerletPoint};
use crate::{Bounciness, Collider};
use bevy::ecs::component::Component;
use bevy::prelude::*;

pub fn handle_collisions<T: Collider + Component>(
    mut points_query: Query<
        (&mut Transform, &mut VerletPoint, Option<&Bounciness>),
        Without<VerletLocked>,
    >,
    collider_query: Query<(&GlobalTransform, &T, Option<&Bounciness>)>,
) {
    for (mut transform, mut point, bounciness) in points_query.iter_mut() {
        for (collider_transform, collider, collider_bounciness) in collider_query.iter() {
            if let Some(collision) = collider.is_within(collider_transform, transform.translation) {
                if let Some(old_pos) = point.old_position {
                    let velocity: Vec3 = transform.translation - old_pos;
                    let rebound_vector =
                        2.0 * velocity.dot(collision.normal) * collision.normal + velocity;
                    point.old_position = Some(
                        collision.target_point
                            + rebound_vector
                                * bounciness.copied().unwrap_or_default().0
                                * collider_bounciness.copied().unwrap_or_default().0,
                    );
                }
                transform.translation = collision.target_point;
            }
        }
    }
}