use bevy::math::Vec3;

#[derive(Debug, Clone)]
pub struct VerletPoint3D {
    pub(crate) current_position: Vec3,
    pub(crate) old_position: Vec3,
}

impl VerletPoint3D {
    pub fn new(position: Vec3) -> Self {
        Self {
            current_position: position,
            old_position: position,
        }
    }

    pub fn direction(&self) -> Vec3 {
        self.velocity().normalize_or_zero()
    }

    pub fn velocity(&self) -> Vec3 {
        self.current_position - self.old_position
    }
}

impl Default for VerletPoint3D {
    fn default() -> Self {
        Self {
            current_position: Default::default(),
            old_position: Default::default(),
        }
    }
}
