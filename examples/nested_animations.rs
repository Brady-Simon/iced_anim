use iced::{
    widget::{button, column, container, row, text},
    Border, Color, Element, Length,
};
use iced_anim::animation_builder::AnimationBuilder;

#[derive(Debug, Clone)]
enum Message {
    AdjustAll,
    AdjustSize,
    AdjustColor,
}

struct State {
    size: f32,
    color: Color,
}

const CYAN: Color = Color::from_rgb(0.0, 0.8, 0.8);
const MAGENTA: Color = Color::from_rgb(1.0, 0.0, 1.0);

impl Default for State {
    fn default() -> Self {
        Self {
            size: 50.0,
            color: CYAN,
        }
    }
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::AdjustAll => {
                self.size = if self.size == 50.0 { 150.0 } else { 50.0 };
                self.color = if self.color == CYAN { MAGENTA } else { CYAN };
            }
            Message::AdjustSize => self.size = if self.size == 50.0 { 150.0 } else { 50.0 },
            Message::AdjustColor => self.color = if self.color == CYAN { MAGENTA } else { CYAN },
        }
    }

    fn view(&self) -> Element<Message> {
        let buttons = row![
            button(text("Adjust size")).on_press(Message::AdjustSize),
            button(text("Adjust color")).on_press(Message::AdjustColor),
            button(text("Adjust all")).on_press(Message::AdjustAll),
        ]
        .spacing(8);

        let animated_box = AnimationBuilder::new(self.size, |size| {
            AnimationBuilder::new(self.color, move |color| {
                container(text(size as isize))
                    .style(move |_: &iced::Theme| container::Style {
                        border: Border {
                            color,
                            width: 1.0,
                            radius: 6.0.into(),
                        },
                        background: Some(color.into()),
                        ..Default::default()
                    })
                    .center(size)
                    .into()
            })
            .animates_layout(true)
            .into()
        })
        .animates_layout(true);

        column![buttons, animated_box]
            .spacing(8)
            .padding(8)
            .width(Length::Shrink)
            .into()
    }
}

pub fn main() -> iced::Result {
    iced::run("Nested AnimationBuilders", State::update, State::view)
}
