use crate::curve::Curve;

pub trait Animate {
    fn animate(start: &Self, end: &Self, progress: f32, curve: Curve) -> Self;
}

impl Animate for f32 {
    fn animate(start: &f32, end: &f32, progress: f32, curve: Curve) -> Self {
        let value_range = end - start;
        match curve {
            Curve::Linear => start + value_range * progress,
            Curve::EaseIn => start + value_range * progress.powi(2),
            Curve::EaseOut => start + value_range * progress.sqrt(),
            Curve::EaseInOut => {
                let progress = progress * 2.0;
                if progress < 1.0 {
                    start + (value_range * 0.5) * progress.powi(2)
                } else {
                    start + value_range * -0.5 * ((progress - 1.0) * (progress - 3.0) - 1.0)
                }
            }
        }
    }
}

impl Animate for iced::Color {
    fn animate(start: &iced::Color, end: &iced::Color, progress: f32, curve: Curve) -> Self {
        let r = f32::animate(&start.r, &end.r, progress, curve);
        let g = f32::animate(&start.g, &end.g, progress, curve);
        let b = f32::animate(&start.b, &end.b, progress, curve);
        let a = f32::animate(&start.a, &end.a, progress, curve);
        iced::Color::from_rgba(r, g, b, a)
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
