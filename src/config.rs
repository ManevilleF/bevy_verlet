use bevy::{
    ecs::prelude::Resource,
    math::{Vec2, Vec3, Vec3Swizzles},
};

/// Verlet physics configuration
#[derive(Debug, Copy, Clone, Resource)]
pub struct VerletConfig {
    /// Custom gravity, classic (0, -9.81, 0) is used by default
    pub gravity: Vec3,
    /// Custom friction to apply to velocity, slowing down the simulation and
    /// reducing elasticity.
    ///
    /// Note: will be clamped between 0.0 and 1.0
    pub friction: f32,
    /// Sets the number of sticks computation iteration.
    /// The higher the value, the more precision and less elasticity for the
    /// sticks but the cost is increased
    pub sticks_computation_depth: u8,
    /// Enables parallel computing for points
    pub parallel_processing: bool,
}

impl Default for VerletConfig {
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0., -9.81, 0.),
            friction: 0.01,
            sticks_computation_depth: 5,
            parallel_processing: true,
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
