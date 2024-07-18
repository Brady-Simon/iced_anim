//! Core definitions for the `Animate` trait and its implementations.
//!
//! The `Animate` trait is the main building block for animations. It is primarily designed for
//! Spring/velocity-based animations to make animations interruptible without any jarring effects.
//!
//! You can implement this trait for custom types using the "derive" feature.
use std::sync::Arc;

use iced::theme::palette;

/// A trait for types that can be animated on a per-property basis.
///
/// You can derive this trait with `#[derive(Animate)]` with the `derive` feature enabled.
/// Otherwise, you can manually implement it while ensuring that `Animate::components` returns
/// the same length vector as `Animate::distance_to` returns.
///
/// Also, ensure that `Animate::update` and `Animate::distance_to` are consistent with each other
/// in both the number of components consumed and the order of the components. Keeping these in
/// sync is important to ensure that updates affect the correct properties.
pub trait Animate: Clone + PartialEq {
    /// The number if animatable components in the type.
    ///
    /// Simple types like `f32` have 1 component, while more complex types like `Color` have 4.
    /// This is used so the animation knows how many properties may be animated.
    fn components() -> usize;

    /// Update the type with the next set of components.
    ///
    /// The `components` is an iterator of new fractional values that should be added to the
    /// current value.
    fn update(&mut self, components: &mut impl Iterator<Item = f32>);

    /// The distance between the current value and the end value.
    ///
    /// The `end` value is the target value that the current value should be animated towards.
    /// This distance can be positive or negative and returns a vector that should be consistent
    /// with `Animate::components` and the update order in `Animate::update`.
    fn distance_to(&self, end: &Self) -> Vec<f32>;
}

impl Animate for f32 {
    fn components() -> usize {
        1
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        *self += components.next().unwrap();
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        vec![self - end]
    }
}

impl Animate for iced::Point<f32> {
    fn components() -> usize {
        2
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.x += components.next().unwrap();
        self.y += components.next().unwrap();
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [self.x.distance_to(&end.x), self.y.distance_to(&end.y)].concat()
    }
}

impl Animate for iced::Color {
    fn components() -> usize {
        4
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.r = (self.r + components.next().unwrap()).clamp(0.0, 1.0);
        self.g = (self.g + components.next().unwrap()).clamp(0.0, 1.0);
        self.b = (self.b + components.next().unwrap()).clamp(0.0, 1.0);
        self.a = (self.a + components.next().unwrap()).clamp(0.0, 1.0);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.r.distance_to(&end.r),
            self.g.distance_to(&end.g),
            self.b.distance_to(&end.b),
            self.a.distance_to(&end.a),
        ]
        .concat()
    }
}

impl Animate for iced::theme::Palette {
    fn components() -> usize {
        5 * iced::Color::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.background.update(components);
        self.text.update(components);
        self.primary.update(components);
        self.success.update(components);
        self.danger.update(components);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.background.distance_to(&end.background),
            self.text.distance_to(&end.text),
            self.primary.distance_to(&end.primary),
            self.success.distance_to(&end.success),
            self.danger.distance_to(&end.danger),
        ]
        .concat()
    }
}

impl Animate for iced::Theme {
    fn components() -> usize {
        iced::theme::Palette::components() + iced::theme::palette::Extended::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        let mut palette = self.palette();
        palette.update(components);

        let mut extended = *self.extended_palette();
        extended.update(components);

        *self = iced::Theme::Custom(Arc::new(iced::theme::Custom::with_fn(
            "Animating Theme".to_owned(),
            palette,
            move |_| extended,
        )))
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.palette().distance_to(&end.palette()),
            self.extended_palette().distance_to(end.extended_palette()),
        ]
        .concat()
    }
}

impl Animate for palette::Pair {
    fn components() -> usize {
        2 * iced::Color::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.color.update(components);
        self.text.update(components);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.color.distance_to(&end.color),
            self.text.distance_to(&end.text),
        ]
        .concat()
    }
}

impl Animate for palette::Primary {
    fn components() -> usize {
        3 * palette::Pair::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.strong.update(components);
        self.base.update(components);
        self.weak.update(components);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.strong.distance_to(&end.strong),
            self.base.distance_to(&end.base),
            self.weak.distance_to(&end.weak),
        ]
        .concat()
    }
}

impl Animate for palette::Secondary {
    fn components() -> usize {
        3 * palette::Pair::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.strong.update(components);
        self.base.update(components);
        self.weak.update(components);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.strong.distance_to(&end.strong),
            self.base.distance_to(&end.base),
            self.weak.distance_to(&end.weak),
        ]
        .concat()
    }
}

impl Animate for palette::Success {
    fn components() -> usize {
        3 * palette::Pair::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.strong.update(components);
        self.base.update(components);
        self.weak.update(components);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.strong.distance_to(&end.strong),
            self.base.distance_to(&end.base),
            self.weak.distance_to(&end.weak),
        ]
        .concat()
    }
}

impl Animate for palette::Danger {
    fn components() -> usize {
        3 * palette::Pair::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.strong.update(components);
        self.base.update(components);
        self.weak.update(components);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.strong.distance_to(&end.strong),
            self.base.distance_to(&end.base),
            self.weak.distance_to(&end.weak),
        ]
        .concat()
    }
}

impl Animate for palette::Background {
    fn components() -> usize {
        3 * palette::Pair::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.strong.update(components);
        self.base.update(components);
        self.weak.update(components);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.strong.distance_to(&end.strong),
            self.base.distance_to(&end.base),
            self.weak.distance_to(&end.weak),
        ]
        .concat()
    }
}

impl Animate for palette::Extended {
    fn components() -> usize {
        palette::Background::components()
            + palette::Primary::components()
            + palette::Secondary::components()
            + palette::Success::components()
            + palette::Danger::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.primary.update(components);
        self.secondary.update(components);
        self.success.update(components);
        self.danger.update(components);
        self.background.update(components);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.primary.distance_to(&end.primary),
            self.secondary.distance_to(&end.secondary),
            self.success.distance_to(&end.success),
            self.danger.distance_to(&end.danger),
            self.background.distance_to(&end.background),
        ]
        .concat()
    }
}

impl<T1, T2> Animate for (T1, T2)
where
    T1: Animate,
    T2: Animate,
{
    fn components() -> usize {
        T1::components() + T2::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.0.update(components);
        self.1.update(components);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [self.0.distance_to(&end.0), self.1.distance_to(&end.1)].concat()
    }
}

impl<T1, T2, T3> Animate for (T1, T2, T3)
where
    T1: Animate,
    T2: Animate,
    T3: Animate,
{
    fn components() -> usize {
        T1::components() + T2::components() + T3::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.0.update(components);
        self.1.update(components);
        self.2.update(components);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.0.distance_to(&end.0),
            self.1.distance_to(&end.1),
            self.2.distance_to(&end.2),
        ]
        .concat()
    }
}

impl<T1, T2, T3, T4> Animate for (T1, T2, T3, T4)
where
    T1: Animate,
    T2: Animate,
    T3: Animate,
    T4: Animate,
{
    fn components() -> usize {
        T1::components() + T2::components() + T3::components() + T4::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.0.update(components);
        self.1.update(components);
        self.2.update(components);
        self.3.update(components);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.0.distance_to(&end.0),
            self.1.distance_to(&end.1),
            self.2.distance_to(&end.2),
            self.3.distance_to(&end.3),
        ]
        .concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f32_components() {
        assert_eq!(f32::components(), 1);
    }

    #[test]
    fn f32_point_components() {
        assert_eq!(iced::Point::<f32>::components(), 2);
    }

    #[test]
    fn f32_color_components() {
        assert_eq!(iced::Color::components(), 4);
    }

    #[test]
    fn color_pair_components() {
        assert_eq!(
            iced::theme::palette::Pair::components(),
            2 * iced::Color::components()
        );
    }

    #[test]
    fn primary_components() {
        assert_eq!(
            iced::theme::palette::Primary::components(),
            3 * iced::theme::palette::Pair::components()
        );
    }

    #[test]
    fn secondary_components() {
        assert_eq!(
            iced::theme::palette::Secondary::components(),
            3 * iced::theme::palette::Pair::components()
        );
    }

    #[test]
    fn success_components() {
        assert_eq!(
            iced::theme::palette::Success::components(),
            3 * iced::theme::palette::Pair::components()
        );
    }

    #[test]
    fn danger_components() {
        assert_eq!(
            iced::theme::palette::Danger::components(),
            3 * iced::theme::palette::Pair::components()
        );
    }

    #[test]
    fn background_components() {
        assert_eq!(
            iced::theme::palette::Background::components(),
            3 * iced::theme::palette::Pair::components()
        );
    }

    #[test]
    fn extended_palette_components() {
        assert_eq!(
            iced::theme::palette::Extended::components(),
            iced::theme::palette::Background::components()
                + iced::theme::palette::Primary::components()
                + iced::theme::palette::Secondary::components()
                + iced::theme::palette::Success::components()
                + iced::theme::palette::Danger::components()
        );
    }

    #[test]
    fn theme_components() {
        assert_eq!(
            iced::Theme::components(),
            iced::theme::Palette::components() + iced::theme::palette::Extended::components()
        );
    }
}
