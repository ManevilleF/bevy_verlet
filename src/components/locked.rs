use bevy::prelude::{Component, Reflect};

/// Component preventing application of  [`VerletPoint`][VerletPoint] physics.
///
/// [VerletPoint]: struct.VerletPoint.html
#[derive(Debug, Copy, Clone, Component, Reflect)]
pub struct VerletLocked;
