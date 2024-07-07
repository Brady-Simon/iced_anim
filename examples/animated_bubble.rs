use iced::{
    widget::{container, MouseArea, Space},
    Border, Color, Element, Length, Padding, Point, Size, Subscription, Theme,
};
use iced_anim::animation_builder::AnimationBuilder;

const BUBBLE_SIZE: f32 = 50.0;

#[derive(Debug, Clone)]
enum Message {
    MoveTo(Point),
    Resized(u32, u32),
}

struct State {
    /// The current position of the bubble in the window.
    position: Point,
    /// The size of the window to help generate a color.
    size: Size<u32>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            position: Point::new(BUBBLE_SIZE / 2.0, BUBBLE_SIZE / 2.0),
            size: Size::new(1024, 768),
        }
    }
}

/// Returns a color based on the position and size of the bubble.
fn get_bubble_color(position: Point, size: Size<u32>) -> Color {
    let width: f32 = (size.width as f32).max(1.0);
    let height: f32 = (size.height as f32).max(1.0);
    let x_ratio = position.x / width;
    let y_ratio = position.y / height;

    Color::from_rgb(x_ratio, y_ratio, 1.0 - x_ratio)
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::MoveTo(position) => self.position = position,
            Message::Resized(width, height) => self.size = Size::new(width, height),
        }
    }

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
            AnimationBuilder::new(self.position, move |position| {
                MouseArea::new(
                    container(
                        container(Space::new(Length::Fill, Length::Fill))
                            .width(Length::Fixed(BUBBLE_SIZE))
                            .height(Length::Fixed(BUBBLE_SIZE))
                            .style(move |_: &Theme| iced::widget::container::Style {
                                border: Border::rounded(BUBBLE_SIZE / 2.0),
                                background: Some(get_bubble_color(position, size).into()),
                                ..Default::default()
                            }),
                    )
                    .padding(Padding::from([
                        position.y - BUBBLE_SIZE / 2.0,
                        0.0,
                        0.0,
                        position.x - BUBBLE_SIZE / 2.0,
                    ]))
                    .fill(),
                )
                .on_move(Message::MoveTo)
                .into()
            })
            .animates_layout(true),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

pub fn main() -> iced::Result {
    iced::application("Animated bubble", State::update, State::view)
        .subscription(State::subscription)
        .run()
}
