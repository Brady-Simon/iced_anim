//! A cubic bezier curve implementation that follows the pattern used by browsers.
//! The primary use-case is enabling curves like `cubic-bezier()` from CSS.
//!
//! The implementation follows this pattern:
//! - Pre-compute coefficients and splices for the cubic bezier curve in [`Bezier::new`].
//! - Use linear interpolation using the pre-computed splices to get an initial guess.
//! - Use Newton's method to refine the solution obtained from linear interpolation.
//! - Fall back to bisection if Newton's method isn't enough to solve the curve with the desired precision.
//!
//! For more information, see:
//! - [Recreating CSS3 Cubic-Bezier Curves](https://stackoverflow.com/a/11697909)
//! - [WebKit implementation](https://github.com/WebKit/WebKit/blob/main/Source/WebCore/platform/graphics/UnitBezier.h)
//! - [Firefox implementation](https://github.com/mozilla/gecko-dev/blob/master/dom/smil/SMILKeySpline.cpp)
use std::sync::LazyLock;

/// Precision used for solving the curve.
/// - See [Newton's method](https://en.wikipedia.org/wiki/Newton%27s_method)
const BEZIER_EPSILON: f32 = 1e-7;

/// The maximum number of iterations used to solve the curve using Newton's method,
/// which is faster than bisection.
const MAX_NEWTON_ITERATIONS: usize = 4;

/// The number of samples used to pre-compute points along the curve.
const CUBIC_BEZIER_SPLINE_SAMPLES: usize = 11;

/// An easing function that starts slow, accelerates sharply, and then slows down gradually.
pub const EASE: LazyLock<Bezier> = LazyLock::new(|| Bezier::new(0.25, 0.1, 0.25, 1.0));

/// An easing function that starts slow and speeds up.
pub const EASE_IN: LazyLock<Bezier> = LazyLock::new(|| Bezier::new(0.42, 0.0, 1.0, 1.0));

/// An easing function that starts fast and slows down.
pub const EASE_OUT: LazyLock<Bezier> = LazyLock::new(|| Bezier::new(0.0, 0.0, 0.58, 1.0));

/// An easing function that starts slow, speeds up, and then slows down.
pub const EASE_IN_OUT: LazyLock<Bezier> = LazyLock::new(|| Bezier::new(0.0, 0.42, 0.58, 1.0));

/// A bezier curve implementation designed to solve cubic bezier curves.
/// The primary use-case is enabling curves like `cubic-bezier()` from CSS.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bezier {
    ax: f32,
    bx: f32,
    cx: f32,
    ay: f32,
    by: f32,
    cy: f32,
    start_gradient: f32,
    end_gradient: f32,
    spline_samples: [f32; CUBIC_BEZIER_SPLINE_SAMPLES],
}

impl Bezier {
    /// Creates a new [`Bezier`] curve with the given control points,
    /// where `(x1, y1)` and `(x2, y2)` are the control points.
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        let cx = 3.0 * x1;
        let bx = 3.0 * (x2 - x1) - cx;
        let ax = 1.0 - cx - bx;

        let cy = 3.0 * y1;
        let by = 3.0 * (y2 - y1) - cy;
        let ay = 1.0 - cy - by;

        let start_gradient = if x1 > 0.0 {
            y1 / x1
        } else if y1 == 0.0 && x2 > 0.0 {
            y2 / x2
        } else if y1 == 0.0 && y2 == 0.0 {
            1.0
        } else {
            0.0
        };

        let end_gradient = if x2 < 1.0 {
            (y2 - 1.0) / (x2 - 1.0)
        } else if y2 == 1.0 && x1 < 1.0 {
            (y1 - 1.0) / (x1 - 1.0)
        } else if y2 == 1.0 && y1 == 1.0 {
            1.0
        } else {
            0.0
        };

        let mut spline_samples = [0.0; CUBIC_BEZIER_SPLINE_SAMPLES];
        let delta_t = 1.0 / (CUBIC_BEZIER_SPLINE_SAMPLES - 1) as f32;
        for i in 0..CUBIC_BEZIER_SPLINE_SAMPLES {
            spline_samples[i] = Self::sample_curve_x(ax, bx, cx, i as f32 * delta_t);
        }

        Bezier {
            ax,
            bx,
            cx,
            ay,
            by,
            cy,
            start_gradient,
            end_gradient,
            spline_samples,
        }
    }

    fn sample_curve_x(ax: f32, bx: f32, cx: f32, t: f32) -> f32 {
        ((ax * t + bx) * t + cx) * t
    }

    fn sample_curve_y(&self, t: f32) -> f32 {
        ((self.ay * t + self.by) * t + self.cy) * t
    }

    fn sample_curve_derivative_x(&self, t: f32) -> f32 {
        (3.0 * self.ax * t + 2.0 * self.bx) * t + self.cx
    }

    fn solve_curve_x(&self, x: f32, epsilon: f32) -> f32 {
        let mut t0 = 0.0;
        let mut t1 = 0.0;
        let mut t2 = x;
        let mut x2 = 0.0;

        // Linear interpolation to provide an initial guess of the solution.
        let delta_t = 1.0 / (CUBIC_BEZIER_SPLINE_SAMPLES - 1) as f32;
        for i in 1..CUBIC_BEZIER_SPLINE_SAMPLES {
            if x <= self.spline_samples[i] {
                t1 = delta_t * i as f32;
                t0 = t1 - delta_t;
                t2 = t0
                    + (t1 - t0) * (x - self.spline_samples[i - 1])
                        / (self.spline_samples[i] - self.spline_samples[i - 1]);
                break;
            }
        }

        // Perform a few iterations of Newton's method to refine the solution.
        let newton_epsilon = BEZIER_EPSILON.min(epsilon);
        for _ in 0..MAX_NEWTON_ITERATIONS {
            x2 = Self::sample_curve_x(self.ax, self.bx, self.cx, t2) - x;
            if x2.abs() < newton_epsilon {
                return t2;
            }
            let d2 = self.sample_curve_derivative_x(t2);
            if d2.abs() < BEZIER_EPSILON {
                break;
            }
            t2 -= x2 / d2;
        }

        if x2.abs() < epsilon {
            return t2;
        }

        // Fall back to bisection for the remaining iterations if Newton's method isn't enough.
        while t0 < t1 {
            x2 = Self::sample_curve_x(self.ax, self.bx, self.cx, t2);
            if (x2 - x).abs() < epsilon {
                return t2;
            }
            if x > x2 {
                t0 = t2;
            } else {
                t1 = t2;
            }
            t2 = (t1 + t0) * 0.5;
        }

        t2
    }

    /// Solves for `y` on the curve given `x`.
    ///
    /// For animations, `x` will usually be your progress in time through the animation.
    pub fn solve(&self, x: f32) -> f32 {
        self.solve_with_precision(x, BEZIER_EPSILON)
    }

    /// Solves for `y` on the curve given `x` using a custom `epsilon`.
    ///
    /// For animations, `x` will usually be your progress in time through the animation.
    pub fn solve_with_precision(&self, x: f32, epsilon: f32) -> f32 {
        match x {
            x if x < 0.0 => self.start_gradient * x,
            x if x > 1.0 => 1.0 + self.end_gradient * (x - 1.0),
            _ => self.sample_curve_y(self.solve_curve_x(x, epsilon)),
        }
    }
}
