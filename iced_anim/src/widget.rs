//! A collection of widgets whose style are implicitly animated.
//!
//! This means the style of these widgets animate changes automatically without any further work
//! from the user. These widgets are generally modified versions of built-in widgets in Iced that
//! have been fitted to include animations by default.
//!
//! > Note: this module is only available when the `widgets` feature is enabled.
//!
//! Animations work out of the box, but there are a couple of things to keep in mind:
//!
//! - Styles with enum representations (including `Option`) don't animate between variants.
//!   This mostly comes down to being unable to cleanly represent a transition between different
//!   variants since the underlying data between them is different. For example, if you're creating
//!   a button style with a particular color, then you would want all backgrounds in that style to
//!   only be an [`iced::Color`] and not an [`iced::Gradient`]. Use default or empty values like
//!   [`iced::Color::TRANSPARENT`] in place of [`None`] to ensure optional values are animated,
//!   since [`None`] counts as a different variant.
//! - You can disable animations by passing a [`Motion`] with a duration of `0.0` to the
//!   `motion` method, but there may be a more ergonomic way to do this in the future.
pub mod animated_state;
pub mod button;
pub mod svg;

pub use animated_state::AnimatedState;
pub use button::{button, Button};
pub use svg::{svg, Svg};
