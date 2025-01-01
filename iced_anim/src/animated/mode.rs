use crate::{spring::Motion, transition::Easing};

/// The different animation modes that can be used to animate a value.
/// Different modes have different configuration options.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Spring(Motion),
    Transition(Easing),
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Transition(Easing::default())
    }
}

impl From<Motion> for Mode {
    fn from(motion: Motion) -> Self {
        Mode::Spring(motion)
    }
}

impl From<Easing> for Mode {
    fn from(easing: Easing) -> Self {
        Mode::Transition(easing)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The default mode should be whatever the default transition is.
    #[test]
    fn default_mode() {
        assert_eq!(Mode::default(), Mode::Transition(Easing::default()));
    }

    #[test]
    fn from_motion() {
        let motion = Motion::default();
        let mode: Mode = motion.into();
        assert_eq!(mode, Mode::Spring(motion));
    }

    #[test]
    fn from_easing() {
        let easing = Easing::default();
        let mode: Mode = easing.into();
        assert_eq!(mode, Mode::Transition(easing));
    }
}
