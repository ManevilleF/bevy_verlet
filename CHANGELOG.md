# CHANGELOG

## Unreleased

## 0.9.0

* Bevy 0.14 (#17)
* Physics improvements (#16)

## 0.8.0

* Bevy 0.13 support (#15)

## 0.7.0

* Bevy 0.12 support (#14)
* Verlet physics always happens on `FixedUpdate` schedule (#12)

## 0.6.1

* Performance improvements (#12)

## 0.6.0

* `bevy` 0.11

## 0.5.0

* `bevy` 0.9
* parallel processing batch size is usef for stick constraints

## 0.4.0

* `bevy` 0.8

## 0.3.0

* Fixed examples in debug mode
* Added `prelude` module
* docs update
* `bevy` 0.7
* Fixed extra elasticity issue around fixed points
* (**BREAKING**) Renamed `BevyVerletPlugin` to `VerletPlugin`
* Improved *cloth cutter* example

## 0.2.0

* Bevy 0.6
* Rust 2021 edition
* Clippy extra restrictions

## 0.1.1

* The `VerletConfig::parallel_processing_batch_size` is only used for
`VerletPoint` processing
* The cloth cutting example improved and scaled up
* The debug stick printing system moved in a private `debug` module

## 0.1.0

First version
