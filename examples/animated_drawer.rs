use iced::{
    advanced::Widget,
    widget::{button, column, container, horizontal_space, row, text, Space, Stack},
    Border, Color, Element,
    Length::{self, Fill},
    Padding, Point, Rectangle, Size, Subscription, Theme, Vector,
};
use iced_anim::{animation_builder, spring::Motion};

/// Some placeholder text to show within the drawer.
const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus commodo blandit posuere. Sed pharetra, lacus at pellentesque gravida, purus sem consequat lectus, vel venenatis justo ex ut nibh. Duis quis risus vitae libero volutpat fringilla vitae et magna. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Cras a malesuada nisl, ac scelerisque mauris. Praesent justo turpis, molestie sed dapibus id, mollis nec erat. Nam eu efficitur eros. Nullam condimentum neque at massa varius, ut interdum est sollicitudin. Etiam sit amet libero purus. In enim ipsum, congue in nulla sit amet, condimentum venenatis augue.

Sed bibendum lectus nec erat venenatis, eget suscipit sapien iaculis. Nunc in tellus id nisi maximus iaculis. Cras dignissim rutrum tristique. Integer molestie eros mi. Vestibulum consequat nulla mi, semper elementum lectus dictum eu. Pellentesque facilisis, dolor quis dictum luctus, lacus ipsum cursus nulla, vel laoreet ex enim eu turpis. Proin bibendum finibus tempus. Pellentesque dolor diam, ultricies quis interdum eget, posuere in magna. Curabitur vel congue est. In feugiat posuere dapibus. Morbi purus purus, blandit ut justo sit amet, convallis sagittis libero. Aliquam tempus nisi et nisi mattis, vitae vehicula massa facilisis.";

#[derive(Debug, Clone)]
enum Message {
    /// Increments the counter within the drawer.
    Increment,
    /// Opens or closes the drawer.
    ToggleDrawer,
    /// Keeps track of the current window size.
    WindowResized(Size),
}

struct State {
    /// A counter value shown within the drawer.
    count: usize,
    /// The current size of the window, which the drawer needs to be offset correctly.
    window_size: Size,
    /// Whether the drawer is currently open.
    is_drawer_open: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            count: 0,
            window_size: Size::new(1024.0, 768.0),
            is_drawer_open: false,
        }
    }
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.count += 1,
            Message::ToggleDrawer => {
                self.is_drawer_open = !self.is_drawer_open;
            }
            Message::WindowResized(size) => self.window_size = size,
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::event::listen_with(|event, _, _| match event {
            iced::Event::Window(iced::window::Event::Resized(size)) => {
                Some(Message::WindowResized(size))
            }
            _ => None,
        })
    }

    fn view(&self) -> Element<Message> {
        let drawer_button =
            container(button(text("Open Drawer")).on_press(Message::ToggleDrawer)).padding(8);
        drawer(
            self.is_drawer_open,
            self.count,
            self.window_size,
            drawer_button,
        )
    }
}

fn drawer<'a>(
    is_open: bool,
    count: usize,
    window_size: Size,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    // The amount of padding put around the outside of the outer drawer container.
    const PADDING: f32 = 8.0;
    // The width of the drawer when open
    const MAX_WIDTH: f32 = 350.0;
    let width = if is_open { MAX_WIDTH } else { 0.0 };

    // The underlay background color
    let background = if is_open {
        Color::from_rgba(0.0, 0.0, 0.0, 0.75)
    } else {
        Color::TRANSPARENT
    };

    let motion = Motion::SNAPPY;

    let drawer_stack = Stack::new()
        .width(Length::Fill)
        .height(Length::Fill)
        .push(content)
        .push(
            // Underlay
            animation_builder((background, width), move |(background, width)| {
                container(
                    button(container(Space::new(Length::Fill, Length::Fill)).center(Length::Fill))
                        .on_press_maybe(is_open.then_some(Message::ToggleDrawer))
                        .style(move |_, _| iced::widget::button::Style {
                            background: Some(background.into()),
                            ..Default::default()
                        }),
                )
                .padding(Padding::new(0.0).right(width + PADDING))
                .into()
            })
            .animation(motion)
            .animates_layout(true),
        )
        .push(
            // Drawer content
            animation_builder((background, width), move |(background, width)| {
                let offset_x = window_size.width - width - PADDING * width / MAX_WIDTH;
                Offset::new(
                    container(
                        container(drawer_content(count))
                            .style(move |theme: &Theme| iced::widget::container::Style {
                                background: Some(
                                    theme.extended_palette().background.base.color.into(),
                                ),
                                border: Border::default().rounded(8),
                                ..Default::default()
                            })
                            .padding(8)
                            .height(Fill)
                            .center_x(Length::Fixed(MAX_WIDTH)),
                    )
                    .padding(Padding::new(PADDING).left(0))
                    .style(move |_| iced::widget::container::Style {
                        background: Some(background.into()),
                        ..Default::default()
                    }),
                )
                .offset(Point::new(offset_x, 0.0))
                .into()
            })
            .animates_layout(true)
            .animation(motion),
        );

    container(drawer_stack)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

// A helper function to create a demo drawer content
fn drawer_content(count: usize) -> Element<'static, Message> {
    column![
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
        .align_y(iced::Alignment::Center)
        .spacing(8),
        column![
            button(text(format!("Increment count: {count}"))).on_press(Message::Increment),
            text(LOREM_IPSUM)
        ]
        .spacing(8)
    ]
    .spacing(12)
    .into()
}

pub fn main() -> iced::Result {
    iced::application("Animated Drawer", State::update, State::view)
        .subscription(State::subscription)
        .run()
}

/// A helper widget that offsets its content by a given amount.
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
        self.content.as_widget().size()
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
            .translate(Vector::new(self.offset.x, self.offset.y))
    }

    fn update(
        &mut self,
        state: &mut iced::advanced::widget::Tree,
        event: &iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) {
        self.content.as_widget_mut().update(
            state, event, layout, cursor, renderer, clipboard, shell, viewport,
        );
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
