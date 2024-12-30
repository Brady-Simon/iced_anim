/// A way to track the progress of a `Transition`.
///
/// A [`Progress`] can be either [`Progress::Forward`] or [`Progress::Reverse`], indicating
/// the direction of the transition. This extra state information lets us reverse transitions
/// smoothly and not cause the transition to jump from one part of the curve to another.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Progress {
    Forward(f32),
    Reverse(f32),
}

impl Default for Progress {
    fn default() -> Self {
        Self::Forward(1.0)
    }
}

impl Progress {
    /// Gets the value on the curve for the current progress, in the range of [0.0, 1.0].
    /// A value of 0.0 indicates the start of the animation, while 1.0 indicates the end.
    ///
    /// [`Progress::Reverse`] will be inverted from [`Progress::Forward`], e.g. 0.2 -> 0.8.
    /// This is what allows transitions to reverse smoothly.
    pub fn value(&self) -> f32 {
        match self {
            Self::Forward(progress) => *progress,
            Self::Reverse(progress) => 1.0 - *progress,
        }
    }

    /// Gets the current progress of the transition, where `0.0` is the beginning of the
    /// transition and `1.0` is the end. This is distinct from [`Progress::value`], which gives
    /// the value to check on the curve.
    pub fn progress(&self) -> f32 {
        match self {
            Self::Forward(progress) => *progress,
            Self::Reverse(progress) => *progress,
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

    /// Whether the transition is complete, i.e. the progress is >= 1.0.
    pub fn is_complete(&self) -> bool {
        self.progress() >= 1.0
    }

    /// Updates the progress by the given delta, clamping the result to the range [0.0, 1.0].
    pub fn update(&mut self, delta_progress: f32) {
        let progress = (self.progress() + delta_progress).clamp(0.0, 1.0);
        match self {
            Self::Forward(p) | Self::Reverse(p) => *p = progress,
        }
    }

    /// Settles this transition, setting the state to Idle.
    pub fn settle(&mut self) {
        match self {
            Self::Forward(progress) => *progress = 1.0,
            Self::Reverse(progress) => *progress = 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Reversing a [`Progress`] should swap the forward/reverse state and value,
    /// e.g. Forward(0.2) -> Reverse(0.8).
    #[test]
    fn reversed() {
        let progress = Progress::Forward(0.2);
        let reversed = progress.reversed();
        assert_eq!(reversed, Progress::Reverse(0.8));
    }

    /// The [`Progress::value`] method should return the progress value.
    #[test]
    fn forward_value() {
        let progress = Progress::Forward(0.25);
        assert_eq!(progress.value(), 0.25);
    }

    #[test]
    fn reverse_value() {
        let progress = Progress::Reverse(0.25);
        assert_eq!(progress.value(), 0.75);
    }

    /// [`Progress::settle`] should set the progress to 1.0.
    #[test]
    fn settle() {
        let mut progress = Progress::Forward(0.5);
        progress.settle();
        assert_eq!(progress, Progress::Forward(1.0));
    }

    #[test]
    fn settle_reverse() {
        let mut progress = Progress::Reverse(0.5);
        progress.settle();
        assert_eq!(progress, Progress::Reverse(1.0));
    }

    /// [`Progress::is_complete`] should return `true` if the progress is `1.0`.
    #[test]
    fn is_complete() {
        let progress = Progress::Forward(0.5);
        assert!(!progress.is_complete());

        let progress = Progress::Forward(1.0);
        assert!(progress.is_complete());

        let progress = Progress::Reverse(1.0);
        assert!(progress.is_complete());
    }

    /// [`Progress::update`] should update the inner progress by the given delta,
    /// clamping the result to the range [0.0, 1.0].
    #[test]
    fn update() {
        let mut progress = Progress::Forward(0.0);
        progress.update(0.5);
        assert_eq!(progress.value(), 0.5);

        progress.update(0.5);
        assert_eq!(progress.value(), 1.0);

        progress.update(0.5);
        assert_eq!(progress.value(), 1.0);
    }

    /// The default [`Progress`] should be [`Progress::Forward`] with a value of 1.0
    /// to ensure new transitions are idle.
    #[test]
    fn default() {
        assert_eq!(Progress::default(), Progress::Forward(1.0));
    }
}
