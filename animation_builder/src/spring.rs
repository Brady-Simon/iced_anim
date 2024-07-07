use std::time::Duration;

use crate::Animate;

/// The default stiffness value for springs.
pub const DEFAULT_STIFFNESS: f32 = 0.5;
// pub const DEFAULT_STIFFNESS: f32 = 0.8;

/// The default damping fraction for springs.
pub const DEFAULT_DAMPING: f32 = 0.825;
// pub const DEFAULT_DAMPING: f32 = 0.0;

pub const DEFAULT_DURATION: Duration = Duration::from_millis(500);

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

    pub fn with_velocity(mut self, velocity: Vec<f32>) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn target(&self) -> &T {
        &self.target
    }

    pub fn update(&mut self, dt: f32) {
        let velocity: Vec<f32> = self
            .target
            .distance_to(&self.value)
            .into_iter()
            .zip(self.velocity.iter().copied())
            .map(|(d, v)| self.new_velocity(d, v, dt))
            .collect();

        let components = velocity.iter().map(|v| v * dt).collect();
        self.velocity = velocity;
        self.value.update(components);
    }

    /// Gets the new velocity of the spring given the `displacement` and `velocity`.
    fn new_velocity(&self, displacement: f32, velocity: f32, dt: f32) -> f32 {
        let spring: f32 = displacement * self.applied_stiffness(DEFAULT_DURATION);
        let damping = -self.applied_damping(DEFAULT_DURATION) * velocity;

        let acceleration = spring + damping;
        let new_velocity = velocity + acceleration * dt;
        println!("New velocity: {}", new_velocity);
        new_velocity
    }

    pub fn applied_stiffness(&self, duration: Duration) -> f32 {
        let duration_fraction = duration.as_secs_f32();
        39.47841760435743 / duration_fraction.powi(2)
    }

    pub fn applied_damping(&self, duration: Duration) -> f32 {
        let duration_fraction = duration.as_secs_f32();
        12.56637061435917 / duration_fraction
    }

    pub fn set_target(&mut self, target: T) {
        self.target = target;
    }

    /// A spring has energy if it has not yet reached its target or if it is still moving.
    pub fn has_energy(&self) -> bool {
        self.value != self.target || self.velocity.iter().all(|&v| v == 0.0)
    }
}
