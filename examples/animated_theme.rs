use std::time::Duration;

use iced::{
    theme::palette::{Extended, Pair},
    widget::{column, container, pick_list, row, text, tooltip, Row, Space},
    Border, Element, Length, Theme,
};
use iced_animation_builder::AnimationBuilder;

#[derive(Debug, Clone)]
enum Message {
    ChangeTheme(Theme),
}

struct State {
    theme: Theme,
}

impl Default for State {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
        }
    }
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::ChangeTheme(theme) => {
                self.theme = theme;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        AnimationBuilder::new(self.theme.clone(), move |theme| {
            container(
                row![
                    pick_list(Theme::ALL, Some(theme.clone()), Message::ChangeTheme),
                    palette_grid(theme.extended_palette()),
                ]
                .spacing(8),
            )
            .padding(8)
            .style(move |_| container::Style {
                background: Some(theme.palette().background.into()),
                ..Default::default()
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        })
        .duration(Duration::from_millis(500))
        .into()
    }
}

fn palette_grid<'a>(palette: &Extended) -> Element<'a, Message> {
    // The various shades of a palette
    let shades = [
        (
            "Primary",
            palette.primary.strong,
            palette.primary.base,
            palette.primary.weak,
        ),
        (
            "Secondary",
            palette.secondary.strong,
            palette.secondary.base,
            palette.secondary.weak,
        ),
        (
            "Success",
            palette.success.strong,
            palette.success.base,
            palette.success.weak,
        ),
        (
            "Danger",
            palette.danger.strong,
            palette.danger.base,
            palette.danger.weak,
        ),
        (
            "Background",
            palette.background.strong,
            palette.background.base,
            palette.background.weak,
        ),
    ];

    container(Row::with_children(shades.map(
        |(name, strong, base, weak)| {
            column![
                pair_square(format!("{name} weak"), weak),
                pair_square(format!("{name} base"), base),
                pair_square(format!("{name} strong"), strong),
            ]
            .into()
        },
    )))
    .padding(1.0)
    .style(|theme: &iced::Theme| container::Style {
        border: Border {
            color: theme.palette().text,
            width: 1.0,
            ..Default::default()
        },
        ..Default::default()
    })
    .into()
}

fn pair_square<'a>(name: String, pair: Pair) -> Element<'a, Message> {
    tooltip(
        container(Space::new(Length::Shrink, Length::Shrink))
            .width(Length::Fixed(48.0))
            .height(Length::Fixed(48.0))
            .style(move |_| container::Style {
                background: Some(pair.color.into()),
                ..Default::default()
            }),
        container(text(name).style(|theme: &iced::Theme| text::Style {
            color: Some(theme.palette().text),
        }))
        .style(|theme: &iced::Theme| container::Style {
            background: Some(theme.extended_palette().background.weak.color.into()),
            border: Border {
                color: theme.extended_palette().background.base.color,
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        })
        .padding(8),
        tooltip::Position::Top,
    )
    .into()
}

pub fn main() -> iced::Result {
    iced::program("Animated theme", State::update, State::view)
        .theme(|state| state.theme.clone())
        .run()
}
