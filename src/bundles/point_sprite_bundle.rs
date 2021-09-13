use crate::VerletPoint;
use bevy::prelude::{Bundle, SpriteBundle};

#[derive(Bundle, Clone)]
pub struct VerletPointSpriteBundle {
    pub verlet_point: VerletPoint,
    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

impl Default for VerletPointSpriteBundle {
    fn default() -> Self {
        Self {
            verlet_point: Default::default(),
            sprite_bundle: Default::default(),
        }
    }
}
