use iced::{
    widget::{button, column, container, row, text},
    Color, Element, Length,
};
use iced_anim::{animation::animation, transition::Easing, Animated, Event};

#[derive(Debug, Clone)]
enum Message {
    AdjustSize(Event<f32>),
}

struct State {
    size: Animated<f32>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            size: Animated::transition(50.0, Easing::EASE),
        }
    }
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::AdjustSize(event) => self.size.update(event),
        }
    }

    fn view(&self) -> Element<Message> {
        let buttons = row![
            button(text("-50")).on_press(Message::AdjustSize((self.size.target() - 50.0).into())),
            button(text("+50")).on_press(Message::AdjustSize((self.size.target() + 50.0).into())),
        ]
        .spacing(8);

        let animated_box = animation(
            &self.size,
            container(text(*self.size.value() as isize))
                .style(|_| container::background(Color::from_rgb(1.0, 0.0, 0.0)))
                .center(*self.size.value()),
        )
        .on_update(Message::AdjustSize);

        column![buttons, animated_box]
            .spacing(8)
            .padding(8)
            .width(Length::Shrink)
            .into()
    }
}

pub fn main() -> iced::Result {
    iced::run("Animated size", State::update, State::view)
}
