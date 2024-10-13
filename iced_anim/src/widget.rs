//! A collection of widgets whose style are implicitly animated.
//!
//! This means the style of these widgets animate changes automatically without any further work
//! from the user. These widgets are generally modified versions of built-in widgets in Iced that
//! have been fitted to include animations by default.
//!
//! > Note: this module is only available when the `widgets` feature is enabled.
mod animated_state;
pub mod button;
pub mod checkbox;
// pub mod svg;

pub use animated_state::AnimatedState;
pub use button::{button, Button};
pub use checkbox::{checkbox, Checkbox};
// pub use svg::{svg, Svg};
