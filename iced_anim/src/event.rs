//! An event associated with an [`crate::Animated`] value.
//!
//! Spring events can represent three general types of events:
//! - A tick event that updates the animated value
//! - A target event that sets the animated target value
//! - A settle event that ends the animation early.
//!
//! This event can be passed to [`crate::Animated::update`] to update the current value.
//! You can also use the `From` impl to create a [`Event::Target`] from a
//! value, e.g. `Message::ChangeSize(5.0.into())` instead of
//! `Message::ChangeSize(Event::Target(5.0))`.
//!
//! ```rust
//! # use iced_anim::{Animated, spring::Motion, Event};
//! let mut spring_1 = Animated::spring(0.0, Motion::default());
//! spring_1.update(Event::Target(5.0));
//!
//! let mut spring_2 = Animated::spring(0.0, Motion::default());
//! spring_2.update(5.0.into());
//!
//! assert_eq!(spring_1.target(), &5.0);
//! assert_eq!(spring_2.target(), &5.0);
//! ```
use crate::Animate;
use std::time::Instant;

/// An event associated with an animated `Spring` value.
///
/// This event represents one of three things:
/// - A tick event that updates the spring's value, e.g. a frame is rendered
///   and the spring's value should be updated.
/// - A target event that sets the spring's target value, e.g. a user presses
///   a button and changes the target size of an animated value.
/// - A settle event that ends the animation early by jumping to the target
///   value.
///
/// This event can be passed to [`crate::Animated::update`] to update the spring's value.
#[derive(Debug, Clone, PartialEq)]
pub enum Event<T> {
    /// A tick event that updates the spring's value, e.g. a frame is rendered
    /// and the spring's value should be updated.
    Tick(Instant),
    /// A target event that sets the spring's target value, e.g. a user presses
    /// a button and changes the target size of an animated value.
    Target(T),
    /// Causes the spring to settle to its target value immediately. This is
    /// useful when the user has indicated they want reduced motion.
    Settle,
    /// Causes the spring to settle to a specific target value immediately.
    /// This is useful if you want to jump to a specific value without animating.
    SettleAt(T),
}

// Impl `Copy` for `Event` when `T` is `Copy`.
impl<T> Copy for Event<T> where T: Copy {}

// Any `From` usages should return a `Event::Target` variant.
impl<T> From<T> for Event<T>
where
    T: Animate,
{
    fn from(value: T) -> Self {
        Event::Target(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The `from` impl should return a [`Event::Target`] variant
    /// to allow users to do `5.0.into()` to create a [`Event::Target`].
    ///
    /// This should slightly reduce the amount of boilerplate needed to
    /// create a [`Event::Target`] from a value.
    #[test]
    fn from_impl_sets_target() {
        let update = Event::from(5.0);
        assert!(matches!(update, Event::Target(5.0)));
    }

    /// [`Event`] should implement `Copy` when `T` does.
    #[test]
    fn copy_impl() {
        let update = Event::from(5.0);
        let copy = update;
        assert_eq!(update, copy);
    }
}
