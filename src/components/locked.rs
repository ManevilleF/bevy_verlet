use bevy::prelude::Component;

/// Component preventing application of  [`VerletPoint`][VerletPoint] physics.
///
/// [VerletPoint]: struct.VerletPoint.html
#[derive(Debug, Copy, Clone, Component)]
pub struct VerletLocked {}
