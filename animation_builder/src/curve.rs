#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Curve {
    #[default]
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}
