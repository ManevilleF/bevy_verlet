use bevy::prelude::{Component, Entity, Reflect};

/// Constraint component between two [`VerletPoint`].
///
/// [`VerletPoint`]: crate::VerletPoint
#[derive(Debug, Clone, Component, Reflect)]
pub struct VerletStick {
    /// Start `VerletPoint` entity
    pub point_a_entity: Entity,
    /// End `VerletPoint` entity
    pub point_b_entity: Entity,
    /// Target stick length
    pub length: f32,
}

impl VerletStick {
    #[inline]
    #[must_use]
    /// Returns [`point_a_entity`, `point_b_entity`]
    pub const fn entities(&self) -> [Entity; 2] {
        [self.point_a_entity, self.point_b_entity]
    }
}
