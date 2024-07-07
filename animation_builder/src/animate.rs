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

// impl Animate for iced::theme::Palette {
//     fn animate_to(&self, end: &Self, progress: f32, curve: Curve) -> Self {
//         let background = self.background.animate_to(&end.background, progress, curve);
//         let text = self.text.animate_to(&end.text, progress, curve);
//         let primary = self.primary.animate_to(&end.primary, progress, curve);
//         let success = self.success.animate_to(&end.success, progress, curve);
//         let danger = self.danger.animate_to(&end.danger, progress, curve);
//         Self {
//             background,
//             text,
//             primary,
//             success,
//             danger,
//         }
//     }

//     fn distance_to(&self, end: &Self) -> f32 {
//         euclidean_distance!(
//             self.background.distance_to(&end.background),
//             self.text.distance_to(&end.text),
//             self.primary.distance_to(&end.primary),
//             self.success.distance_to(&end.success),
//             self.danger.distance_to(&end.danger),
//         )
//     }
// }

// impl Animate for iced::Theme {
//     fn animate_to(&self, end: &Self, progress: f32, curve: Curve) -> Self {
//         match progress {
//             ..=0.0 => self.clone(),
//             1.0.. => end.clone(),
//             _ => iced::Theme::custom(
//                 if progress < 0.5 {
//                     self.to_string()
//                 } else {
//                     end.to_string()
//                 },
//                 self.palette().animate_to(&end.palette(), progress, curve),
//             ),
//         }
//     }

//     fn distance_to(&self, end: &Self) -> f32 {
//         self.palette().distance_to(&end.palette())
//     }
// }

// impl Animate for iced::theme::palette::Pair {
//     fn animate_to(&self, end: &Self, progress: f32, curve: Curve) -> Self {
//         let color = self.color.animate_to(&end.color, progress, curve);
//         let text = self.text.animate_to(&end.text, progress, curve);
//         Self { color, text }
//     }

//     fn distance_to(&self, end: &Self) -> f32 {
//         euclidean_distance!(
//             self.color.distance_to(&end.color),
//             self.text.distance_to(&end.text)
//         )
//     }
// }

// impl Animate for iced::theme::palette::Primary {
//     fn animate_to(&self, end: &Self, progress: f32, curve: Curve) -> Self {
//         let strong = self.strong.animate_to(&end.strong, progress, curve);
//         let base = self.base.animate_to(&end.base, progress, curve);
//         let weak = self.weak.animate_to(&end.weak, progress, curve);
//         Self { strong, base, weak }
//     }

//     fn distance_to(&self, end: &Self) -> f32 {
//         euclidean_distance!(
//             self.strong.distance_to(&end.strong),
//             self.base.distance_to(&end.base),
//             self.weak.distance_to(&end.weak),
//         )
//     }
// }

// impl Animate for iced::theme::palette::Secondary {
//     fn animate_to(&self, end: &Self, progress: f32, curve: Curve) -> Self {
//         let strong = self.strong.animate_to(&end.strong, progress, curve);
//         let base = self.base.animate_to(&end.base, progress, curve);
//         let weak = self.weak.animate_to(&end.weak, progress, curve);
//         Self { strong, base, weak }
//     }

//     fn distance_to(&self, end: &Self) -> f32 {
//         euclidean_distance!(
//             self.strong.distance_to(&end.strong),
//             self.base.distance_to(&end.base),
//             self.weak.distance_to(&end.weak),
//         )
//     }
// }

// impl Animate for iced::theme::palette::Success {
//     fn animate_to(&self, end: &Self, progress: f32, curve: Curve) -> Self {
//         let strong = self.strong.animate_to(&end.strong, progress, curve);
//         let base = self.base.animate_to(&end.base, progress, curve);
//         let weak = self.weak.animate_to(&end.weak, progress, curve);
//         Self { strong, base, weak }
//     }

//     fn distance_to(&self, end: &Self) -> f32 {
//         euclidean_distance!(
//             self.strong.distance_to(&end.strong),
//             self.base.distance_to(&end.base),
//             self.weak.distance_to(&end.weak),
//         )
//     }
// }

// impl Animate for iced::theme::palette::Danger {
//     fn animate_to(&self, end: &Self, progress: f32, curve: Curve) -> Self {
//         let strong = self.strong.animate_to(&end.strong, progress, curve);
//         let base = self.base.animate_to(&end.base, progress, curve);
//         let weak = self.weak.animate_to(&end.weak, progress, curve);
//         Self { strong, base, weak }
//     }

//     fn distance_to(&self, end: &Self) -> f32 {
//         euclidean_distance!(
//             self.strong.distance_to(&end.strong),
//             self.base.distance_to(&end.base),
//             self.weak.distance_to(&end.weak),
//         )
//     }
// }

// impl Animate for iced::theme::palette::Background {
//     fn animate_to(&self, end: &Self, progress: f32, curve: Curve) -> Self {
//         let strong = self.strong.animate_to(&end.strong, progress, curve);
//         let base = self.base.animate_to(&end.base, progress, curve);
//         let weak = self.weak.animate_to(&end.weak, progress, curve);
//         Self { strong, base, weak }
//     }

//     fn distance_to(&self, end: &Self) -> f32 {
//         euclidean_distance!(
//             self.strong.distance_to(&end.strong),
//             self.base.distance_to(&end.base),
//             self.weak.distance_to(&end.weak),
//         )
//     }
// }

// impl Animate for iced::theme::palette::Extended {
//     fn animate_to(&self, end: &Self, progress: f32, curve: Curve) -> Self {
//         let is_dark = if progress < 0.5 {
//             self.is_dark
//         } else {
//             end.is_dark
//         };

//         let primary = self.primary.animate_to(&end.primary, progress, curve);
//         let secondary = self.secondary.animate_to(&end.secondary, progress, curve);
//         let success = self.success.animate_to(&end.success, progress, curve);
//         let danger = self.danger.animate_to(&end.danger, progress, curve);
//         let background = self.background.animate_to(&end.background, progress, curve);

//         Self {
//             is_dark,
//             primary,
//             secondary,
//             success,
//             danger,
//             background,
//         }
//     }

//     fn distance_to(&self, end: &Self) -> f32 {
//         euclidean_distance!(
//             self.primary.distance_to(&end.primary),
//             self.secondary.distance_to(&end.secondary),
//             self.success.distance_to(&end.success),
//             self.danger.distance_to(&end.danger),
//             self.background.distance_to(&end.background),
//         )
//     }
// }

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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     /// f32 values should animate correctly.
//     #[test]
//     fn animate_f32() {
//         let start = 0.0;
//         let end = 10.0;
//         let animated = start.animate_to(&end, 0.5, Curve::Linear);

//         assert_eq!(animated, 5.0);
//     }

//     /// Animations can go in any direction.
//     #[test]
//     fn animate_f32_backwards() {
//         let start = 10.0;
//         let end = 0.0;
//         let animated = start.animate_to(&end, 0.5, Curve::Linear);

//         assert_eq!(animated, 5.0);
//     }

//     /// Iced colors should animate correctly.
//     #[test]
//     fn animate_color() {
//         let start = iced::Color::BLACK;
//         let end = iced::Color::WHITE;
//         let animated = start.animate_to(&end, 0.5, Curve::Linear);

//         assert_eq!(animated, iced::Color::from_rgb(0.5, 0.5, 0.5));
//     }
// }
