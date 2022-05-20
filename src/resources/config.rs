use bevy::math::{Vec2, Vec3, Vec3Swizzles};

/// Verlet physics configuration
#[derive(Debug, Copy, Clone)]
pub struct VerletConfig {
    /// Custom gravity, classic (0, -9.81, 0) is used by default
    pub gravity: Vec3,
    /// Custom friction to apply to velocity, 0.01 by default
    ///
    /// Note: will be clamped between 0.0 and 1.0
    pub friction: f32,
    /// Sets the number of sticks computation iteration.
    /// The higher the value, the more precision and less elasticity for the sticks but the cost is increased
    pub sticks_computation_depth: u8,
    /// Enables parallel computing for points, setting the parallel batch size
    pub parallel_processing_batch_size: Option<usize>,
}

impl Default for VerletConfig {
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0., -9.81, 0.),
            friction: 0.05,
            sticks_computation_depth: 2,
            parallel_processing_batch_size: None,
        }
    }
}

impl VerletConfig {
    #[must_use]
    #[inline]
    pub(crate) fn friction_coefficient(&self) -> f32 {
        1.0 - self.friction.clamp(0.0, 1.0)
    }

    /// Retrieves the `gravity` field without the `z` axis
    #[must_use]
    #[inline]
    pub fn gravity_2d(&self) -> Vec2 {
        self.gravity.xy()
    }
}
