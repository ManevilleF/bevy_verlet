use crate::VerletPoint;
use bevy::prelude::{Bundle, PbrBundle};

#[derive(Bundle)]
pub struct VerletPointPbrBundle {
    pub verlet_point: VerletPoint,
    #[bundle]
    pub pbr_bundle: PbrBundle,
}

impl Default for VerletPointPbrBundle {
    fn default() -> Self {
        Self {
            verlet_point: Default::default(),
            pbr_bundle: Default::default(),
        }
    }
}
