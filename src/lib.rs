//! Simple Verlet points and sticks implementation for bevy.
//!
//! If you are looking for cloth physics, please check
//! [`bevy_silk`](https://github.com/ManevilleF/bevy_silk) instead
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
//!  | 0.7.x         | 0.12.x    |
//!  | 0.8.x         | 0.13.x    |
//!
//! ## Features
//!
//! You can simply add a `VerletPoint` component on any entity with a
//! `Transform` and the verlet physics will apply.
//!
//! Connect points using `VerletStick` to constrain movement (see
//! [examples](./examples)).
//!
//! Lock some points by adding the `VerletLocked` component on a `VerletPoint`
//! entity.
//!
//! Customize *friction* and *gravity* with the `VerletConfig` resource.
//!
//! > Works in 2D and 3D.
//!
//! ## Cargo features
//!
//! 1. `debug`
//!
//! This feature will add a *system* drawing debug lines for every stick using
//! bevy gizmos
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

pub use components::*;
pub use config::*;

mod components;
mod config;
mod systems;

use bevy::{log, prelude::*, time::common_conditions::on_timer};
use std::time::Duration;
use systems::{
    points::update_points,
    sticks::{handle_stick_constraints, update_sticks},
};

/// Prelude
pub mod prelude {
    pub use crate::{components::*, config::*, VerletPlugin};
}
/// Plugin for Verlet physics
#[derive(Debug, Copy, Clone, Default)]
pub struct VerletPlugin {
    /// Custom time step in seconds for verlet physics, if set to `None` physics
    /// will run every [`FixedUpdate`] frame
    pub time_step: Option<f64>,
}

impl Plugin for VerletPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VerletConfig>();
        let system_set = (update_points, update_sticks, handle_stick_constraints).chain();
        if let Some(step) = self.time_step {
            app.add_systems(
                FixedUpdate,
                system_set.run_if(on_timer(Duration::from_secs_f64(step))),
            );
        } else {
            app.add_systems(FixedUpdate, system_set);
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
