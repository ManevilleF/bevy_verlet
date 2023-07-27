//! Simple Verlet points and sticks implementation for bevy.
//!
//! If you are looking for cloth physics, please check [`bevy_silk`](https://github.com/ManevilleF/bevy_silk) instead,
//!
//! ## Bevy versions
//!
//!  | `bevy_verlet` | `bevy`    |
//!  |---------------|-----------|
//!  | 0.1.x         | 0.5.x     |
//!  | 0.2.x         | 0.6.x     |
//!  | 0.3.x         | 0.7.x     |
//!  | 0.4.x         | 0.8.x     |
//!  | 0.5.x         | 0.9.x     |
//!  | 0.6.x         | 0.11.x    |
//!
//! ## Features
//!
//! You can simply add a `VerletPoint` component on any entity with a `Transform` and the verlet physics will apply.
//!
//! Connect points using `VerletStick` to constrain movement (see [examples](./examples)).
//!
//! Lock some points by adding the `VerletLocked` component on a `VerletPoint` entity.
//!
//! Customize *friction* and *gravity* with the `VerletConfig` resource.
//!
//! > Works in 2D and 3D.
//!
//! ## Cargo features
//!
//! 1. `debug`
//!
//! This feature will add a *system* drawing debug lines for every stick using bevy gizmos
//!
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![warn(
    clippy::nursery,
    clippy::pedantic,
    nonstandard_style,
    rustdoc::broken_intra_doc_links
)]
#![allow(
    clippy::default_trait_access,
    clippy::module_name_repetitions,
    clippy::redundant_pub_crate
)]

pub use {components::*, resources::*};

mod components;
mod resources;
mod systems;

use crate::verlet_time_step::VerletTimeStep;
use bevy::log;
use bevy::prelude::*;
use bevy::time::common_conditions::on_fixed_timer;
use std::time::Duration;
use systems::{
    points::update_points,
    sticks::{handle_stick_constraints, update_sticks},
};

/// Prelude
pub mod prelude {
    pub use crate::components::*;
    pub use crate::resources::*;
    pub use crate::VerletPlugin;
}
/// Plugin for Verlet physics
#[derive(Debug, Copy, Clone, Default)]
pub struct VerletPlugin {
    /// Custom time step in seconds for verlet physics, if set to `None` physics will run every frame
    pub time_step: Option<f64>,
}

impl Plugin for VerletPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VerletConfig>();
        if let Some(step) = self.time_step {
            app.add_systems(
                FixedUpdate,
                (update_points, update_sticks, handle_stick_constraints)
                    .chain()
                    .run_if(on_fixed_timer(Duration::from_secs_f64(step))),
            );
            app.insert_resource(VerletTimeStep::FixedDeltaTime(step));
        } else {
            app.add_systems(
                Update,
                (update_points, update_sticks, handle_stick_constraints).chain(),
            );
            app.insert_resource(VerletTimeStep::DeltaTime);
        };
        #[cfg(feature = "debug")]
        {
            app.add_systems(PostUpdate, systems::debug::debug_draw_sticks);
        }
        app.register_type::<VerletPoint>()
            .register_type::<VerletLocked>()
            .register_type::<VerletStick>()
            .register_type::<VerletStickMaxTension>();
        log::info!("Loaded verlet plugin");
    }
}

impl VerletPlugin {
    /// Instantiates a new plugin with a custom time step
    #[must_use]
    #[inline]
    pub const fn new(time_step: f64) -> Self {
        Self {
            time_step: Some(time_step),
        }
    }
}
