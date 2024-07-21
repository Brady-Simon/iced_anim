//! An event associated with an animated `Spring` value.
//!
//! Spring events can represent two general types of events:
//! - A tick event that updates the spring's value
//! - A target event that sets the spring's target value
//!
//! This event can be passed to `Spring::update` to update the spring's value.
//! You can also use the `From` impl to create a `SpringEvent::Target` from a
//! value, e.g. `Message::ChangeSize(5.0.into())` instead of
//! `Message::ChangeSize(SpringEvent::Target(5.0))`.
//!
//! ```rust
//! # use iced_anim::{Spring, SpringEvent};
//! let mut spring_1 = Spring::new(0.0);
//! spring_1.update(SpringEvent::Target(5.0));
//!
//! let mut spring_2 = Spring::new(0.0);
//! spring_2.update(5.0.into());
//!
//! assert_eq!(spring_1.target(), &5.0);
//! assert_eq!(spring_2.target(), &5.0);
//! ```
use std::time::Instant;

use crate::Animate;

/// An event associated with an animated `Spring` value.
///
/// This event represents one of two things:
/// - A tick event that updates the spring's value, e.g. a frame is rendered
/// and the spring's value should be updated.
/// - A target event that sets the spring's target value, e.g. a user presses
/// a button and changes the target size of an animated value.
///
/// This event can be passed to `Spring::update` to update the spring's value.
#[derive(Debug, Clone, PartialEq)]
pub enum SpringEvent<T> {
    /// A tick event that updates the spring's value, e.g. a frame is rendered
    /// and the spring's value should be updated.
    Tick(Instant),
    /// A target event that sets the spring's target value, e.g. a user presses
    /// a button and changes the target size of an animated value.
    Target(T),
}

// Impl `Copy` for `SpringEvent` when `T` is `Copy`.
impl<T> Copy for SpringEvent<T> where T: Copy {}

// Any `From` usages should return a `SpringEvent::Target` variant.
impl<T> From<T> for SpringEvent<T>
where
    T: Animate,
{
    fn from(value: T) -> Self {
        SpringEvent::Target(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The `from` impl should return a `SpringEvent::Target` variant
    /// to allow users to do `5.0.into()` to create a `SpringEvent::Target`.
    ///
    /// This should slightly reduce the amount of boilerplate needed to
    /// create a `SpringEvent::Target` from a value.
    #[test]
    fn from_impl_sets_target() {
        let update = SpringEvent::from(5.0);
        assert!(matches!(update, SpringEvent::Target(5.0)));
    }

    /// `SpringEvent` should implement `Copy` when `T` does.
    #[test]
    fn copy_impl() {
        let update = SpringEvent::from(5.0);
        let copy = update;
        assert_eq!(update, copy);
    }
}
