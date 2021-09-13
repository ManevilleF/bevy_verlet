pub use {bundles::*, components::*, resources::*};

mod bundles;
mod components;
mod resources;
mod systems;

use bevy::core::FixedTimestep;
use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_prototype_debug_lines::DebugLinesPlugin;

pub struct BevyVerletPlugin {
    pub time_step: Option<f64>,
}

impl Plugin for BevyVerletPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let system_set = SystemSet::new()
            .with_system(systems::points::update_points.system().label("points"))
            .with_system(
                systems::sticks::update_sticks
                    .system()
                    .label("sticks")
                    .after("points"),
            );
        let system_set = if let Some(step) = self.time_step {
            system_set.with_run_criteria(FixedTimestep::step(step))
        } else {
            system_set
        };
        app.add_system_set(system_set);
        #[cfg(feature = "debug")]
        {
            app.add_plugin(DebugLinesPlugin);
            app.add_system(systems::sticks::debug_draw_sticks.system().after("points"));
        }
    }
}

impl Default for BevyVerletPlugin {
    fn default() -> Self {
        Self { time_step: None }
    }
}
