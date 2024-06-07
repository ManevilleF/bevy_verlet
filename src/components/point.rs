use bevy::prelude::{Component, Reflect, Vec3};

/// Main verlet physics component.
/// Any entity with this component will have physics applied to it
#[derive(Debug, Clone, Component, Reflect)]
pub struct VerletPoint {
    pub(crate) old_position: Option<Vec3>,
    /// Point mass, defaults to 1.0
    pub mass: f32,
}

impl Default for VerletPoint {
    fn default() -> Self {
        Self {
            old_position: None,
            mass: 1.0,
        }
    }
}

impl VerletPoint {
    /// Creates a point with a custom mass value
    ///
    /// # Panics
    ///
    /// Panics if `mass` is lesser or equal to zero
    #[inline]
    #[must_use]
    pub fn new(mass: f32) -> Self {
        assert!(mass > 0.0);
        Self {
            mass,
            old_position: None,
        }
    }
}
