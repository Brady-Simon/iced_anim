use std::f32::consts::PI;

/// An animation curve that can be applied to an animation.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Curve {
    /// A simple linear curve.
    Linear,
    /// A curve that starts slow and speeds up. Useful for animating off-screen.
    EaseIn,
    /// A curve that starts fast and slows down. Useful for animating on-screen.
    EaseOut,
    /// A curve that starts slow, speeds up in the middle, and slows down at the end.
    /// Useful for animating across the screen.
    #[default]
    EaseInOut,
    /// A curve that follows the sine function.
    Sine,
}

impl Curve {
    /// Gets the value of the curve for the given `progress`, where the
    /// `progress` is in the range `[0, 1]`.
    ///
    /// The return value is normally in the range `[0, 1]`, but it can be
    /// outside of this range depending on the curve (e.g. `Spring`).
    pub fn value(&self, progress: f32) -> f32 {
        match self {
            Curve::Linear => progress,
            Curve::EaseIn => progress.powi(2),
            Curve::EaseOut => progress.sqrt(),
            Curve::EaseInOut => {
                let progress = progress * 2.0;
                if progress < 1.0 {
                    0.5 * progress.powi(2)
                } else {
                    -0.5 * ((progress - 1.0) * (progress - 3.0) - 1.0)
                }
            }
            Curve::Sine => (progress * PI / 2.0).sin(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// The default curve should be `Curve::EaseInOut`
    #[test]
    fn default_curve() {
        assert_eq!(Curve::default(), Curve::EaseInOut);
    }

    #[test]
    fn curve_linear() {
        assert_eq!(Curve::Linear.value(0.0), 0.0);
        assert_eq!(Curve::Linear.value(0.25), 0.25);
        assert_eq!(Curve::Linear.value(0.5), 0.5);
        assert_eq!(Curve::Linear.value(0.75), 0.75);
        assert_eq!(Curve::Linear.value(1.0), 1.0);
    }
}
