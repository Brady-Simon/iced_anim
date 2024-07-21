//! An example of a stateful animation using the `Animation` widget instead of
//! the `AnimationBuilder` widget. Stateful animations store the animation
//! within the app state whereas the `AnimationBuilder` widget stores the
//! animation within the widget tree.
use std::time::Duration;

use iced::{
    widget::{container, mouse_area, Space},
    Border, Color, Element, Length, Point, Size, Subscription, Theme,
};
use iced_anim::{Animation, Spring, SpringEvent, SpringMotion};

#[derive(Debug, Clone)]
enum Message {
    UpdatePosition(SpringEvent<Point>),
    Resized(u32, u32),
}

struct State {
    /// The current position of the bubble in the window.
    position: Spring<Point>,
    /// The size of the window to help generate a color.
    size: Size<u32>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            position: Spring::new(Point::new(50.0, 50.0)).with_motion(SpringMotion::Custom {
                response: Duration::from_millis(500),
                damping: 0.6,
            }),
            size: Size::new(1024, 768),
        }
    }
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::UpdatePosition(event) => self.position.update(event),
            Message::Resized(width, height) => self.size = Size::new(width, height),
        }
    }

    /// Listens for window resize events to ensure the bubble color is accurate.
    fn subscription(&self) -> Subscription<Message> {
        iced::event::listen_with(|event, _, _| match event {
            iced::Event::Window(iced::window::Event::Resized { width, height }) => {
                Some(Message::Resized(width, height))
            }
            _ => None,
        })
    }

    fn view(&self) -> Element<Message> {
        let size = self.size;
        container(
            mouse_area(
                Animation::new(
                    &self.position,
                    container(
                        container(Space::new(Length::Fill, Length::Fill))
                            .width(self.position.value().x)
                            .height(self.position.value().y)
                            .style(move |_: &Theme| container::Style {
                                border: Border::rounded(8),
                                background: Some(get_color(self.position.value(), size).into()),
                                ..Default::default()
                            }),
                    )
                    .fill(),
                )
                .on_update(Message::UpdatePosition),
            )
            .on_move(|point| Message::UpdatePosition(point.into())),
        )
        .fill()
        .into()
    }
}

/// Returns a color based on the position and size of the screen.
fn get_color(position: &Point, size: Size<u32>) -> Color {
    let width: f32 = (size.width as f32).max(1.0);
    let height: f32 = (size.height as f32).max(1.0);
    let x_ratio = position.x / width;
    let y_ratio = position.y / height;

    Color::from_rgb(x_ratio, y_ratio, 1.0 - x_ratio)
}

pub fn main() -> iced::Result {
    iced::application("Stateful Animation", State::update, State::view)
        .subscription(State::subscription)
        .run()
}
