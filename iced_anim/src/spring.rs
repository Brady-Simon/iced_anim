//! Spring physics to enable natural and interactive animations.
use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use crate::{spring_event::SpringEvent, Animate, SpringMotion};

/// The minimum percent at which a spring is considered near its target.
///
/// This value is used to determine when a spring is near its target and has low velocity.
/// It needs to strike a balance between stopping the animation too early and continuing
/// needlessly when the target is effectively reached.
pub const ESPILON: f32 = 0.005;

/// The maximum duration between spring updates that is allowed before clamping the time
/// to avoid large jumps in the spring's value.
///
/// This is particularly noticeable when the window loses focus and the app stops receiving
/// redraws, causing the next update to have a much larger duration than the last one.
pub const MAX_DURATION: Duration = Duration::from_millis(33);

/// A representation of a spring animation that interpolates between values.
///
/// You can use this alongside the `Animation` widget to animate changes to your UI
/// by storing the spring in your state and updating it with events.
///
/// As this is designed for GUI animations and not general-purpose physics simulations,
/// it includes some features targeted toward avoiding UI issues like overshooting.
/// See [`MAX_DURATION`] and [`ESPILON`] for examples of this.
#[derive(Debug, Clone, PartialEq)]
pub struct Spring<T> {
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

// Impls that don't require an `Animate` bound.
impl<T> Spring<T> {
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

    /// Returns the spring's current [`SpringMotion`].
    pub fn motion(&self) -> SpringMotion {
        self.motion
    }

    /// Returns the instant at which the spring was last updated.
    pub fn last_update(&self) -> Instant {
        self.last_update
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
}

impl<T> Spring<T>
where
    T: Animate,
{
    /// Creates a new [`Spring`] with the initial `value`.
    ///
    /// Use the builder methods to customize the spring's behavior and target.
    pub fn new(value: T) -> Self {
        let motion = SpringMotion::default();
        Self {
            value: value.clone(),
            target: value,
            motion,
            last_update: Instant::now(),
            velocity: vec![0.0; T::components()],
            initial_distance: vec![0.0; T::components()],
        }
    }

    /// Returns an updated spring with the given `target`.
    pub fn with_target(mut self, target: T) -> Self {
        self.interrupt(target);
        self
    }

    /// A spring has energy if it has not yet reached its target or if it is still moving.
    /// This being `true` means the spring is at rest and doesn't need to be updated.
    pub fn has_energy(&self) -> bool {
        self.value != self.target || self.velocity.iter().any(|&v| v != 0.0)
    }

    /// Updates the spring based on the given `event`.
    ///
    /// You can update either the current value by passing [`SpringEvent::Tick`]
    /// or change the target value by passing [`SpringEvent::Target`].
    ///
    /// ```rust
    /// # use iced_anim::{Spring, SpringEvent};
    /// let mut spring = Spring::new(0.0);
    /// spring.update(SpringEvent::Target(5.0));
    /// assert_eq!(spring.target(), &5.0);
    ///
    /// spring.update(SpringEvent::Tick(std::time::Instant::now()));
    /// assert!(*spring.value() > 0.0);
    ///
    /// spring.update(SpringEvent::Settle);
    /// assert_eq!(spring.value(), spring.target());
    /// ```
    pub fn update(&mut self, event: SpringEvent<T>) {
        match event {
            SpringEvent::Tick(now) => self.tick(now),
            SpringEvent::Target(target) => self.interrupt(target),
            SpringEvent::Settle => self.settle(),
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

        let dt = now.duration_since(self.last_update).min(MAX_DURATION);
        self.last_update = now;

        // End the animation if the spring is near the target wiht low velocity.
        if self.is_near_end() {
            self.settle();
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
        // Reset the last update if the spring doesn't have any energy.
        // This avoids resetting the last update during continuously interrupted animations.
        if !self.has_energy() {
            self.last_update = Instant::now();
        }

        self.target = new_target;
        self.initial_distance = self.value.distance_to(&self.target);
    }

    /// Causes the spring to settle immediately at the target value,
    /// ending any ongoing animation and setting the velocity to zero.
    pub fn settle(&mut self) {
        self.value = self.target.clone();
        self.velocity = vec![0.0; T::components()];
    }

    /// Makes the spring value and target immediately settle at the given `value`.
    pub fn settle_at(&mut self, value: T) {
        self.value = value.clone();
        self.target = value;
        self.velocity = vec![0.0; T::components()];
    }

    /// Whether the spring is near the end of its animation.
    ///
    /// The animation will be stopped when the spring is near the target and has low velocity
    /// to avoid needlessly animating imperceptible changes.
    fn is_near_end(&self) -> bool {
        self.motion.duration().is_zero()
            || self
                .value
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
    use std::time::Duration;

    use super::*;

    /// The maximum duration between spring updates should be 33ms, or 1 frame at 30fps.
    #[test]
    fn max_duration_time() {
        assert_eq!(MAX_DURATION, Duration::from_millis(33));
    }

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

    /// [`Spring::settle_at`] should set the spring's value and target to the given value
    /// and bring all velocity components to zero.
    #[test]
    fn settle_at() {
        let mut spring = Spring::new(0.0).with_target(3.0);
        spring.settle_at(5.0);
        assert_eq!(spring.value(), &5.0);
        assert_eq!(spring.target(), &5.0);
        assert_eq!(spring.velocity, vec![0.0]);
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
    }

    /// An spring at rest should have its last update reset when interrupted.
    #[test]
    fn interrupt_resets_last_update_when_at_rest() {
        let start_time = Instant::now();
        let mut spring = Spring::new(0.0);
        spring.interrupt(5.0);

        // The last update time should be reset if the spring has no energy.
        assert!(spring.last_update > start_time);
    }

    /// A spring with energy shouldn't have its last update reset when interrupted.
    /// This is to avoid resetting the last update during continuously interrupted animations,
    /// which can cause the Diff -> Event loop to have 0ms duration between updates when the real
    /// duration between renders is much longer.
    #[test]
    fn interrupt_does_not_reset_last_update_with_energy() {
        let mut spring = Spring::new(0.0).with_target(10.0).with_velocity(vec![1.0]);
        let update_time = Instant::now();
        spring.update(SpringEvent::Tick(update_time));
        spring.interrupt(5.0);

        // The last update time should not be reset if the spring has energy.
        assert_eq!(spring.last_update, update_time);
    }

    /// Calling `.settle()` should jump the spring's value to the target value.
    #[test]
    fn settle_sets_value_to_target() {
        let mut spring = Spring::new(0.0).with_target(5.0);
        spring.settle();
        assert_eq!(spring.value(), spring.target());
    }

    /// Calling `.settle()` should bring all velocity components to zero.
    #[test]
    fn settle_resets_velocity() {
        let mut spring = Spring::new(0.0).with_target(5.0).with_velocity(vec![1.0]);
        spring.settle();
        assert_eq!(spring.velocity, vec![0.0]);
    }

    /// Springs should implement [`Default`] if `T` does.
    #[test]
    fn default_impl() {
        let spring = Spring::<f32>::default();
        assert_eq!(spring.value(), &f32::default());
    }

    /// A response of zero should imply the spring is near its target.
    #[test]
    fn is_near_end_with_zero_duration() {
        let spring = Spring::new(0.0)
            .with_target(1.0)
            .with_motion(SpringMotion::Custom {
                response: Duration::ZERO,
                damping: 0.5,
            });
        assert!(spring.is_near_end());
    }

    /// A spring with a response of zero should settle immediately.
    #[test]
    fn update_zero_response() {
        let mut spring = Spring::new(0.0).with_target(1.0);
        spring.set_motion(SpringMotion::Custom {
            response: Duration::ZERO,
            damping: 0.5,
        });
        spring.update(SpringEvent::Tick(Instant::now()));
        assert_eq!(spring.value(), spring.target());
    }
}
