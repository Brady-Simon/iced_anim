use crate::{spring::Motion, transition::Curve};

/// The different animation modes that can be used to animate a value.
/// Different modes have different configuration options.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Spring(Motion),
    Transition(Curve),
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Transition(Curve::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The default mode should be whatever the default transition is.
    #[test]
    fn default_mode() {
        assert_eq!(Mode::default(), Mode::Transition(Curve::default()));
    }
}
