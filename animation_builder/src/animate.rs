use crate::curve::Curve;

pub trait Animate {
    fn animate(start: &Self, end: &Self, progress: f32, curve: Curve) -> Self;
}

impl Animate for f32 {
    fn animate(start: &Self, end: &Self, progress: f32, curve: Curve) -> Self {
        let value_range = end - start;
        curve.value(progress) * value_range + start
    }
}

impl Animate for iced::Color {
    fn animate(start: &Self, end: &Self, progress: f32, curve: Curve) -> Self {
        let r = f32::animate(&start.r, &end.r, progress, curve);
        let g = f32::animate(&start.g, &end.g, progress, curve);
        let b = f32::animate(&start.b, &end.b, progress, curve);
        let a = f32::animate(&start.a, &end.a, progress, curve);
        Self::from_rgba(r, g, b, a)
    }
}

impl Animate for iced::theme::Palette {
    fn animate(start: &Self, end: &Self, progress: f32, curve: Curve) -> Self {
        let background = Animate::animate(&start.background, &end.background, progress, curve);
        let text = Animate::animate(&start.text, &end.text, progress, curve);
        let primary = Animate::animate(&start.primary, &end.primary, progress, curve);
        let success = Animate::animate(&start.success, &end.success, progress, curve);
        let danger = Animate::animate(&start.danger, &end.danger, progress, curve);
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
    fn animate(start: &Self, end: &Self, progress: f32, curve: Curve) -> Self {
        match progress {
            ..=0.0 => start.clone(),
            1.0.. => end.clone(),
            _ => iced::Theme::custom(
                if progress < 0.5 {
                    start.to_string()
                } else {
                    end.to_string()
                },
                Animate::animate(&start.palette(), &end.palette(), progress, curve),
            ),
        }
    }
}

impl Animate for iced::theme::palette::Pair {
    fn animate(start: &Self, end: &Self, progress: f32, curve: Curve) -> Self {
        let color = Animate::animate(&start.color, &end.color, progress, curve);
        let text = Animate::animate(&start.text, &end.text, progress, curve);
        Self { color, text }
    }
}

impl Animate for iced::theme::palette::Primary {
    fn animate(start: &Self, end: &Self, progress: f32, curve: Curve) -> Self {
        let strong = Animate::animate(&start.strong, &end.strong, progress, curve);
        let base = Animate::animate(&start.base, &end.base, progress, curve);
        let weak = Animate::animate(&start.weak, &end.weak, progress, curve);
        Self { strong, base, weak }
    }
}

impl Animate for iced::theme::palette::Secondary {
    fn animate(start: &Self, end: &Self, progress: f32, curve: Curve) -> Self {
        let strong = Animate::animate(&start.strong, &end.strong, progress, curve);
        let base = Animate::animate(&start.base, &end.base, progress, curve);
        let weak = Animate::animate(&start.weak, &end.weak, progress, curve);
        Self { strong, base, weak }
    }
}

impl Animate for iced::theme::palette::Success {
    fn animate(start: &Self, end: &Self, progress: f32, curve: Curve) -> Self {
        let strong = Animate::animate(&start.strong, &end.strong, progress, curve);
        let base = Animate::animate(&start.base, &end.base, progress, curve);
        let weak = Animate::animate(&start.weak, &end.weak, progress, curve);
        Self { strong, base, weak }
    }
}

impl Animate for iced::theme::palette::Danger {
    fn animate(start: &Self, end: &Self, progress: f32, curve: Curve) -> Self {
        let strong = Animate::animate(&start.strong, &end.strong, progress, curve);
        let base = Animate::animate(&start.base, &end.base, progress, curve);
        let weak = Animate::animate(&start.weak, &end.weak, progress, curve);
        Self { strong, base, weak }
    }
}

impl Animate for iced::theme::palette::Background {
    fn animate(start: &Self, end: &Self, progress: f32, curve: Curve) -> Self {
        let strong = Animate::animate(&start.strong, &end.strong, progress, curve);
        let base = Animate::animate(&start.base, &end.base, progress, curve);
        let weak = Animate::animate(&start.weak, &end.weak, progress, curve);
        Self { strong, base, weak }
    }
}

impl Animate for iced::theme::palette::Extended {
    fn animate(start: &Self, end: &Self, progress: f32, curve: Curve) -> Self {
        let is_dark = if progress < 0.5 {
            start.is_dark
        } else {
            end.is_dark
        };

        let primary = Animate::animate(&start.primary, &end.primary, progress, curve);
        let secondary = Animate::animate(&start.secondary, &end.secondary, progress, curve);
        let success = Animate::animate(&start.success, &end.success, progress, curve);
        let danger = Animate::animate(&start.danger, &end.danger, progress, curve);
        let background = Animate::animate(&start.background, &end.background, progress, curve);

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
    fn animate(start: &Self, end: &Self, progress: f32, curve: Curve) -> Self {
        let t1 = T1::animate(&start.0, &end.0, progress, curve);
        let t2 = T2::animate(&start.1, &end.1, progress, curve);
        (t1, t2)
    }
}
