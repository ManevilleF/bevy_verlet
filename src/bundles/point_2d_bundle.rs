use crate::{VerletPoint2, VerletPoint2Influence};
use bevy::prelude::{Bundle, SpriteBundle};

#[derive(Bundle, Clone)]
pub struct VerletPointSpriteBundle {
    pub verlet_point: VerletPoint2,
    pub verlet_point_influence: VerletPoint2Influence,
    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

impl Default for VerletPointSpriteBundle {
    fn default() -> Self {
        Self {
            verlet_point: Default::default(),
            verlet_point_influence: Default::default(),
            sprite_bundle: Default::default(),
        }
    }
}
