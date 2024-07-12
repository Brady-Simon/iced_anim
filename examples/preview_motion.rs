use iced::{
    widget::{button, column, container, pick_list, row, text, Column, Row, Space, Stack},
    Border, Element, Length,
};
use iced_anim::{animation_builder::AnimationBuilder, SpringMotion};

const CIRCLE_DIAMETER: f32 = 50.0;

const MAX_OFFSET: f32 = 200.0;

#[derive(Debug, Clone)]
enum Message {
    ToggleOffset,
    ChangeMotion(SpringMotion),
}

struct State {
    offset: f32,
    motion: SpringMotion,
}

impl Default for State {
    fn default() -> Self {
        Self {
            offset: 0.0,
            motion: SpringMotion::default(),
        }
    }
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleOffset => {
                self.offset = if self.offset == 0.0 { MAX_OFFSET } else { 0.0 };
            }
            Message::ChangeMotion(motion) => {
                println!("Change motion to {motion:?}");
                self.motion = motion;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let motion_picker = pick_list(
            [
                SpringMotion::Smooth,
                SpringMotion::Snappy,
                SpringMotion::Bouncy,
            ],
            Some(self.motion),
            Message::ChangeMotion,
        );

        let toggle_button = button(text("Toggle")).on_press(Message::ToggleOffset);
        let buttons = row![motion_picker, toggle_button].spacing(8);

        let animated_circles = AnimationBuilder::new(self.offset, |offset| {
            row![
                // Vertical circle
                circle_track(offset, false),
                // Horizontal circle
                circle_track(MAX_OFFSET - offset, true),
            ]
            .width(Length::Fill)
            .spacing(8.0)
            .into()
        })
        .motion(self.motion)
        .animates_layout(true);

        container(column![buttons, animated_circles].spacing(8).padding(8))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn circle_track<'a>(offset: f32, is_horizontal: bool) -> Element<'a, Message> {
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

    let track_items = vec![circle(false, offset), spacer.into(), circle(false, offset)];
    let track_circles: Element<'a, Message, _, _> = if is_horizontal {
        Row::with_children(track_items).into()
    } else {
        Column::with_children(track_items).into()
    };
    let track_background = container(track_circles).width(width).height(height);

    let mut track = Stack::new().push(track_background);

    let main_track_circle: Element<'a, Message, _, _> = if is_horizontal {
        Row::new()
            .push(Space::new(
                Length::Fixed(offset),
                Length::Fixed(CIRCLE_DIAMETER),
            ))
            .push(circle(true, offset))
            .into()
    } else {
        Column::new()
            .push(Space::new(
                Length::Fixed(CIRCLE_DIAMETER),
                Length::Fixed(offset),
            ))
            .push(circle(true, offset))
            .into()
    };

    track = track.push(main_track_circle);

    track.into()
}

fn circle<'a>(is_active: bool, offset: f32) -> Element<'a, Message> {
    container(Space::new(Length::Fill, Length::Fill))
        .style(move |theme: &iced::Theme| {
            let color = if is_active {
                circle_color(offset)
            } else {
                theme.palette().text
            };
            iced::widget::container::Style {
                border: Border {
                    color,
                    width: 4.0,
                    radius: (CIRCLE_DIAMETER / 2.0).into(),
                },
                background: is_active.then_some(color.into()),
                ..Default::default()
            }
        })
        .width(CIRCLE_DIAMETER)
        .height(CIRCLE_DIAMETER)
        .into()
}

fn circle_color(offset: f32) -> iced::Color {
    let ratio = offset / MAX_OFFSET;
    iced::Color::from_rgb(ratio, 1.0 - ratio, 0.75)
}

pub fn main() -> iced::Result {
    iced::run("Preview Motion", State::update, State::view)
}
