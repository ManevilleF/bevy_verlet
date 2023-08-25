use bevy::prelude::{Component, Reflect};
use std::ops::Deref;

/// Component adding a maximum tension to a [`VerletStick`].
///
/// The stick will break when its size becomes bigger than its `length`
/// multiplied by this factor
///
/// If you set it to `1.0` the stick will break almost instantly
/// If you set it to `2.0` the stick will break when stretched to twice its
/// `length`
///
/// [`VerletStick`]: crate::VerletStick
#[derive(Debug, Copy, Clone, Component, Reflect)]
pub struct VerletStickMaxTension(pub f32);

impl Default for VerletStickMaxTension {
    fn default() -> Self {
        Self(2.0)
    }
}

impl Deref for VerletStickMaxTension {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
