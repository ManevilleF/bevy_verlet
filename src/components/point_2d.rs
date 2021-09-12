use bevy::math::Vec2;

#[derive(Debug, Clone)]
pub struct VerletPoint2 {
    pub(crate) current_position: Vec2,
    pub(crate) old_position: Vec2,
}

impl VerletPoint2 {
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
}

impl Default for VerletPoint2 {
    fn default() -> Self {
        Self {
            current_position: Default::default(),
            old_position: Default::default(),
        }
    }
}
