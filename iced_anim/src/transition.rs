pub mod bezier;
pub mod curve;

use crate::{Animate, Event};
pub use curve::Curve;
use std::time::{Duration, Instant};

/// The default duration for animations used for [`Default`] implementations.
pub(crate) const DEFAULT_DURATION: Duration = Duration::from_millis(500);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Forward(f32),
    Reverse(f32),
}

impl Default for Status {
    fn default() -> Self {
        Self::Forward(1.0)
    }
}

impl Status {
    /// Gets the the progress of the animation, in the range of [0.0, 1.0].
    /// A value of 0.0 indicates the start of the animation, while 1.0 indicates the end.
    pub fn progress(&self) -> f32 {
        match self {
            Self::Forward(progress) => *progress,
            Self::Reverse(progress) => 1.0 - progress,
        }
    }

    /// Reverses this status, e.g. Idle -> Idle, Forward -> Reverse, Reverse -> Forward.
    /// Reversing will swap the progress value, e.g. Forward(0.2) -> Reverse(0.8).
    pub fn reverse(&mut self) {
        *self = self.reversed();
    }

    /// Returns a reversed copy of this status.
    pub fn reversed(&self) -> Self {
        match self {
            Self::Forward(progress) => Self::Reverse(1.0 - progress),
            Self::Reverse(progress) => Self::Forward(1.0 - progress),
        }
    }

    pub fn is_complete(&self) -> bool {
        match self {
            Self::Forward(progress) | Self::Reverse(progress) => *progress >= 1.0,
        }
    }

    pub fn update(&mut self, delta_progress: f32) {
        let new_status = match self {
            Self::Forward(ref progress) => {
                let progress = (progress + delta_progress).min(1.0);
                Self::Forward(progress)
            }
            Self::Reverse(ref progress) => {
                let progress = (progress + delta_progress).min(1.0);
                Self::Reverse(progress)
            }
        };
        *self = new_status;
    }

    /// Settles this transition, setting the state to Idle.
    pub fn settle(&mut self) {
        *self = match *self {
            Self::Forward(_) => Self::Forward(1.0),
            Self::Reverse(_) => Self::Reverse(1.0),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Transition<T> {
    initial: T,
    value: T,
    target: T,
    curve: Curve,
    duration: Duration,
    status: Status,
    last_update: Instant,
}

impl<T> Transition<T>
where
    T: Animate,
{
    pub fn new(value: T) -> Self {
        Self {
            initial: value.clone(),
            target: value.clone(),
            value,
            curve: Curve::default(),
            duration: DEFAULT_DURATION,
            status: Status::default(),
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

    /// Returns a reference to the current `value` of the transition.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Returns a reference to the current `target` of the transition.
    /// This is the final value that the transition is moving towards.
    pub fn target(&self) -> &T {
        &self.target
    }

    /// Updates the transition with details of the given `event`.
    pub fn update(&mut self, event: Event<T>) {
        match event {
            Event::Settle => {
                self.status.settle();
                self.value = self.target.clone();
            }
            Event::Tick(now) => {
                let delta = now.duration_since(self.last_update);
                self.last_update = now;

                // TODO: Make status only have forward/backward and then add `is_complete` method
                self.status
                    .update(delta.as_secs_f32() / self.duration.as_secs_f32());
                if self.status.is_complete() {
                    match self.status {
                        Status::Forward(_) => {
                            self.value = self.target.clone();
                        }
                        Status::Reverse(_) => {
                            self.value = self.initial.clone();
                        }
                    }
                } else {
                    self.value.lerp(
                        &self.initial,
                        &self.target,
                        self.curve.value(self.status.progress()),
                    );
                }
            }
            Event::Target(target) => {
                // Reverse the transition if the new target is the initial
                // value and the animation isn't done. This ensures that the
                // animation follows the same curve when reversing.
                if target == self.initial && !self.status.is_complete() {
                    self.status.reverse();
                } else if target != self.target {
                    self.status = Status::Forward(0.0);
                    self.initial = self.value.clone();
                    self.target = target;
                }

                self.last_update = Instant::now();
            }
        }
    }

    /// Whether this transition is currently animating towards its target.
    pub fn is_animating(&self) -> bool {
        let matches_destination = match self.status {
            Status::Forward(_) => self.value != self.target,
            Status::Reverse(_) => self.value != self.initial,
        };

        matches_destination && !self.status.is_complete()
    }
}
