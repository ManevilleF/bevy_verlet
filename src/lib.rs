//! # Verlet Integration for Bevy
//!
//! [![workflow](https://github.com/ManevilleF/bevy_verlet/actions/workflows/rust.yml/badge.svg)](https://github.com/ManevilleF/bevy_verlet/actions/workflows/rust.yml)
//!
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//! [![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//! [![Crates.io](https://img.shields.io/crates/v/bevy_verlet.svg)](https://crates.io/crates/bevy_verlet)
//! [![Docs.rs](https://docs.rs/bevy_verlet/badge.svg)](https://docs.rs/bevy_verlet)
//! [![dependency status](https://deps.rs/crate/bevy_verlet/0.4.0/status.svg)](https://deps.rs/crate/bevy_verlet)
//!
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
//! This feature will add a *system* drawing debug lines for every stick using [`bevy_prototype_debug_lines`](https://crates.io/crates/bevy_prototype_debug_lines)
//!
#![forbid(missing_docs)]
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
use bevy::time::FixedTimestep;
#[cfg(feature = "debug")]
use bevy_prototype_debug_lines::DebugLinesPlugin;

/// Prelude
pub mod prelude {
    pub use crate::components::*;
    pub use crate::resources::*;
    pub use crate::VerletPlugin;
}

/// Plugin for Verlet physics
#[derive(Debug, Copy, Clone, Default)]
pub struct VerletPlugin {
    /// Custom time step for verlet physics, if set to `None` physics will run every frame
    pub time_step: Option<f64>,
}

impl Plugin for VerletPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VerletConfig>();
        let system_set = SystemSet::new()
            .with_system(systems::points::update_points.label("VERLET_UPDATE_POINTS"))
            .with_system(
                systems::sticks::update_sticks
                    .label("VERLET_UPDATE_STICKS")
                    .after("VERLET_UPDATE_POINTS"),
            )
            .with_system(systems::sticks::handle_stick_constraints.after("VERLET_UPDATE_STICKS"));
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
            app.add_plugin(DebugLinesPlugin::default());
            app.add_system(systems::debug::debug_draw_sticks);
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
