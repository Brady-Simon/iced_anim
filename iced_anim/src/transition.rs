//! A type of animation that transitions between two values by following a curve.
pub mod bezier;
pub mod curve;
mod progress;

use crate::{animated::DEFAULT_DURATION, Animate, Event};
pub use curve::Curve;
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
    /// The curve to use to determine how to update the current value over time.
    curve: Curve,
    /// How long the transition should take to complete.
    duration: Duration,
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
            curve: Curve::default(),
            duration: DEFAULT_DURATION,
            progress: Progress::default(),
            last_update: Instant::now(),
        }
    }

    /// Sets the curve to use for the transition and returns the updated transition.
    pub fn with_curve(mut self, curve: Curve) -> Self {
        self.curve = curve;
        self
    }

    /// Sets the duration of the transition and returns the updated transition.
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Sets the duratoin of the transition.
    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }

    /// Sets the curve of the transition.
    pub fn set_curve(&mut self, curve: Curve) {
        self.curve = curve;
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

    /// Returns the duration of the transition.
    pub fn duration(&self) -> Duration {
        self.duration
    }

    /// Reverses the transition, swapping the initial and target values
    /// and adjusts the animation status to be in the opposite direction.
    pub fn reverse(&mut self) {
        self.progress.reverse();
    }

    /// Ends the transition, immediately setting the current value to the target value.
    pub fn settle(&mut self) {
        self.progress.settle();
        match self.progress {
            Progress::Forward(_) => self.value = self.target.clone(),
            Progress::Reverse(_) => self.value = self.initial.clone(),
        }
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
            .update(delta.as_secs_f32() / self.duration.as_secs_f32());
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
                self.curve.value(self.progress.value()),
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
            .with_curve(Curve::Ease)
            .with_duration(duration);
        assert_eq!(*transition.value(), 0.0);
        assert_eq!(*transition.target(), 1.0);
        assert_eq!(transition.curve, Curve::Ease);
        assert_eq!(transition.duration, duration);
    }

    /// Transitions should be reversible such that changing targets in the middle of an
    /// animation will reverse the animation if the target is the initial value.
    #[test]
    fn reversible_transitions() {
        let mut transition = Transition::new(0.0).to(1.0);
        let halfway = Instant::now() + DEFAULT_DURATION / 2;
        let done = Instant::now() + DEFAULT_DURATION;

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

        let done = Instant::now() + DEFAULT_DURATION;
        transition.tick(done);
        assert!(!transition.is_animating());
    }
}
