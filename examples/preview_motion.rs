use iced::{
    widget::{button, column, container, pick_list, radio, row, stack, text, Column, Row, Space},
    Alignment::Center,
    Border, Element, Length,
};
use iced_anim::{
    animated::AnimationConfig, animation_builder::AnimationBuilder, spring::Motion,
    transition::Curve,
};

const CIRCLE_DIAMETER: f32 = 50.0;

const MAX_OFFSET: f32 = 200.0;

/// A version of [`Motion`] that has an associated `name` for display purposes.
#[derive(Debug, Clone, PartialEq)]
struct PreviewMotion {
    name: &'static str,
    motion: Motion,
}

impl std::fmt::Display for PreviewMotion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct PreviewCurve {
    name: &'static str,
    curve: Curve,
}

impl std::fmt::Display for PreviewCurve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

const MOTIONS: [PreviewMotion; 3] = [
    PreviewMotion {
        name: "Smooth",
        motion: Motion::SMOOTH,
    },
    PreviewMotion {
        name: "Snappy",
        motion: Motion::SNAPPY,
    },
    PreviewMotion {
        name: "Bouncy",
        motion: Motion::BOUNCY,
    },
];

const CURVES: [PreviewCurve; 6] = [
    PreviewCurve {
        name: "Linear",
        curve: Curve::Linear,
    },
    PreviewCurve {
        name: "Ease",
        curve: Curve::Ease,
    },
    PreviewCurve {
        name: "Ease In",
        curve: Curve::EaseIn,
    },
    PreviewCurve {
        name: "Ease Out",
        curve: Curve::EaseOut,
    },
    PreviewCurve {
        name: "Ease In Out",
        curve: Curve::EaseInOut,
    },
    PreviewCurve {
        name: "Ease In Out Circular",
        curve: Curve::EaseInOutCirc,
    },
];

#[derive(Debug, Clone)]
enum Message {
    SelectAnimationType(bool),
    ToggleOffset,
    ChangeMotion(PreviewMotion),
    ChangeCurve(PreviewCurve),
}

struct State {
    is_spring: bool,
    offset: f32,
    preview_motion: PreviewMotion,
    preview_curve: PreviewCurve,
}

impl Default for State {
    fn default() -> Self {
        Self {
            is_spring: true,
            offset: -MAX_OFFSET,
            preview_motion: MOTIONS[0].clone(),
            preview_curve: CURVES[0].clone(),
        }
    }
}

impl State {
    pub fn animation_config(&self) -> AnimationConfig {
        if self.is_spring {
            AnimationConfig::spring(self.preview_motion.motion)
        } else {
            AnimationConfig::transition(self.preview_curve.curve)
        }
    }
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleOffset => {
                self.offset *= -1.0;
            }
            Message::SelectAnimationType(is_spring) => {
                self.is_spring = is_spring;
            }
            Message::ChangeMotion(motion) => {
                self.preview_motion = motion;
            }
            Message::ChangeCurve(curve) => {
                self.preview_curve = curve;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let radio_buttons = column![
            radio(
                "Spring",
                true,
                Some(self.is_spring),
                Message::SelectAnimationType,
            ),
            radio(
                "Transition",
                false,
                Some(self.is_spring),
                Message::SelectAnimationType,
            ),
        ];

        let animation_picker: Element<Message> = if self.is_spring {
            pick_list(
                MOTIONS,
                Some(self.preview_motion.clone()),
                Message::ChangeMotion,
            )
            .into()
        } else {
            pick_list(
                CURVES,
                Some(self.preview_curve.clone()),
                Message::ChangeCurve,
            )
            .into()
        };

        let toggle_button = button(text("Toggle")).on_press(Message::ToggleOffset);
        let buttons = row![animation_picker, toggle_button].spacing(8);
        let header = row![radio_buttons, buttons].spacing(16).align_y(Center);

        let animated_circles = container(
            AnimationBuilder::new(self.offset, |offset| {
                container(
                    stack![
                        track_background(false),
                        track_background(true),
                        // Vertical circle
                        circle_track(offset, false),
                        // Horizontal circle
                        circle_track(offset, true),
                    ]
                    .width(Length::Fill)
                    .height(Length::Fill),
                )
                .center(Length::Fill)
                .into()
            })
            .animation(self.animation_config())
            .animates_layout(true),
        )
        .center(Length::Fill);

        container(column![header, animated_circles].spacing(8).padding(8))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

/// The background circles showing the final animation positions.
fn track_background<'a>(is_horizontal: bool) -> Element<'a, Message> {
    let width: Length = if is_horizontal {
        Length::Fill
    } else {
        Length::Fixed(CIRCLE_DIAMETER)
    };
    let height: Length = if is_horizontal {
        Length::Fixed(CIRCLE_DIAMETER)
    } else {
        Length::Fill
    };
    let spacer = if is_horizontal {
        Space::with_width(MAX_OFFSET - CIRCLE_DIAMETER)
    } else {
        Space::with_height(MAX_OFFSET - CIRCLE_DIAMETER)
    };

    let track_items = vec![
        circle(None, is_horizontal),
        spacer.into(),
        circle(None, is_horizontal),
    ];
    let track_circles: Element<'a, Message, _, _> = if is_horizontal {
        Row::with_children(track_items).into()
    } else {
        Column::with_children(track_items).into()
    };
    container(track_circles)
        .width(width)
        .height(height)
        .center(Length::Fill)
        .into()
}

/// The circle that animates along the track.
fn circle_track<'a>(offset: f32, is_horizontal: bool) -> Element<'a, Message> {
    if is_horizontal {
        container(
            Row::new()
                .push_maybe(offset.is_sign_positive().then_some(Space::new(
                    Length::Fixed(offset),
                    Length::Fixed(CIRCLE_DIAMETER),
                )))
                .push(circle(Some(offset), is_horizontal))
                .push_maybe(offset.is_sign_negative().then_some(Space::new(
                    Length::Fixed(-offset),
                    Length::Fixed(CIRCLE_DIAMETER),
                ))),
        )
        .center(Length::Fill)
        .into()
    } else {
        container(
            Column::new()
                .push_maybe(offset.is_sign_positive().then_some(Space::new(
                    Length::Fixed(CIRCLE_DIAMETER),
                    Length::Fixed(offset),
                )))
                .push(circle(Some(offset), is_horizontal))
                .push_maybe(offset.is_sign_negative().then_some(Space::new(
                    Length::Fixed(CIRCLE_DIAMETER),
                    Length::Fixed(-offset),
                ))),
        )
        .center(Length::Fill)
        .into()
    }
}

/// A circle that animates along the track or a background circle.
fn circle<'a>(offset: Option<f32>, is_horizontal: bool) -> Element<'a, Message> {
    container(Space::new(Length::Fill, Length::Fill))
        .style(move |theme: &iced::Theme| {
            let color = if let Some(offset) = offset {
                circle_color(offset, is_horizontal)
            } else {
                theme.palette().text
            };
            iced::widget::container::Style {
                border: Border {
                    color,
                    width: 4.0,
                    radius: (CIRCLE_DIAMETER / 2.0).into(),
                },
                background: offset
                    .map(|offset| circle_color(offset, is_horizontal))
                    .map(Into::into),
                ..Default::default()
            }
        })
        .width(CIRCLE_DIAMETER)
        .height(CIRCLE_DIAMETER)
        .into()
}

/// Gets a unique color based on the current offset.
fn circle_color(offset: f32, is_horizontal: bool) -> iced::Color {
    let ratio = (offset + MAX_OFFSET) / MAX_OFFSET / 2.0;

    if is_horizontal {
        iced::Color::from_rgb(1.0 - ratio, 0.75, ratio)
    } else {
        iced::Color::from_rgb(ratio, 1.0 - ratio, 0.75)
    }
}

pub fn main() -> iced::Result {
    iced::run("Preview Motion", State::update, State::view)
}
