use crate::{Animate, Spring, Transition};

/// The type of animation that will be used to animate a value.
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationType<T> {
    /// An animation that uses [`Spring`] physics to enable responsivity.
    /// Use springs when the user is directly interacting with the animation
    /// or if there is lots of motion in the animation that might often change
    /// direction.
    Spring(Spring<T>),
    /// An animation that uses a [`Transition`] to animate between two values.
    /// This is primarily a set of easing functions that can be used to animate
    /// between two values and is good for predictable animations.
    Transition(Transition<T>),
}

impl<T> From<Spring<T>> for AnimationType<T>
where
    T: Animate,
{
    fn from(value: Spring<T>) -> Self {
        AnimationType::Spring(value)
    }
}

impl<T> From<Transition<T>> for AnimationType<T>
where
    T: Animate,
{
    fn from(value: Transition<T>) -> Self {
        AnimationType::Transition(value)
    }
}
