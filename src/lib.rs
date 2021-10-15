pub use {components::*, resources::*};

#[cfg(any(feature = "2d_collisions", feature = "3d_collisions"))]
pub use collider::*;
#[cfg(any(feature = "2d_collisions", feature = "3d_collisions"))]
use components::colliders::*;
#[cfg(any(feature = "2d_collisions", feature = "3d_collisions"))]
mod collider;
mod components;
mod resources;
mod systems;

use crate::verlet_time_step::VerletTimeStep;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_prototype_debug_lines::DebugLinesPlugin;

/// Plugin for Verlet physics
pub struct BevyVerletPlugin {
    /// Custom time step for verlet physics, if set to `None` physics will run every frame
    pub time_step: Option<f64>,
}

impl Plugin for BevyVerletPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let system_set = SystemSet::new()
            .with_system(
                systems::points::update_points
                    .system()
                    .label("points")
                    .after("sticks"),
            )
            .with_system(systems::sticks::update_sticks.system().label("sticks"))
            .with_system(
                systems::sticks::handle_stick_constraints
                    .system()
                    .after("sticks"),
            );
        #[cfg(feature = "2d_collisions")]
        let system_set =
            system_set.with_system(systems::collisions::handle_collisions::<Collider2d>.system());
        #[cfg(feature = "3d_collisions")]
        let system_set =
            system_set.with_system(systems::collisions::handle_collisions::<Collider3d>.system());
        let system_set = if let Some(step) = self.time_step {
            app.insert_resource(VerletTimeStep::FixedDeltaTime(step));
            system_set.with_run_criteria(FixedTimestep::step(step))
        } else {
            app.insert_resource(VerletTimeStep::DeltaTime);
            system_set
        };
        app.add_system_set(system_set);
        #[cfg(feature = "debug")]
        {
            app.add_plugin(DebugLinesPlugin);
            app.add_system(systems::sticks::debug_draw_sticks.system().after("sticks"));
        }
    }
}

impl Default for BevyVerletPlugin {
    fn default() -> Self {
        Self {
            time_step: Some(0.02),
        }
    }
}

impl BevyVerletPlugin {
    pub fn new(time_step: f64) -> Self {
        Self {
            time_step: Some(time_step),
        }
    }
}
