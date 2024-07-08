use std::time::Duration;

use crate::Animate;

/// The default stiffness value for springs.
pub const DEFAULT_STIFFNESS: f32 = 0.5;
// pub const DEFAULT_STIFFNESS: f32 = 0.8;

/// The default damping fraction for springs.
pub const DEFAULT_DAMPING: f32 = 0.825;
// pub const DEFAULT_DAMPING: f32 = 0.0;

pub const DEFAULT_DURATION: Duration = Duration::from_millis(500);

#[derive(Debug, Clone, PartialEq)]
pub struct Spring<T: Animate> {
    value: T,
    target: T,
    /// The stiffness of the spring, defined as an approximate duration in seconds.
    /// A value of zero requests an infinitely-stiff spring, suitable for driving
    /// interactive animations.
    stiffness: f32,
    /// The amount of drag applied to the value being animated, as a fraction
    /// of an estimate of amount needed to produce critical damping.
    ///
    /// A damping of 1 will smoothly descelerate the spring to its target.
    /// Damping ratios less than 1 will cause the spring to oscillate around
    /// the target more before coming to a stop.
    damping: f32,
    velocity: Vec<f32>,
    duration: Duration,
    /// The amount of time remaining before the spring should reach its target.
    remaining: Duration,
}

impl<T> Spring<T>
where
    T: Animate,
{
    pub fn new(value: T) -> Self {
        Self {
            value: value.clone(),
            target: value,
            stiffness: DEFAULT_STIFFNESS,
            damping: DEFAULT_DAMPING,
            velocity: (0..T::components()).into_iter().map(|_| 0.0).collect(),
            duration: DEFAULT_DURATION,
            remaining: DEFAULT_DURATION,
        }
    }

    pub fn with_stiffness(mut self, stiffness: f32) -> Self {
        self.stiffness = stiffness;
        self
    }

    pub fn with_damping(mut self, damping: f32) -> Self {
        self.damping = damping;
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

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self.remaining = duration;
        self
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn target(&self) -> &T {
        &self.target
    }

    pub fn update(&mut self, dt: Duration) {
        // Don't attempt to update anything if the spring has no energy.
        if !self.has_energy() {
            return;
        }

        // TODO: Consider finding a way to make this more graceful than jumping to the end.
        // Update the remaining time, ending the animation if it has reached zero.
        self.remaining = self.remaining.saturating_sub(dt);
        if self.remaining.is_zero() {
            self.value = self.target.clone();
            self.velocity = (0..T::components()).into_iter().map(|_| 0.0).collect();
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
        let spring: f32 = displacement * self.applied_stiffness(self.duration);
        let damping = -self.applied_damping(self.duration) * velocity;

        let acceleration = spring + damping;
        let new_velocity = velocity + acceleration * dt;
        new_velocity
    }

    /// The amount of stiffness applied to the spring, which varies based on the `duration`.
    pub fn applied_stiffness(&self, duration: Duration) -> f32 {
        let duration_fraction = duration.as_secs_f32();
        39.47841760435743 / duration_fraction.powi(2)
    }

    /// The amount of damping applied to the spring, which varies based on the `duration`.
    pub fn applied_damping(&self, duration: Duration) -> f32 {
        let duration_fraction = duration.as_secs_f32();
        12.56637061435917 / duration_fraction
    }

    /// Interrupts the existing animation and starts a new one with the `new_target`,
    /// resetting the remaining time to the original duration.
    pub fn interrupt(&mut self, new_target: T) {
        self.target = new_target;
        self.remaining = self.duration;
    }

    /// A spring has energy if it has not yet reached its target or if it is still moving.
    pub fn has_energy(&self) -> bool {
        self.value != self.target || self.velocity.iter().any(|&v| v != 0.0)
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
            .with_duration(Duration::from_secs(1));
        spring.update(Duration::from_millis(500));
        assert_eq!(spring.remaining, Duration::from_millis(500));
    }

    /// The spring should reach its target when the remaining duration is zero.
    #[test]
    fn update_ends_animation_when_remaining_duration_is_zero() {
        let mut spring = Spring::new(0.0)
            .with_target(1.0)
            .with_duration(Duration::from_secs(1));
        spring.update(Duration::from_secs(1));
        assert_eq!(spring.remaining, Duration::from_secs(0));
        assert_eq!(spring.value, spring.target);
    }

    /// Interrupting the spring should reset the remaining duration and change the target.
    #[test]
    fn interrupt_resets_duration_and_changes_target() {
        let mut spring = Spring::new(0.0)
            .with_target(1.0)
            .with_duration(Duration::from_secs(1));

        spring.update(Duration::from_millis(500));
        spring.interrupt(5.0);

        assert_eq!(spring.remaining, Duration::from_secs(1));
        assert_eq!(spring.target, 5.0);
    }
}
