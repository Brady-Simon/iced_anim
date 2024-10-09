use iced::{
    widget::{column, container, text},
    Element,
    Length::Fill,
};
use iced_anim::widget::button;

#[derive(Debug, Clone)]
enum Message {
    Adjust(i32),
}

#[derive(Debug, Default)]
struct State {
    counter: i32,
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::Adjust(amount) => self.counter += amount,
        };
    }

    fn view(&self) -> Element<Message> {
        container(
            column![
                button(text("+")).on_press(Message::Adjust(1)),
                text(self.counter.to_string()).size(24),
                button(text("-")).on_press(Message::Adjust(-1)),
            ]
            .spacing(8)
            .padding(8),
        )
        .center(Fill)
        .into()
    }
}

pub fn main() -> iced::Result {
    iced::run("Animated widgets", State::update, State::view)
}
