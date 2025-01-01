use crate::animated::DEFAULT_DURATION;

use super::Curve;
use std::time::Duration;

/// A configuration for creating a `Transition`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Easing {
    /// The curve to use to determine how to update the current value over time.
    pub curve: Curve,
    /// How long the transition should take to complete.
    pub duration: Duration,
    /// Whether the transition will move in reverse if a new target value is the
    /// same as the initial value.
    pub reversible: bool,
}

impl Default for Easing {
    fn default() -> Self {
        Self {
            curve: Curve::default(),
            duration: DEFAULT_DURATION,
            reversible: false,
        }
    }
}

impl Easing {
    /// Creates a new [`Easing`] with the given `curve`.
    pub fn new(curve: Curve) -> Self {
        Self {
            curve,
            duration: DEFAULT_DURATION,
            reversible: false,
        }
    }

    /// Sets the `curve` of the easing and returns the updated easing.
    pub fn with_curve(mut self, curve: Curve) -> Self {
        self.curve = curve;
        self
    }

    /// Sets the `duration` of the easing and returns the updated easing.
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Sets whether the easing is reversible and returns the updated easing.
    pub fn reversible(mut self, reversible: bool) -> Self {
        self.reversible = reversible;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let easing = Easing::default();

        assert_eq!(easing.curve, Curve::default());
        assert_eq!(easing.duration, DEFAULT_DURATION);
        assert!(!easing.reversible);
    }

    #[test]
    fn builders() {
        let easing = Easing::new(Curve::EaseInOut)
            .with_duration(Duration::from_millis(300))
            .reversible(true);

        assert_eq!(easing.curve, Curve::EaseInOut);
        assert_eq!(easing.duration, Duration::from_millis(300));
        assert!(easing.reversible);
    }
}
