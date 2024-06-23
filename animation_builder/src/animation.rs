use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum Animation {
    Stopped,
    Running { start: Instant, duration: Duration },
}

impl Animation {
    pub fn stop(&mut self) {
        *self = Self::Stopped;
    }

    pub fn progress(&self, now: Instant) -> f32 {
        match self {
            Self::Running { start, duration } => {
                let elapsed = now.duration_since(*start);
                elapsed.as_secs_f32() / duration.as_secs_f32()
            }
            Self::Stopped => 1.0,
        }
    }

    pub fn update(&mut self, now: Instant) -> bool {
        let Self::Running { start, duration } = self else {
            return true;
        };

        let elapsed = now.duration_since(*start);
        let new_progress = elapsed.as_secs_f32() / duration.as_secs_f32();

        if new_progress >= 1.0 {
            *self = Self::Stopped;
        } else {
            *self = Self::Running {
                start: *start,
                duration: *duration,
            }
        }

        new_progress >= 1.0
    }
}
