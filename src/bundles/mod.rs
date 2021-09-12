#[cfg(feature = "3D")]
pub use point_pbr_bundle::*;
#[cfg(feature = "2D")]
pub use point_sprite_bundle::*;

#[cfg(feature = "3D")]
mod point_pbr_bundle;
#[cfg(feature = "2D")]
mod point_sprite_bundle;
