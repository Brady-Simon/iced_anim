use crate::curve::Curve;

pub trait Animate {
    /// Animates the current value to the `end` value based on the `progress` and `curve`.
    fn animate_to(&self, end: &Self, progress: f32, curve: Curve) -> Self;
}

impl Animate for f32 {
    fn animate_to(&self, end: &Self, progress: f32, curve: Curve) -> Self {
        let value_range = end - self;
        curve.value(progress) * value_range + self
    }
}

impl Animate for iced::Point<f32> {
    fn animate_to(&self, end: &Self, progress: f32, curve: Curve) -> Self {
        let x = self.x.animate_to(&end.x, progress, curve);
        let y = self.y.animate_to(&end.y, progress, curve);
        iced::Point { x, y }
    }
}

impl Animate for iced::Color {
    fn animate_to(&self, end: &Self, progress: f32, curve: Curve) -> Self {
        let r = self.r.animate_to(&end.r, progress, curve);
        let g = self.g.animate_to(&end.g, progress, curve);
        let b = self.b.animate_to(&end.b, progress, curve);
        let a = self.a.animate_to(&end.a, progress, curve);
        iced::Color { r, g, b, a }
    }
}

impl Animate for iced::theme::Palette {
    fn animate_to(&self, end: &Self, progress: f32, curve: Curve) -> Self {
        let background = self.background.animate_to(&end.background, progress, curve);
        let text = self.text.animate_to(&end.text, progress, curve);
        let primary = self.primary.animate_to(&end.primary, progress, curve);
        let success = self.success.animate_to(&end.success, progress, curve);
        let danger = self.danger.animate_to(&end.danger, progress, curve);
        Self {
            background,
            text,
            primary,
            success,
            danger,
        }
    }
}

impl Animate for iced::Theme {
    fn animate_to(&self, end: &Self, progress: f32, curve: Curve) -> Self {
        match progress {
            ..=0.0 => self.clone(),
            1.0.. => end.clone(),
            _ => iced::Theme::custom(
                if progress < 0.5 {
                    self.to_string()
                } else {
                    end.to_string()
                },
                self.palette().animate_to(&end.palette(), progress, curve),
            ),
        }
    }
}

impl Animate for iced::theme::palette::Pair {
    fn animate_to(&self, end: &Self, progress: f32, curve: Curve) -> Self {
        let color = self.color.animate_to(&end.color, progress, curve);
        let text = self.text.animate_to(&end.text, progress, curve);
        Self { color, text }
    }
}

impl Animate for iced::theme::palette::Primary {
    fn animate_to(&self, end: &Self, progress: f32, curve: Curve) -> Self {
        let strong = self.strong.animate_to(&end.strong, progress, curve);
        let base = self.base.animate_to(&end.base, progress, curve);
        let weak = self.weak.animate_to(&end.weak, progress, curve);
        Self { strong, base, weak }
    }
}

impl Animate for iced::theme::palette::Secondary {
    fn animate_to(&self, end: &Self, progress: f32, curve: Curve) -> Self {
        let strong = self.strong.animate_to(&end.strong, progress, curve);
        let base = self.base.animate_to(&end.base, progress, curve);
        let weak = self.weak.animate_to(&end.weak, progress, curve);
        Self { strong, base, weak }
    }
}

impl Animate for iced::theme::palette::Success {
    fn animate_to(&self, end: &Self, progress: f32, curve: Curve) -> Self {
        let strong = self.strong.animate_to(&end.strong, progress, curve);
        let base = self.base.animate_to(&end.base, progress, curve);
        let weak = self.weak.animate_to(&end.weak, progress, curve);
        Self { strong, base, weak }
    }
}

impl Animate for iced::theme::palette::Danger {
    fn animate_to(&self, end: &Self, progress: f32, curve: Curve) -> Self {
        let strong = self.strong.animate_to(&end.strong, progress, curve);
        let base = self.base.animate_to(&end.base, progress, curve);
        let weak = self.weak.animate_to(&end.weak, progress, curve);
        Self { strong, base, weak }
    }
}

impl Animate for iced::theme::palette::Background {
    fn animate_to(&self, end: &Self, progress: f32, curve: Curve) -> Self {
        let strong = self.strong.animate_to(&end.strong, progress, curve);
        let base = self.base.animate_to(&end.base, progress, curve);
        let weak = self.weak.animate_to(&end.weak, progress, curve);
        Self { strong, base, weak }
    }
}

impl Animate for iced::theme::palette::Extended {
    fn animate_to(&self, end: &Self, progress: f32, curve: Curve) -> Self {
        let is_dark = if progress < 0.5 {
            self.is_dark
        } else {
            end.is_dark
        };

        let primary = self.primary.animate_to(&end.primary, progress, curve);
        let secondary = self.secondary.animate_to(&end.secondary, progress, curve);
        let success = self.success.animate_to(&end.success, progress, curve);
        let danger = self.danger.animate_to(&end.danger, progress, curve);
        let background = self.background.animate_to(&end.background, progress, curve);

        Self {
            is_dark,
            primary,
            secondary,
            success,
            danger,
            background,
        }
    }
}

impl<T1, T2> Animate for (T1, T2)
where
    T1: Animate,
    T2: Animate,
{
    fn animate_to(&self, end: &Self, progress: f32, curve: Curve) -> Self {
        let t1 = self.0.animate_to(&end.0, progress, curve);
        let t2 = self.1.animate_to(&end.1, progress, curve);
        (t1, t2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// f32 values should animate correctly.
    #[test]
    fn animate_f32() {
        let start = 0.0;
        let end = 10.0;
        let animated = start.animate_to(&end, 0.5, Curve::Linear);

        assert_eq!(animated, 5.0);
    }

    /// Animations can go in any direction.
    #[test]
    fn animate_f32_backwards() {
        let start = 10.0;
        let end = 0.0;
        let animated = start.animate_to(&end, 0.5, Curve::Linear);

        assert_eq!(animated, 5.0);
    }

    /// Iced colors should animate correctly.
    #[test]
    fn animate_color() {
        let start = iced::Color::BLACK;
        let end = iced::Color::WHITE;
        let animated = start.animate_to(&end, 0.5, Curve::Linear);

        assert_eq!(animated, iced::Color::from_rgb(0.5, 0.5, 0.5));
    }
}
