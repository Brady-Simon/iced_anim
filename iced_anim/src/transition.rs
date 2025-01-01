//! A type of animation that transitions between two values by following a curve.
pub mod bezier;
pub mod curve;
mod easing;
mod progress;

use crate::{Animate, Event};
pub use curve::Curve;
pub use easing::Easing;
pub use progress::Progress;
use std::time::{Duration, Instant};

/// A type of animation that transitions between two values.
#[derive(Debug, Clone, PartialEq)]
pub struct Transition<T> {
    /// The initial value of the transition - the starting point.
    initial: T,
    /// The current value at this point in the transition.
    value: T,
    /// The target value of the transition - the end point.
    target: T,
    /// The easing properties used to determine how to update a value over time.
    easing: Easing,
    /// How far along the transition is.
    progress: Progress,
    /// The time at which the transition was last updated.
    last_update: Instant,
}

impl<T> Transition<T>
where
    T: Animate,
{
    /// Creates a new transition with the given `value`.
    pub fn new(value: T) -> Self {
        Self {
            initial: value.clone(),
            target: value.clone(),
            value,
            easing: Easing::default(),
            progress: Progress::default(),
            last_update: Instant::now(),
        }
    }

    /// Sets the easing to use for the transition and returns the updated transition.
    pub fn with_easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    /// Sets the easing of the transition.
    pub fn set_easing(&mut self, easing: Easing) {
        self.easing = easing;
    }

    /// Returns a reference to the current `value` of the transition.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Returns a reference to the current `target` of the transition.
    /// This is the final value that the transition is moving towards.
    ///
    /// If the transition is currently reversing, this will be the `initial` value.
    pub fn target(&self) -> &T {
        match self.progress {
            Progress::Forward(_) => &self.target,
            Progress::Reverse(_) => &self.initial,
        }
    }

    /// Returns the transition's current [`Easing`] configuration.
    pub fn easing(&self) -> Easing {
        self.easing
    }

    /// Returns the duration of the transition.
    pub fn duration(&self) -> Duration {
        self.easing.duration
    }

    /// Reverses the transition, swapping the initial and target values
    /// and adjusts the animation status to be in the opposite direction.
    ///
    /// If the transition is not reversible, this will use the initial value as the new target
    /// and reset the progress to start from the beginning.
    pub fn reverse(&mut self) {
        if self.easing.reversible {
            self.progress.reverse();
        } else {
            // If the transition isn't reversible, change the target value to the initial value
            // and reset the progress to start from the beginning.
            self.target = self.initial.clone();
            self.initial = self.value.clone();
            self.progress = Progress::Forward(0.0);
        }
    }

    /// Ends the transition, immediately setting the current value to the target value.
    pub fn settle(&mut self) {
        self.progress.settle();
        match self.progress {
            Progress::Forward(_) => self.value = self.target.clone(),
            Progress::Reverse(_) => self.value = self.initial.clone(),
        }
    }

    /// Makes the transition immediately settle at the given `target`.
    pub fn settle_at(&mut self, target: T) {
        self.value = target.clone();
        self.target = target;
        self.progress = Progress::Forward(1.0);
    }

    /// Updates the transition with details of the given `event`.
    pub fn update(&mut self, event: Event<T>) {
        match event {
            Event::Settle => self.settle(),
            Event::Tick(now) => self.tick(now),
            Event::Target(target) => self.set_target(target),
        }
    }

    /// Sets the `target` value of the transition, and returns the updated transition.
    pub fn to(mut self, target: T) -> Self {
        self.set_target(target);
        self
    }

    /// Interrupts the existing transition and starts a new one with the new `target`.
    pub fn set_target(&mut self, target: T) {
        // Don't do anything if the target hasn't changed.
        if self.target() == &target {
            return;
        }

        // Reset the last update if the transition isn't moving.
        // This avoids resetting the last update during continuously interrupted animations.
        if !self.is_animating() {
            self.last_update = Instant::now();
        }

        // Reverse the transition if the new target is the initial
        // value and the animation isn't done. This ensures that the
        // animation follows the same curve when reversing.
        let is_initial_target = match self.progress {
            Progress::Forward(_) => target == self.initial,
            Progress::Reverse(_) => target == self.target,
        };

        if is_initial_target && !self.progress.is_complete() {
            self.reverse();
        } else if &target != self.target() {
            // Target has changed, reset the progress and update the initial value.
            self.progress = Progress::Forward(0.0);
            self.initial = self.value.clone();
            self.target = target;
        }

        self.last_update = Instant::now();
    }

    /// Updates the transition's value based on the elapsed time since the last update.
    pub fn tick(&mut self, now: Instant) {
        if !self.is_animating() {
            return;
        }

        // Figure out how much time has passed since the last update
        let delta = now.duration_since(self.last_update);
        self.last_update = now;

        self.progress
            .update(delta.as_secs_f32() / self.easing.duration.as_secs_f32());
        if self.progress.is_complete() {
            // We're at the target - assign the current value to the target value.
            // This ensures that the value is exactly the target value, even if the
            // curve doesn't reach it or the animation implementation isn't correct.
            self.value = self.target().clone();
        } else {
            // Continue to lerp the value towards the target
            self.value.lerp(
                &self.initial,
                &self.target,
                self.easing.curve.value(self.progress.value()),
            );
        }
    }

    /// Whether this transition is currently animating towards its target.
    pub fn is_animating(&self) -> bool {
        !self.progress.is_complete()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animated::DEFAULT_DURATION;

    /// Various builder functions should behave as expected.
    #[test]
    fn initialization() {
        let duration = Duration::from_millis(100);
        let transition = Transition::new(0.0)
            .to(1.0)
            .with_easing(Easing::EASE.with_duration(duration));
        assert_eq!(*transition.value(), 0.0);
        assert_eq!(*transition.target(), 1.0);
        assert_eq!(transition.easing, Easing::EASE.with_duration(duration));
    }

    /// Transitions that are not reversible should always use the `Progress::Forward` variant
    /// and update the target value to be the new target value.
    #[test]
    fn non_reversible_transitions() {
        let mut transition = Transition::new(0.0)
            .to(1.0)
            .with_easing(Easing::default().reversible(false));
        let halfway = Instant::now() + DEFAULT_DURATION / 2;
        let new_target = 2.0;

        // Forward progress should maintain the `forward` direction.
        transition.tick(halfway);
        assert!(matches!(transition.progress, Progress::Forward(_)));

        // Setting a new target should not reverse the transition.
        transition.set_target(new_target);
        assert!(matches!(transition.progress, Progress::Forward(_)));
        assert_eq!(*transition.target(), new_target);

        // The final result should have the new target value.
        let done = Instant::now() + DEFAULT_DURATION + Duration::from_millis(1);
        transition.tick(done);
        assert_eq!(*transition.value(), new_target);
    }

    /// Transitions should be reversible such that changing targets in the middle of an
    /// animation will reverse the animation if the target is the initial value.
    #[test]
    fn reversible_transitions() {
        let mut transition = Transition::new(0.0)
            .to(1.0)
            .with_easing(Easing::default().reversible(true));
        let halfway = Instant::now() + DEFAULT_DURATION / 2;
        let done = Instant::now() + DEFAULT_DURATION + Duration::from_millis(1);

        // Forward progress should maintain the `forward` direction.
        transition.tick(halfway);
        assert!(matches!(transition.progress, Progress::Forward(_)));

        // Reversing the transition should change the direction.
        transition.reverse();
        assert!(matches!(transition.progress, Progress::Reverse(_)));

        // The final result should have the initial value.
        transition.tick(done);
        assert_eq!(*transition.value(), 0.0);

        // Reversing the transition again should change the direction back to forward.
        transition.reverse();
        assert!(matches!(transition.progress, Progress::Forward(_)));
    }

    /// [`Transition::is_animating`] should return `true` when the transition is still in progress.
    #[test]
    fn is_animating() {
        let mut transition = Transition::new(0.0).to(1.0);
        assert!(transition.is_animating());

        let halfway = Instant::now() + DEFAULT_DURATION / 2;
        transition.tick(halfway);
        assert!(transition.is_animating());

        let done = Instant::now() + DEFAULT_DURATION + Duration::from_millis(1);
        transition.tick(done);
        assert!(!transition.is_animating());
    }

    /// [`Transition::settle_at`] should jump the transition to the given value.
    #[test]
    fn settle_at() {
        let mut transition = Transition::new(0.0).to(1.0);
        transition.settle_at(0.5);
        assert_eq!(*transition.value(), 0.5);
        assert_eq!(*transition.target(), 0.5);
        assert!(!transition.is_animating());
        assert_eq!(transition.progress, Progress::Forward(1.0));
    }
}
