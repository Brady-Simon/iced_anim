use std::sync::Arc;

use iced::theme::palette;

pub trait Animate: Clone + PartialEq {
    fn components() -> usize;
    fn update(&mut self, components: &mut impl Iterator<Item = f32>);
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
        vec![self.x.distance_to(&end.x), self.y.distance_to(&end.y)].concat()
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
        vec![
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
        vec![
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
        let mut palette = self.palette().clone();
        palette.update(components);

        let mut extended = self.extended_palette().clone();
        extended.update(components);

        *self = iced::Theme::Custom(Arc::new(iced::theme::Custom::with_fn(
            "Animating Theme".to_owned(),
            palette,
            move |_| extended,
        )))
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        vec![
            self.palette().distance_to(&end.palette()),
            self.extended_palette().distance_to(&end.extended_palette()),
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
        vec![
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
        vec![
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
        vec![
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
        vec![
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
        vec![
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
        vec![
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
        vec![
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
        vec![self.0.distance_to(&end.0), self.1.distance_to(&end.1)].concat()
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
