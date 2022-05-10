<!-- cargo-sync-readme start -->

# Verlet Integration for Bevy

[![workflow](https://github.com/ManevilleF/bevy_verlet/actions/workflows/rust.yml/badge.svg)](https://github.com/ManevilleF/bevy_verlet/actions/workflows/rust.yml)

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![Crates.io](https://img.shields.io/crates/v/bevy_verlet.svg)](https://crates.io/crates/bevy_verlet)
[![Docs.rs](https://docs.rs/bevy_verlet/badge.svg)](https://docs.rs/bevy_verlet)
[![dependency status](https://deps.rs/crate/bevy_verlet/0.2.0/status.svg)](https://deps.rs/crate/bevy_verlet)

Simple Verlet points and sticks implementation for bevy.

If you are looking for cloth physics, please check [bevy_cloth](https://github.com/ManevilleF/bevy_cloth) instead,

## Features

You can simply add a `VerletPoint` component on any entity with a `Transform` and the verlet physics will apply.

Connect points using `VerletStick` to constrain movement (see [examples](./examples)).

Lock some points by adding the `VerletLocked` component on a `VerletPoint` entity.

Customize *friction* and *gravity* with the `VerletConfig` resource.

> Works in 2D and 3D.

## Cargo features

1. `debug`

This feature will add a *system* drawing debug lines for every stick using [`bevy_prototype_debug_lines`](https://crates.io/crates/bevy_prototype_debug_lines)


<!-- cargo-sync-readme end -->

## Examples

### 2D

1. 2D Line

   `cargo run --example 2d_line --features "debug"`

   ![Alt](./docs/demo_line.gif "demo gif")

3. 2D cloth

   `cargo run --example 2d_cloth --features "debug"`

   ![Alt](./docs/demo_cloth.gif "demo gif")

4. 2D cloth cutting

   `cargo run --example 2d_cloth_cutter --features "debug" --release`
   
   > Note: the release flag is necessary for smooth running
   
   ![Alt](./docs/demo_cloth_cutting.gif "demo gif")

### 3D

* `cargo run --example 3d_line --features "debug"`
* `cargo run --example 3d_cloth --features "debug"`

## Credits

Inspired by:
- This [Sebastian Lague video](https://www.youtube.com/watch?v=PGk0rnyTa1U)
- This [Coding Math course](https://www.youtube.com/watch?v=3HjO_RGIjCU)
