use iced::{
    widget::{button, container, horizontal_space, row, text, Space, Stack},
    Border, Color, Element, Length, Theme,
};
use iced_anim::animation_builder;

#[derive(Debug, Clone)]
enum Message {
    ToggleDrawer,
}

struct State {
    is_drawer_open: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            is_drawer_open: false,
        }
    }
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleDrawer => {
                self.is_drawer_open = !self.is_drawer_open;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let drawer_button = button(text("Open Drawer")).on_press(Message::ToggleDrawer);

        let x = drawer(self.is_drawer_open, drawer_button);

        x.into()
    }
}

fn drawer<'a>(is_open: bool, content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    // The width of the drawer when open
    let width = if is_open { 350.0 } else { 0.0 };

    // The underlay background color
    let background = if is_open {
        Color::from_rgba(0.0, 0.0, 0.0, 0.75)
    } else {
        Color::TRANSPARENT
    };

    let drawer_stack = Stack::new()
        .width(Length::Fill)
        .height(Length::Fill)
        .push(content)
        .push(row![animation_builder(background, move |background| {
            row![
                button(container(Space::new(Length::Fill, Length::Fill)).center(Length::Fill))
                    .on_press_maybe(is_open.then_some(Message::ToggleDrawer))
                    .style(move |_, _| iced::widget::button::Style {
                        background: Some(background.into()),
                        ..Default::default()
                    }),
                container(
                    animation_builder(width, |width| {
                        container(drawer_header())
                            .style(move |theme: &Theme| iced::widget::container::Style {
                                background: Some(
                                    theme.extended_palette().background.base.color.into(),
                                ),
                                border: Border::rounded(8),
                                ..Default::default()
                            })
                            .padding(8)
                            .align_x(iced::alignment::Horizontal::Right)
                            .fill_y()
                            .center_x(Length::Fixed(width))
                            .into()
                    })
                    .animates_layout(true)
                )
                .padding(8)
                .style(move |_| iced::widget::container::Style {
                    background: Some(background.into()),
                    ..Default::default()
                })
            ]
            .into()
        })]);

    container(drawer_stack)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

// A helper function to create a demo drawer header
fn drawer_header() -> Element<'static, Message> {
    row![
        horizontal_space(),
        container(text("Drawer Title").size(18)).width(Length::Fill),
        button(text("Close ›").shaping(text::Shaping::Advanced))
            .on_press(Message::ToggleDrawer)
            .style(|theme: &Theme, _status| {
                iced::widget::button::Style {
                    text_color: theme.extended_palette().primary.base.color,
                    background: Some(Color::TRANSPARENT.into()),
                    ..Default::default()
                }
            }),
    ]
    .align_items(iced::Alignment::Center)
    .spacing(8)
    .into()
}

pub fn main() -> iced::Result {
    iced::run("Animated Drawer", State::update, State::view)
}
