use std::time::Duration;

use crate::{Animate, SpringMotion};

/// The minimum percent at which a spring is considered near its target.
///
/// This value is used to determine when a spring is near its target and has low velocity.
/// It needs to strike a balance between stopping the animation too early and continuing
/// needlessly when the target is effectively reached.
pub const ESPILON: f32 = 0.005;

/// A representation of a spring animation that interpolates between values.
/// You typically won't need to use this directly, but it's used by the `AnimationBuilder`.
#[derive(Debug, Clone, PartialEq)]
pub struct Spring<T: Animate> {
    /// The current value of the spring.
    value: T,
    /// The target value that the spring will animate towards.
    target: T,
    /// The type of motion that the spring will follow, which controls damping/stiffness.
    motion: SpringMotion,
    /// The total amount of time this spring has been animating to the current target.
    /// This is mostly useful for debugging.
    total: Duration,
    /// The current velocity components that make up this spring animation.
    velocity: Vec<f32>,
    /// The initial distance from the target when the animation was started or interrupted.
    /// This is used to help determine when the spring is near its target and is precomputed
    /// to avoid recalculating it every frame.
    initial_distance: Vec<f32>,
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
            total: Duration::ZERO,
            velocity: (0..T::components()).into_iter().map(|_| 0.0).collect(),
            initial_distance: (0..T::components()).into_iter().map(|_| 0.0).collect(),
        }
    }

    /// Updates the spring's `motion` to the given value.
    pub fn set_motion(&mut self, motion: SpringMotion) {
        self.motion = motion;
    }

    /// Returns an updated spring with the given `motion`.
    pub fn with_motion(mut self, motion: SpringMotion) -> Self {
        self.motion = motion;
        self
    }

    /// Returns an updated spring with the given `target`.
    pub fn with_target(mut self, target: T) -> Self {
        self.initial_distance = self.value.distance_to(&target);
        self.target = target;
        self
    }

    /// Returns an updated spring with the given `velocity`.
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

    /// Returns the spring's current `SpringMotion`.
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
        self.total += dt;

        // End the animation if the spring is near the target wiht low velocity.
        if self.is_near_end() {
            self.value = self.target.clone();
            self.velocity = (0..T::components()).into_iter().map(|_| 0.0).collect();
            return;
        }

        // Still animating, so calculate the new velocity and update the values.
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

        let acceleration = spring + damping;
        let new_velocity = velocity + acceleration * dt;
        new_velocity
    }

    /// Interrupts the existing animation and starts a new one with the `new_target`.
    pub fn interrupt(&mut self, new_target: T) {
        self.target = new_target;
        self.initial_distance = self.value.distance_to(&self.target);
        self.total = Duration::ZERO;
    }

    /// A spring has energy if it has not yet reached its target or if it is still moving.
    /// This being `true` means the spring is at rest and doesn't need to be updated.
    pub fn has_energy(&self) -> bool {
        self.value != self.target || self.velocity.iter().any(|&v| v != 0.0)
    }

    /// Whether the spring is near the end of its animation.
    ///
    /// The animation will be stopped when the spring is near the target and has low velocity
    /// to avoid needlessly animating imperceptible changes.
    fn is_near_end(&self) -> bool {
        self.value
            .distance_to(&self.target)
            .iter()
            .zip(&self.initial_distance)
            .zip(&self.velocity)
            .all(|((d, i), v)| match i {
                0.0 => true,
                _ => {
                    let d_percent = (d / i).abs();
                    let v_percent = (v / i).abs();
                    d_percent <= ESPILON && v_percent <= ESPILON
                }
            })
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

    /// Interrupting the spring should change the target.
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

        assert_eq!(spring.target, 5.0);
    }
}
