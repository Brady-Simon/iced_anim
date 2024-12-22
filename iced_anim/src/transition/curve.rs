use super::bezier::{EASE, EASE_IN, EASE_IN_OUT, EASE_OUT};

/// A curve that describes how a transition should progress.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Curve {
    #[default]
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInOutCirc,
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
            Curve::EaseInOutCirc => {
                if progress < 0.5 {
                    (1.0 - (1.0 - (2.0 * progress).powf(2.0)).sqrt()) / 2.0
                } else {
                    (1.0 + (1.0 - (-2.0 * progress + 2.0).powf(2.0)).sqrt()) / 2.0
                }
            }
        }
    }
}
