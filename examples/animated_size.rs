use iced::{
    widget::{button, column, container, row, text},
    Border, Element, Length,
};
use iced_anim::{animation_builder::AnimationBuilder, transition::Easing};

#[derive(Debug, Clone)]
enum Message {
    AdjustSize(f32),
}

struct State {
    size: f32,
}

impl Default for State {
    fn default() -> Self {
        Self { size: 50.0 }
    }
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::AdjustSize(dx) => {
                self.size = (self.size + dx).max(0.0);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let buttons = row![
            button(text("-150")).on_press(Message::AdjustSize(-150.0)),
            button(text("-50")).on_press(Message::AdjustSize(-50.0)),
            button(text("+50")).on_press(Message::AdjustSize(50.0)),
            button(text("+150")).on_press(Message::AdjustSize(150.0)),
        ]
        .spacing(8);

        let animated_box = AnimationBuilder::new(self.size, |size| {
            container(text(size as isize))
                .style(move |theme: &iced::Theme| iced::widget::container::Style {
                    border: Border {
                        color: theme.extended_palette().secondary.strong.color,
                        width: 1.0,
                        radius: 6.0.into(),
                    },
                    background: Some(theme.extended_palette().secondary.weak.color.into()),
                    ..Default::default()
                })
                .center(size)
                .into()
        })
        .animates_layout(true)
        .animation(Easing::EASE);

        let label = text("Animated size");

        column![buttons, animated_box, label]
            .spacing(8)
            .padding(8)
            .width(Length::Shrink)
            .into()
    }
}

pub fn main() -> iced::Result {
    iced::run("Animated size", State::update, State::view)
}
