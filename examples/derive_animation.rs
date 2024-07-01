use iced::{
    widget::{button, column, container, text},
    Border, Color, Element, Length,
};
use iced_animation_builder::{Animate, AnimationBuilder};

#[derive(Animate, Clone, PartialEq)]
struct Rectangle {
    width: f32,
    height: f32,
    border_radius: f32,
    color: Color,
}

impl Rectangle {
    /// A fake randomizer that will adjust the rectangle's properties
    fn randomize(&mut self) {
        self.border_radius = (self.border_radius + 20.0) % 32.0;
        self.width = (self.width + 69.0) % 200.0 + 50.0;
        self.height = (self.height + 83.0) % 200.0 + 50.0;
        self.color = Color::from_rgb(
            (self.color.r + 0.3) % 1.0,
            (self.color.g + 0.45) % 1.0,
            (self.color.b + 0.6) % 1.0,
        );
    }
}

#[derive(Debug, Clone)]
enum Message {
    Randomize,
}

struct State {
    rectangle: Rectangle,
}

impl Default for State {
    fn default() -> Self {
        Self {
            rectangle: Rectangle {
                width: 100.0,
                height: 100.0,
                border_radius: 8.0,
                color: Color::from_rgb(1., 0., 0.),
            },
        }
    }
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::Randomize => self.rectangle.randomize(),
        }
    }

    fn view(&self) -> Element<Message> {
        let adjust_button = button(text("Adjust")).on_press(Message::Randomize);

        let animated_box = AnimationBuilder::new(self.rectangle.clone(), |rectangle| {
            container(text(format!(
                "{}x{}",
                rectangle.width as isize, rectangle.height as isize
            )))
            .style(move |_: &iced::Theme| iced::widget::container::Style {
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 1.0,
                    radius: rectangle.border_radius.into(),
                },
                background: Some(rectangle.color.into()),
                ..Default::default()
            })
            .width(Length::Fixed(rectangle.width))
            .height(Length::Fixed(rectangle.height))
            .center_x()
            .center_y()
            .into()
        })
        .animates_layout(true);

        let label = text("Animated rectangle");

        column![adjust_button, animated_box, label]
            .spacing(8)
            .padding(8)
            .width(Length::Shrink)
            .into()
    }
}

pub fn main() -> iced::Result {
    iced::run("Animated size", State::update, State::view)
}
