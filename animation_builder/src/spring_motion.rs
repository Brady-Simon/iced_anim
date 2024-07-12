use std::{fmt::Display, time::Duration};

/// The motion associated with a spring animation.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SpringMotion {
    /// A smooth animation without any overshoot of the target.
    #[default]
    Smooth,
    /// A small overshoot of the target before settling.
    Snappy,
    /// A bouncier animation where the value overshoots the target before settling.
    Bouncy,
    /// A custom spring animation with the given `response` and `damping`.
    Custom {
        /// The stiffness of the spring, defined as an approximate duration in seconds.
        /// A value of zero requests an infinitely-stiff spring, suitable for driving
        /// interactive animations.
        response: Duration,
        /// The fractional  amount of drag applied needed to produce critical damping.
        /// A value of 1 will smoothly decelerate the spring to its target, while values
        /// less than 1 will cause the spring to oscillate around the target more before
        /// coming to a stop.
        damping: f32,
    },
}

impl SpringMotion {
    pub fn duration(&self) -> Duration {
        match self {
            Self::Bouncy | Self::Smooth | Self::Snappy => Duration::from_millis(500),
            Self::Custom { response, .. } => *response,
        }
    }

    pub fn damping(&self) -> f32 {
        match self {
            Self::Bouncy => 0.7,
            Self::Smooth => 1.0,
            Self::Snappy => 0.85,
            Self::Custom { damping, .. } => *damping,
        }
    }

    pub fn applied_stiffness(&self) -> f32 {
        let duration_fraction = self.duration().as_secs_f32();
        39.47841760435743 / duration_fraction.powi(2)
    }

    /// The amount of damping applied to the spring, which varies based on the `duration`.
    pub fn applied_damping(&self) -> f32 {
        let duration = self.duration().as_secs_f32();
        self.damping() * 12.56637061435917 / duration
    }
}

impl Display for SpringMotion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Smooth => write!(f, "Smooth"),
            Self::Snappy => write!(f, "Snappy"),
            Self::Bouncy => write!(f, "Bouncy"),
            Self::Custom { .. } => write!(f, "Custom"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The default spring motion should be `SpringMotion::Smooth`.
    #[test]
    fn default_is_smooth() {
        assert_eq!(SpringMotion::default(), SpringMotion::Smooth);
    }

    #[test]
    fn test_spring_motion_duration() {
        assert_eq!(SpringMotion::Bouncy.duration(), Duration::from_millis(500));
        assert_eq!(SpringMotion::Smooth.duration(), Duration::from_millis(500));
        assert_eq!(SpringMotion::Snappy.duration(), Duration::from_millis(500));
        assert_eq!(
            SpringMotion::Custom {
                response: Duration::from_millis(300),
                damping: 0.5
            }
            .duration(),
            Duration::from_millis(300)
        );
    }

    #[test]
    fn test_spring_motion_damping() {
        assert_eq!(SpringMotion::Smooth.damping(), 1.0);
        assert_eq!(SpringMotion::Snappy.damping(), 0.85);
        assert_eq!(SpringMotion::Bouncy.damping(), 0.7);
        assert_eq!(
            SpringMotion::Custom {
                response: Duration::from_millis(300),
                damping: 0.5
            }
            .damping(),
            0.5
        );
    }

    #[test]
    fn smooth_applied_forces() {
        let motion = SpringMotion::Custom {
            response: Duration::from_millis(500),
            damping: 1.0,
        };
        assert_eq!(motion.applied_damping().trunc(), 25.0);
        assert_eq!(motion.applied_stiffness().trunc(), 157.0);
    }

    #[test]
    fn snappy_applied_forces() {
        let motion = SpringMotion::Snappy;
        assert_eq!(motion.applied_damping().trunc(), 21.0);
        assert_eq!(motion.applied_stiffness().trunc(), 157.0);
    }

    #[test]
    fn bouncy_applied_forces() {
        let motion = SpringMotion::Bouncy;
        assert_eq!(motion.applied_damping().trunc(), 17.0);
        assert_eq!(motion.applied_stiffness().trunc(), 157.0);
    }

    #[test]
    fn custom_applied_forces() {
        let motion = SpringMotion::Custom {
            response: Duration::from_millis(250),
            damping: 0.75,
        };
        assert_eq!(motion.applied_damping().trunc(), 37.0);
        assert_eq!(motion.applied_stiffness().trunc(), 631.0);
    }
}
