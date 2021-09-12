pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;

pub struct BevyVerletPlugin {}

impl Plugin for BevyVerletPlugin {
    fn build(&self, app: &mut AppBuilder) {
        #[cfg(feature = "2D")]
        {
            app.add_system(
                systems::sticks::update_2d_sticks
                    .system()
                    .label("2d_sticks"),
            )
            .add_system(systems::points::update_2d_points.system().after("2d_sticks"));
        }
    }
}
