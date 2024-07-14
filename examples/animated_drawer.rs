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
    let width = if is_open { 300.0 } else { 0.0 };

    // The underlay background color
    let background = if is_open {
        Color::from_rgba(0.0, 0.0, 0.0, 0.5)
    } else {
        Color::TRANSPARENT
    };

    let drawer_stack = Stack::new()
        .width(Length::Fill)
        .height(Length::Fill)
        .push(content)
        .push(animation_builder(background, move |background| {
            button(container(Space::new(Length::Fill, Length::Fill)).center(Length::Fill))
                .on_press_maybe(is_open.then_some(Message::ToggleDrawer))
                .style(move |_, _| iced::widget::button::Style {
                    background: Some(background.into()),
                    ..Default::default()
                })
                .into()
        }))
        .push(row![
            // animation_builder(background, move |background| {
            //     button(container(Space::new(Length::Fill, Length::Fill)).center(Length::Fill))
            //         .on_press_maybe(is_open.then_some(Message::ToggleDrawer))
            //         .style(move |_, _| iced::widget::button::Style {
            //             background: Some(background.into()),
            //             ..Default::default()
            //         })
            //         .into()
            // }),
            horizontal_space(),
            container(
                animation_builder(width, |width| {
                    container(text("Drawer content"))
                        .style(move |theme: &Theme| iced::widget::container::Style {
                            background: Some(theme.extended_palette().background.weak.color.into()),
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
        ]);

    container(drawer_stack)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

pub fn main() -> iced::Result {
    iced::run("Animated Drawer", State::update, State::view)
}
