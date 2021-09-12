use bevy::prelude::Vec3;

#[derive(Debug, Clone)]
pub struct VerletPoint3Influence {
    pub new_position: Option<Vec3>,
}

impl VerletPoint3Influence {
    pub fn apply_new_position(&mut self, new_position: Vec3) {
        match self.new_position {
            None => self.new_position = Some(new_position),
            Some(pos) => self.new_position = Some((pos + new_position) / 2.),
        }
    }
}

impl Default for VerletPoint3Influence {
    fn default() -> Self {
        Self { new_position: None }
    }
}
