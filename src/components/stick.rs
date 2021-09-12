use bevy::prelude::Entity;

#[derive(Debug, Clone)]
pub struct VerletStick {
    pub point_a_entity: Entity,
    pub point_b_entity: Entity,
    pub length: f32,
}
