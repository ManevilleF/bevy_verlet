use bevy::math::Vec3;

#[derive(Debug, Clone)]
pub struct VerletPoint {
    pub(crate) old_position: Option<Vec3>,
}

impl Default for VerletPoint {
    fn default() -> Self {
        Self {
            old_position: None,
        }
    }
}
