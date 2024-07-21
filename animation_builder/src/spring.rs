//! Spring physics to enable natural and interactive animations.
use std::{fmt::Debug, time::Instant};

use crate::{spring_event::SpringEvent, Animate, SpringMotion};

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
    /// The last instant at which this spring's value was updated.
    last_update: Instant,
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
            last_update: Instant::now(),
            velocity: (0..T::components()).map(|_| 0.0).collect(),
            initial_distance: (0..T::components()).map(|_| 0.0).collect(),
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
        self.interrupt(target);
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

    /// Returns the instant at which the spring was last updated.
    pub fn last_update(&self) -> Instant {
        self.last_update
    }

    /// Updates the spring based on the given `event`.
    ///
    /// You can update either the current value by passing `SpringEvent::Tick`
    /// or change the target value by passing `SpringEvent::Target`.
    ///
    /// ```rust
    /// # use iced_anim::{Spring, SpringEvent};
    /// let mut spring = Spring::new(0.0);
    /// spring.update(SpringEvent::Target(5.0));
    /// assert_eq!(spring.target(), &5.0);
    ///
    /// spring.update(SpringEvent::Tick(std::time::Instant::now()));
    /// assert!(*spring.value() > 0.0);
    /// ```
    pub fn update(&mut self, event: SpringEvent<T>) {
        match event {
            SpringEvent::Tick(now) => self.tick(now),
            SpringEvent::Target(target) => self.interrupt(target),
        }
    }

    /// Updates the spring's value based on the elapsed time since the last update.
    /// The spring will automatically reach its target when the remaining time reaches zero.
    /// This function will do nothing if the spring has no energy.
    pub fn tick(&mut self, now: Instant) {
        // Don't attempt to update anything if the spring has no energy.
        if !self.has_energy() {
            return;
        }

        let dt = now.duration_since(self.last_update);
        self.last_update = now;

        // End the animation if the spring is near the target wiht low velocity.
        if self.is_near_end() {
            self.value = self.target.clone();
            self.velocity = (0..T::components()).map(|_| 0.0).collect();
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

        self.velocity.clone_from(&velocity);
        let mut components = velocity.iter().map(|v| v * dt.as_secs_f32());
        self.value.update(&mut components);
    }

    /// Gets the new velocity of the spring given the `displacement` and `velocity`.
    fn new_velocity(&self, displacement: f32, velocity: f32, dt: f32) -> f32 {
        let spring: f32 = displacement * self.motion.applied_stiffness();
        let damping = -self.motion.applied_damping() * velocity;

        let acceleration = spring + damping;

        velocity + acceleration * dt
    }

    /// Interrupts the existing animation and starts a new one with the `new_target`.
    pub fn interrupt(&mut self, new_target: T) {
        self.target = new_target;
        self.initial_distance = self.value.distance_to(&self.target);
        self.last_update = Instant::now();
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

impl<T> Default for Spring<T>
where
    T: Animate + Default,
{
    fn default() -> Self {
        Self::new(T::default())
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

    #[test]
    fn tick_changes_value_and_last_update_time() {
        let mut spring = Spring::new(0.0).with_target(1.0);
        let now = Instant::now();
        spring.tick(now);

        // Updating should move the spring's value closer to the target
        // and update the last update time to the given instant.
        assert!(spring.last_update() == now);
        assert!(*spring.value() > 0.0);
    }

    #[test]
    fn interrupt_changes_target_and_resets_last_update_time() {
        let mut spring = Spring::new(0.0).with_target(1.0);

        let now = Instant::now();
        spring.tick(now);
        spring.interrupt(5.0);

        // The target should change as well as adjust the last update time.
        assert_eq!(spring.target, 5.0);
        assert!(spring.last_update() > now);
    }

    /// Springs should implement `Default` if `T` does.
    #[test]
    fn default_impl() {
        let spring = Spring::<f32>::default();
        assert_eq!(spring.value(), &f32::default());
    }
}
