use std::time::Duration;

use crate::{Animate, SpringMotion};

/// The default stiffness value for springs.
pub const DEFAULT_STIFFNESS: f32 = 0.5;

/// The default damping fraction for springs.
pub const DEFAULT_DAMPING: f32 = 0.825;

/// The value at which a spring's velocity is considered zero.
///
/// This is used to determine when a spring has reached its target.
pub const EPSILON: f32 = 0.001;

#[derive(Debug, Clone, PartialEq)]
pub struct Spring<T: Animate> {
    value: T,
    target: T,
    motion: SpringMotion,
    velocity: Vec<f32>,
    /// The amount of time remaining before the spring should reach its target.
    remaining: Duration,
}

impl<T> Spring<T>
where
    T: Animate,
{
    /// Creates a new `Spring` with the initial `value`.
    ///
    /// Use the builder methods to customize the spring's behavior and target.
    pub fn new(value: T) -> Self {
        let motion = SpringMotion::default();
        Self {
            value: value.clone(),
            target: value,
            motion,
            velocity: (0..T::components()).into_iter().map(|_| 0.0).collect(),
            remaining: motion.duration(),
        }
    }

    pub fn use_motion(&mut self, motion: SpringMotion) {
        self.motion = motion;
    }

    pub fn with_motion(mut self, motion: SpringMotion) -> Self {
        self.remaining = motion.duration();
        self.motion = motion;
        self
    }

    pub fn with_target(mut self, target: T) -> Self {
        self.target = target;
        self
    }

    pub fn with_velocity(mut self, velocity: Vec<f32>) -> Self {
        self.velocity = velocity;
        self
    }

    /// Returns a reference to this spring's current value.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Returns a reference to this spring's current target.
    pub fn target(&self) -> &T {
        &self.target
    }

    pub fn motion(&self) -> SpringMotion {
        self.motion
    }

    /// Updates the spring's value based on the elapsed time since the last update.
    /// The spring will automatically reach its target when the remaining time reaches zero.
    /// This function will do nothing if the spring has no energy.
    pub fn update(&mut self, dt: Duration) {
        // Don't attempt to update anything if the spring has no energy.
        if !self.has_energy() {
            return;
        }

        // TODO: Consider finding a way to make this more graceful than jumping to the end.
        // Update the remaining time, ending the animation if it has reached zero.
        self.remaining = self.remaining.saturating_sub(dt);
        // if self.remaining.is_zero() {
        //     self.value = self.target.clone();
        //     self.velocity = (0..T::components()).into_iter().map(|_| 0.0).collect();
        // }

        if self.is_near_target() {
            self.value = self.target.clone();
            self.velocity = (0..T::components()).into_iter().map(|_| 0.0).collect();
            return;
        }

        // Remaining duration, so calculate the new velocity and update the values.
        let velocity: Vec<f32> = self
            .target
            .distance_to(&self.value)
            .into_iter()
            .zip(self.velocity.iter().copied())
            .map(|(d, v)| self.new_velocity(d, v, dt.as_secs_f32()))
            .collect();

        self.velocity = velocity.clone();
        let mut components = velocity.iter().map(|v| v * dt.as_secs_f32());
        self.value.update(&mut components);
    }

    /// Gets the new velocity of the spring given the `displacement` and `velocity`.
    fn new_velocity(&self, displacement: f32, velocity: f32, dt: f32) -> f32 {
        let spring: f32 = displacement * self.motion.applied_stiffness();
        let damping = -self.motion.applied_damping() * velocity;
        // println!(
        //     "New velocity with damping {} -> {}, stiffness {} -> {}",
        //     self.damping,
        //     self.applied_damping(self.duration),
        //     self.stiffness,
        //     self.applied_stiffness(self.duration),
        // );

        let acceleration = spring + damping;
        let new_velocity = velocity + acceleration * dt;
        new_velocity
    }

    /// Interrupts the existing animation and starts a new one with the `new_target`,
    /// resetting the remaining time to the original duration.
    pub fn interrupt(&mut self, new_target: T) {
        self.target = new_target;
        self.remaining = self.motion.duration();
    }

    /// A spring has energy if it has not yet reached its target or if it is still moving.
    /// This being `true` means the spring is at rest and doesn't need to be updated.
    pub fn has_energy(&self) -> bool {
        self.value != self.target || self.velocity.iter().any(|&v| v != 0.0)
    }

    /// Returns `true` if the spring is near its target, within a small epsilon value.
    pub fn is_near_target(&self) -> bool {
        self.target
            .distance_to(&self.value)
            .iter()
            .all(|d| d.abs() < EPSILON)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Initial springs should have no energy.
    #[test]
    fn new_springs_have_no_energy() {
        let spring = Spring::new(0.0);
        assert!(!spring.has_energy());
    }

    /// Springs should have energy when the current value is not equal to the target.
    #[test]
    fn has_energy_when_target_is_not_current() {
        let spring = Spring::new(0.0).with_target(5.0);
        assert!(spring.has_energy());
    }

    /// Springs should have energy when the velocity is not zero.
    #[test]
    fn has_energy_when_velocity_is_nonzero() {
        let spring = Spring::new(0.0).with_velocity(vec![1.0]);
        assert!(spring.has_energy());
    }

    /// Updating the spring should change the remaining duration.
    #[test]
    fn update_reduces_remaining_duration() {
        let mut spring = Spring::new(0.0)
            .with_target(1.0)
            .with_motion(SpringMotion::Custom {
                response: Duration::from_secs(1),
                damping: 1.0,
            });
        spring.update(Duration::from_millis(500));
        assert_eq!(spring.remaining, Duration::from_millis(500));
    }

    /// The spring should reach its target when the remaining duration is zero.
    #[ignore = "Waiting until behavior around remaining time is decided"]
    #[test]
    fn update_ends_animation_when_remaining_duration_is_zero() {
        let mut spring = Spring::new(0.0)
            .with_target(1.0)
            .with_motion(SpringMotion::Custom {
                response: Duration::from_secs(1),
                damping: 1.0,
            });
        spring.update(Duration::from_secs(1));
        assert_eq!(spring.remaining, Duration::from_secs(0));
        assert_eq!(spring.value, spring.target);
    }

    /// Interrupting the spring should reset the remaining duration and change the target.
    #[test]
    fn interrupt_resets_duration_and_changes_target() {
        let mut spring = Spring::new(0.0)
            .with_target(1.0)
            .with_motion(SpringMotion::Custom {
                response: Duration::from_secs(1),
                damping: 1.0,
            });

        spring.update(Duration::from_millis(500));
        spring.interrupt(5.0);

        assert_eq!(spring.remaining, Duration::from_secs(1));
        assert_eq!(spring.target, 5.0);
    }
}
