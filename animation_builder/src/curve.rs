#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Curve {
    #[default]
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

impl Curve {
    /// Gets the value of the curve for the given `progress`.
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
        }
    }
}
