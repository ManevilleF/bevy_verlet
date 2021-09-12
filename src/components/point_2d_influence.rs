use bevy::prelude::Vec2;

#[derive(Debug, Clone)]
pub struct VerletPoint2Influence {
    pub new_position: Option<Vec2>,
}

impl VerletPoint2Influence {
    pub fn apply_new_position(&mut self, new_position: Vec2) {
        match self.new_position {
            None => self.new_position = Some(new_position),
            Some(pos) => self.new_position = Some((pos + new_position) / 2.),
        }
    }
}

impl Default for VerletPoint2Influence {
    fn default() -> Self {
        Self { new_position: None }
    }
}
