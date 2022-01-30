use bevy::prelude::{Component, Reflect, Vec3};

/// Main verlet physics component.
/// Any entity with this component will have physics applied to it
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct VerletPoint {
    pub(crate) old_position: Option<Vec3>,
}
