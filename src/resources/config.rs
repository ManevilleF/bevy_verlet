use bevy::math::{Vec2, Vec3};

/// Verlet physics configuration
#[derive(Debug, Copy, Clone)]
pub struct VerletConfig {
    /// Custom gravity, classic (0, -9.81, 0) is used by default
    pub gravity: Vec3,
    /// Custom friction to apply to velocity, 0.01 by default
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
            friction: 0.02,
            sticks_computation_depth: 2,
            parallel_processing_batch_size: None,
        }
    }
}

impl VerletConfig {
    pub(crate) fn friction_coefficient(&self) -> f32 {
        1.0 - self.friction
    }

    /// Retrieves the `gravity` field without the `z` axis
    pub fn gravity_2d(&self) -> Vec2 {
        Vec2::new(self.gravity.x, self.gravity.y)
    }
}
