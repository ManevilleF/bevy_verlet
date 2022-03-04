use bevy::prelude::{Component, Entity, Reflect};

/// Constraint component between two [`VerletPoint`][VerletPoint].
///
/// [VerletPoint]: struct.VerletPoint.html
#[derive(Debug, Clone, Component, Reflect)]
pub struct VerletStick {
    /// Start `VerletPoint` entity
    pub point_a_entity: Entity,
    /// End `VerletPoint` entity
    pub point_b_entity: Entity,
    /// Target stick length
    pub length: f32,
}
