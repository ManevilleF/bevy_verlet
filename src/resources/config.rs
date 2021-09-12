use bevy::math::{Vec2, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct VerletConfig {
    pub gravity: Vec3,
    pub friction: f32,
    pub max_velocity: f32,
}

impl Default for VerletConfig {
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0., -9.81, 0.),
            friction: 0.01,
            max_velocity: 10.,
        }
    }
}

impl VerletConfig {
    pub(crate) fn friction_coefficient(&self) -> f32 {
        1.0 - self.friction
    }

    pub fn gravity_2d(&self) -> Vec2 {
        Vec2::new(self.gravity.x, self.gravity.y)
    }
}
