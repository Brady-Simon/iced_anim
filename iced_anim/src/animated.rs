mod animation_type;
mod mode;

use crate::{
    spring::Motion,
    transition::{Easing, Transition},
    Animate, Event, Spring,
};
pub use animation_type::AnimationType;
pub use mode::Mode;
use std::time::{Duration, Instant};

/// The default duration used by animations.
pub const DEFAULT_DURATION: Duration = Duration::from_millis(500);

/// Designed to wrap an [`Animate`] value and enable animating changes to it.
#[derive(Debug, Clone, PartialEq)]
pub struct Animated<T> {
    /// The animation that will be used to animate the value.
    animation: AnimationType<T>,
}

impl<T> Animated<T>
where
    T: Animate,
{
    /// Creates a new [`Animated`] value based on the given [`AnimationConfig`].
    pub fn new(value: T, mode: Mode) -> Self {
        match mode {
            Mode::Spring(motion) => Self::spring(value, motion),
            Mode::Transition(easing) => Self::transition(value, easing),
        }
    }

    /// Creates a new [`Animated`] value using a [`Spring`] animation.
    pub fn spring(value: T, motion: Motion) -> Self {
        Self {
            animation: AnimationType::Spring(Spring::new(value).with_motion(motion)),
        }
    }

    /// Creates a new [`Animated`] value using a [`Transition`] animation.
    pub fn transition(value: T, easing: Easing) -> Self {
        Self {
            animation: AnimationType::Transition(Transition::new(value).with_easing(easing)),
        }
    }

    /// Sets the duration that the animation will last and returns the updated animation.
    pub fn with_duration(mut self, duration: Duration) -> Self {
        match &mut self.animation {
            AnimationType::Spring(spring) => {
                let motion = spring.motion().with_duration(duration);
                spring.set_motion(motion);
            }
            AnimationType::Transition(transition) => {
                transition.set_easing(transition.easing().with_duration(duration));
            }
        }

        self
    }

    /// Updates the animation based on some [`Event`] that occurred.
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

    /// Returns the duration of the animation.
    pub fn duration(&self) -> Duration {
        match &self.animation {
            AnimationType::Spring(spring) => spring.motion().duration(),
            AnimationType::Transition(transition) => transition.duration(),
        }
    }

    /// Applies the given `config` to this animation, updating any duration/motion/curve settings.
    /// Changing modes will reset the animation, interrupting any existing animation curve.
    pub(crate) fn apply(&mut self, mode: Mode) {
        match mode {
            Mode::Spring(motion) => {
                if let AnimationType::Spring(spring) = &mut self.animation {
                    spring.set_motion(motion);
                } else {
                    let value = self.value().clone();
                    let target = self.target().clone();
                    self.animation =
                        AnimationType::Spring(Spring::new(value).to(target).with_motion(motion));
                }
            }
            Mode::Transition(easing) => {
                if let AnimationType::Transition(transition) = &mut self.animation {
                    transition.set_easing(easing);
                } else {
                    let value = self.value().clone();
                    let target = self.target().clone();
                    self.animation = AnimationType::Transition(
                        Transition::new(value).to(target).with_easing(easing),
                    );
                }
            }
        }
    }

    /// Causes the animation to settle immediately at the target value, ending the animation.
    pub fn settle(&mut self) {
        match &mut self.animation {
            AnimationType::Spring(spring) => spring.settle(),
            AnimationType::Transition(transition) => transition.settle(),
        }
    }

    /// Makes the animation immediately settle at the given `value`.
    pub fn settle_at(&mut self, target: T) {
        match &mut self.animation {
            AnimationType::Spring(spring) => spring.settle_at(target),
            AnimationType::Transition(transition) => transition.settle_at(target),
        }
    }

    /// Updates the animation's current value based on the elapsed time since the last update.
    pub fn tick(&mut self, now: Instant) {
        match &mut self.animation {
            AnimationType::Spring(spring) => spring.tick(now),
            AnimationType::Transition(transition) => transition.tick(now),
        }
    }
}

impl<T> From<T> for Animated<T>
where
    T: Animate + Into<AnimationType<T>>,
{
    fn from(value: T) -> Self {
        Self {
            animation: value.into(),
        }
    }
}

impl<T> Default for Animated<T>
where
    T: Animate + Default,
{
    fn default() -> Self {
        Animated::new(T::default(), Mode::default())
    }
}
