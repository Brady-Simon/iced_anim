//! Motion that defines how a spring animation will behave.
use std::time::Duration;

use crate::animated::DEFAULT_DURATION;

/// The motion associated with a spring animation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Motion {
    /// The fractional amount of drag applied needed to produce critical damping.
    /// A value of 1 will smoothly decelerate the spring to its target, while values
    /// less than 1 will cause the spring to oscillate around the target more before
    /// coming to a stop.
    pub damping: f32,
    /// The stiffness of the spring, defined as an approximate duration.
    /// A value of zero requests an infinitely-stiff spring, suitable for driving
    /// interactive animations.
    pub response: Duration,
}

impl Motion {
    /// A smooth animation without any overshoot of the target.
    pub const SMOOTH: Self = Self {
        damping: 1.0,
        response: DEFAULT_DURATION,
    };

    /// A small overshoot of the target before settling.
    pub const SNAPPY: Self = Self {
        damping: 0.85,
        response: DEFAULT_DURATION,
    };

    /// A bouncier animation where the value overshoots the target before settling.
    pub const BOUNCY: Self = Self {
        damping: 0.7,
        response: DEFAULT_DURATION,
    };

    /// A motion that causes all animations to transition instantly.
    pub const INSTANT: Self = Self {
        damping: 1.0,
        response: Duration::ZERO,
    };

    /// Create a custom spring motion with the given response `duration`.
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.response = duration;
        self
    }

    /// Create a custom spring motion with the given `damping` fraction.
    pub fn with_damping(mut self, damping: f32) -> Self {
        self.damping = damping;
        self
    }

    /// The estimated duration of how long the spring animation.
    /// This is used in the spring physics calculations and does not represent
    /// a strict duration for the animation.
    pub fn duration(&self) -> Duration {
        self.response
    }

    /// The fractional amount of drag applied needed to produce critical damping.
    pub fn damping(&self) -> f32 {
        self.damping
    }

    /// The amount of stiffness applied to the spring, which varies based on the `duration`.
    pub fn applied_stiffness(&self) -> f32 {
        let duration_fraction = self.duration().as_secs_f32();
        39.478_416 / duration_fraction.powi(2)
    }

    /// The amount of damping applied to the spring, which varies based on the `duration`.
    pub fn applied_damping(&self) -> f32 {
        let duration = self.duration().as_secs_f32();
        self.damping() * 12.566_371 / duration
    }
}

impl Default for Motion {
    fn default() -> Self {
        Self::SMOOTH
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The default spring motion should be [`MotionSMOOTH`].
    #[test]
    fn default_is_smooth() {
        assert_eq!(Motion::default(), Motion::SMOOTH);
    }

    #[test]
    fn with_duration() {
        let motion = Motion::SMOOTH.with_duration(Duration::from_millis(300));
        assert_eq!(
            motion,
            Motion {
                response: Duration::from_millis(300),
                damping: Motion::SMOOTH.damping()
            }
        );
    }

    #[test]
    fn with_damping() {
        let motion = Motion::SMOOTH.with_damping(0.5);
        assert_eq!(
            motion,
            Motion {
                response: Motion::SMOOTH.duration(),
                damping: 0.5
            }
        );
    }

    #[test]
    fn test_spring_motion_duration() {
        assert_eq!(Motion::BOUNCY.duration(), DEFAULT_DURATION);
        assert_eq!(Motion::SMOOTH.duration(), DEFAULT_DURATION);
        assert_eq!(Motion::SNAPPY.duration(), DEFAULT_DURATION);
        assert_eq!(
            Motion {
                response: Duration::from_millis(300),
                damping: 0.5
            }
            .duration(),
            Duration::from_millis(300)
        );
    }

    #[test]
    fn test_spring_motion_damping() {
        assert_eq!(Motion::SMOOTH.damping(), 1.0);
        assert_eq!(Motion::SNAPPY.damping(), 0.85);
        assert_eq!(Motion::BOUNCY.damping(), 0.7);
        assert_eq!(
            Motion {
                response: Duration::from_millis(300),
                damping: 0.5
            }
            .damping(),
            0.5
        );
    }

    #[test]
    fn smooth_applied_forces() {
        let motion = Motion {
            response: Duration::from_millis(500),
            damping: 1.0,
        };
        assert_eq!(motion.applied_damping().trunc(), 25.0);
        assert_eq!(motion.applied_stiffness().trunc(), 157.0);
    }

    #[test]
    fn snappy_applied_forces() {
        assert_eq!(Motion::SNAPPY.applied_damping().trunc(), 21.0);
        assert_eq!(Motion::SNAPPY.applied_stiffness().trunc(), 157.0);
    }

    #[test]
    fn bouncy_applied_forces() {
        assert_eq!(Motion::BOUNCY.applied_damping().trunc(), 17.0);
        assert_eq!(Motion::BOUNCY.applied_stiffness().trunc(), 157.0);
    }

    #[test]
    fn custom_applied_forces() {
        let motion = Motion {
            response: Duration::from_millis(250),
            damping: 0.75,
        };
        assert_eq!(motion.applied_damping().trunc(), 37.0);
        assert_eq!(motion.applied_stiffness().trunc(), 631.0);
    }

    /// [MotionINSTANT] should have zero duration and the default damping.
    #[test]
    fn instant() {
        assert_eq!(Motion::INSTANT.duration(), Duration::ZERO);
    }
}
