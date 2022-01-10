use bevy::prelude::{Component, Vec3};

/// Main verlet physics component.
/// Any entity with this component will have physics applied to it
#[derive(Debug, Clone, Component)]
pub struct VerletPoint {
    pub(crate) old_position: Option<Vec3>,
}

impl Default for VerletPoint {
    fn default() -> Self {
        Self { old_position: None }
    }
}
