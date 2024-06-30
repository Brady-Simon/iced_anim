pub mod animate;
pub mod animation;
pub mod animation_builder;
pub mod curve;

pub use animate::Animate;
pub use animation_builder::{AnimationBuilder, DEFAULT_DURATION};
pub use curve::Curve;

#[cfg(feature = "derive")]
pub use iced_animation_builder_derive::Animate;
