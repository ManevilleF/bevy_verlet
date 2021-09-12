use bevy::math::{Vec2, Vec3};

#[derive(Debug, Clone)]
pub struct VerletPoint2D {
    pub(crate) current_position: Vec2,
    pub(crate) old_position: Vec2,
}

impl VerletPoint2D {
    pub fn new(position: Vec2) -> Self {
        Self {
            current_position: position,
            old_position: position,
        }
    }

    pub fn direction(&self) -> Vec2 {
        self.velocity().normalize_or_zero()
    }

    pub fn velocity(&self) -> Vec2 {
        self.current_position - self.old_position
    }

    pub fn vec3_position(&self) -> Vec3 {
        Vec3::new(self.current_position.x, self.current_position.y, 0.)
    }
}

impl Default for VerletPoint2D {
    fn default() -> Self {
        Self {
            current_position: Default::default(),
            old_position: Default::default(),
        }
    }
}
