use super::bezier::{Bezier, EASE, EASE_IN, EASE_IN_OUT, EASE_OUT};

/// A curve that describes how a transition should progress.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Curve {
    /// A linear curve where the value changes at a constant rate.
    #[default]
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    /// A custom bezier curve.
    Bezier(Bezier),
    /// A custom curve that takes a progress value in [0.0, 1.0] and returns the value to use
    /// for the transition when interpolating between two values. The output should generally be
    /// in the range of [0.0, 1.0].
    Custom(fn(f32) -> f32),
}

impl Curve {
    /// The value of the curve at the given `progress`.
    ///
    /// Use this to interpolate between two values. The `progress` should be in the range of
    /// [0.0, 1.0] and represent the amount of time that has passed in the animation, where
    /// 0.0 is the start and 1.0 is the end.
    pub fn value(&self, progress: f32) -> f32 {
        match self {
            Curve::Linear => progress,
            Curve::Ease => EASE.solve(progress),
            Curve::EaseIn => EASE_IN.solve(progress),
            Curve::EaseOut => EASE_OUT.solve(progress),
            Curve::EaseInOut => EASE_IN_OUT.solve(progress),
            Curve::Bezier(bezier) => bezier.solve(progress),
            Curve::Custom(f) => f(progress),
        }
    }
}
