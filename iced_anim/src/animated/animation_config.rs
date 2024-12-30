use crate::{spring::Motion, transition::Curve};

use super::{Mode, DEFAULT_DURATION};
use std::time::Duration;

/// A configuration that can be used to create an animated value.
///
/// This is primarily intended to make it easy to customize animations from within widgets
/// without needing to know the exact generic types used by the animation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnimationConfig {
    /// The mode of the animation indicating whether to use a spring or a transition.
    mode: Mode,
    // TODO: This `duration` can probably be embedded within a new `transition::Motion`. It's redundant for springs.
    /// The duration of the animation.
    duration: Duration,
}

impl AnimationConfig {
    /// Returns the mode of the animation config.
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Returns the duration of the animation config.
    pub fn duration(&self) -> Duration {
        self.duration
    }

    /// Creates a new spring animation config.
    pub fn spring(motion: Motion) -> Self {
        Self {
            mode: Mode::Spring(motion),
            duration: motion.duration(),
        }
    }

    /// Creates a new transition animation config.
    pub fn transition(curve: Curve) -> Self {
        Self {
            mode: Mode::Transition(curve),
            duration: DEFAULT_DURATION,
        }
    }

    /// Changes the configuration's duration and returns the updated configuration.
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        if let Mode::Spring(motion) = self.mode {
            self.mode = Mode::Spring(motion.with_duration(duration));
        }

        self
    }
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            mode: Mode::default(),
            duration: DEFAULT_DURATION,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The default configuration should be whatever the default mode and duration are.
    #[test]
    fn default_config() {
        assert_eq!(
            AnimationConfig::default(),
            AnimationConfig {
                mode: Mode::default(),
                duration: DEFAULT_DURATION
            }
        );
    }
}
