use bevy::prelude::{Component, Reflect};

/// Marker component preventing application of [`VerletPoint`] physics.
///
/// [`VerletPoint`]: crate::VerletPoint
#[derive(Debug, Copy, Clone, Component, Reflect)]
pub struct VerletLocked;
