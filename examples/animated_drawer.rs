use iced::{
    advanced::Widget,
    widget::{button, container, horizontal_space, row, text, Space, Stack},
    Border, Color, Element, Length, Point, Rectangle, Size, Theme, Vector,
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
        let offset_button = Offset::new(drawer_button).offset(Point::new(1000.0, 150.0));
        drawer(self.is_drawer_open, offset_button)
    }
}

fn drawer<'a>(is_open: bool, content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    // The width of the drawer when open
    const MAX_WIDTH: f32 = 350.0;
    let width = if is_open { MAX_WIDTH } else { 0.0 };

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
        .push(
            animation_builder((background, width), move |(background, width)| {
                println!("Animating width: {}", width);
                row![
                    // Underlay
                    button(container(Space::new(Length::Fill, Length::Fill)).center(Length::Fill))
                        .on_press_maybe(is_open.then_some(Message::ToggleDrawer))
                        .style(move |_, _| iced::widget::button::Style {
                            background: Some(background.into()),
                            ..Default::default()
                        }),
                    // Drawer content
                    Offset::new(
                        container(
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
                                .center_x(Length::Fixed(MAX_WIDTH))
                        )
                        .padding(8)
                        .style(move |_| iced::widget::container::Style {
                            background: Some(background.into()),
                            ..Default::default()
                        })
                    )
                    .offset(Point::new(width, 20.0))
                ]
                .into()
            })
            .animates_layout(true),
        );

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

struct Offset<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: iced::advanced::Renderer,
{
    offset: Point,
    content: Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message, Theme, Renderer> Offset<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: iced::advanced::Renderer,
{
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            offset: Point::ORIGIN,
            content: content.into(),
        }
    }

    pub fn offset(mut self, offset: Point) -> Self {
        self.offset = offset;
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Offset<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: iced::advanced::Renderer,
{
    fn size(&self) -> iced::Size<Length> {
        // self.content.as_widget().size()
        Size::new(Length::Fixed(350.0), Length::Fill)
    }

    fn children(&self) -> Vec<iced::advanced::widget::Tree> {
        self.content.as_widget().children()
    }

    fn diff(&self, tree: &mut iced::advanced::widget::Tree) {
        self.content.as_widget().diff(tree);
    }

    fn mouse_interaction(
        &self,
        state: &iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> iced::advanced::mouse::Interaction {
        self.content
            .as_widget()
            .mouse_interaction(state, layout, cursor, viewport, renderer)
    }

    fn layout(
        &self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        self.content
            .as_widget()
            .layout(tree, renderer, limits)
            // .translate(Vector::new(self.offset.x, self.offset.y))
            .translate(Vector::new(self.offset.x, self.offset.y))
    }

    fn on_event(
        &mut self,
        state: &mut iced::advanced::widget::Tree,
        event: iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        self.content.as_widget_mut().on_event(
            state, event, layout, cursor, renderer, clipboard, shell, viewport,
        )
    }

    fn operate(
        &self,
        state: &mut iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced::advanced::widget::Operation<()>,
    ) {
        self.content
            .as_widget()
            .operate(state, layout, renderer, operation);
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        renderer: &Renderer,
        translation: iced::Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, Renderer>> {
        self.content
            .as_widget_mut()
            .overlay(state, layout, renderer, translation)
    }

    fn size_hint(&self) -> Size<Length> {
        self.content.as_widget().size_hint()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        self.content.as_widget().state()
    }

    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        self.content.as_widget().tag()
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        // let new_viewport = Rectangle {
        //     x: self.offset.x + viewport.x,
        //     y: self.offset.y + viewport.y,
        //     width: viewport.width,
        //     height: viewport.height,
        // };

        self.content
            .as_widget()
            .draw(tree, renderer, theme, style, layout, cursor, viewport);
    }
}

impl<'a, Message, Theme, Renderer> From<Offset<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: 'a + iced::advanced::Renderer,
{
    fn from(offset: Offset<'a, Message, Theme, Renderer>) -> Self {
        Self::new(offset)
    }
}
