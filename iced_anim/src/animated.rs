use std::time::{Duration, Instant};

use crate::{
    spring::Motion,
    transition::{Curve, Transition},
    Animate, Event, Spring,
};

/// Designed to wrap an [`Animate`] value and enable animating changes to it.
#[derive(Debug, Clone, PartialEq)]
pub struct Animated<T> {
    /// The animation that will be used to animate the value.
    animation: AnimationType<T>,
}

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

impl<T> Animated<T>
where
    T: Animate,
{
    pub fn new(value: impl Into<AnimationType<T>>) -> Self {
        Self {
            animation: value.into(),
        }
    }

    pub fn spring(value: T, motion: Motion) -> Self {
        Self {
            animation: AnimationType::Spring(Spring::new(value).with_motion(motion)),
        }
    }

    pub fn transition(value: T, curve: Curve) -> Self {
        Self {
            animation: AnimationType::Transition(Transition::new(value).with_curve(curve)),
        }
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        match &mut self.animation {
            AnimationType::Spring(spring) => {
                let motion = spring.motion().with_duration(duration);
                spring.set_motion(motion);
            }
            AnimationType::Transition(transition) => {
                transition.set_duration(duration);
            }
        }

        self
    }

    pub fn update(&mut self, event: Event<T>) {
        match &mut self.animation {
            AnimationType::Spring(spring) => spring.update(event),
            AnimationType::Transition(transition) => transition.update(event),
        }
    }

    /// Whether this animated value is still undergoing an animation.
    pub fn is_animating(&self) -> bool {
        match &self.animation {
            AnimationType::Spring(spring) => spring.has_energy(),
            AnimationType::Transition(transition) => transition.is_animating(),
        }
    }

    /// Returns a reference to the current `value` of the animated value.
    pub fn value(&self) -> &T {
        match &self.animation {
            AnimationType::Spring(spring) => spring.value(),
            AnimationType::Transition(transition) => transition.value(),
        }
    }

    /// Returns a reference to the current `target` of the animated value.
    /// This is the final value that the animation is moving towards.
    pub fn target(&self) -> &T {
        match &self.animation {
            AnimationType::Spring(spring) => spring.target(),
            AnimationType::Transition(transition) => transition.target(),
        }
    }

    /// Sets the `target` value of the animation.
    pub fn set_target(&mut self, target: T) {
        match &mut self.animation {
            AnimationType::Spring(spring) => spring.set_target(target),
            AnimationType::Transition(transition) => transition.set_target(target),
        }
    }

    /// Sets the `target` value of the animation, and returns the updated animation.
    pub fn to(mut self, target: T) -> Self {
        match &mut self.animation {
            AnimationType::Spring(spring) => spring.set_target(target),
            AnimationType::Transition(transition) => transition.set_target(target),
        }

        self
    }

    pub fn settle(&mut self) {
        match &mut self.animation {
            AnimationType::Spring(spring) => spring.settle(),
            AnimationType::Transition(transition) => transition.settle(),
        }
    }

    pub fn tick(&mut self, now: Instant) {
        match &mut self.animation {
            AnimationType::Spring(spring) => spring.tick(now),
            AnimationType::Transition(transition) => transition.tick(now),
        }
    }

    /// Interrupts the existing animation and starts a new one with the new `target`.
    pub fn interrupt(&mut self, target: T) {
        match &mut self.animation {
            AnimationType::Spring(spring) => spring.set_target(target),
            AnimationType::Transition(transition) => transition.set_target(target),
        }
    }
}

impl<T> From<T> for Animated<T>
where
    T: Animate + Into<AnimationType<T>>,
{
    fn from(value: T) -> Self {
        Animated::new(value)
    }
}

impl<T> Default for Animated<T>
where
    T: Animate + Default,
{
    fn default() -> Self {
        Animated::new(Transition::new(T::default()))
    }
}
