use crate::{VerletPoint3D, VerletPoint3Influence};
use bevy::prelude::{Bundle, PbrBundle};

#[derive(Bundle)]
pub struct VerletPointPbrBundle {
    pub verlet_point: VerletPoint3D,
    pub verlet_point_influence: VerletPoint3Influence,
    #[bundle]
    pub pbr_bundle: PbrBundle,
}

impl Default for VerletPointPbrBundle {
    fn default() -> Self {
        Self {
            verlet_point: Default::default(),
            verlet_point_influence: Default::default(),
            pbr_bundle: Default::default(),
        }
    }
}
