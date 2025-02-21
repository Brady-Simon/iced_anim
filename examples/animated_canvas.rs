//! An example that shows how to draw animated shapes from within a canvas.
use iced::{
    border::Radius,
    mouse::{self, Cursor},
    widget::{
        canvas::{self, Frame, Geometry, Path},
        Canvas,
    },
    Color, Element,
    Length::Fill,
    Padding, Point, Rectangle, Renderer, Size, Theme,
};
use iced_anim::{spring::Motion, Animated, Animation, Event};
use std::time::Duration;

#[derive(Debug, Clone)]
enum Message {
    SelectIndex(usize),
    UpdateAnimatedSelection(Event<Rectangle>),
}

#[derive(Debug, Clone)]
struct AppState {
    /// The index of the user's currently hovered selection.
    hovered_index: usize,
    /// The animated part of the user's selection.
    animated_selection: Animated<Rectangle>,
    /// The collection of shapes drawn on screen.
    shapes: Vec<Rectangle>,
}

const PADDING: Padding = Padding::new(-4.0);
impl Default for AppState {
    fn default() -> Self {
        let shapes = vec![
            Rectangle::new(Point::ORIGIN, Size::new(256.0, 256.0)).expand(PADDING),
            Rectangle::new(Point::new(256.0, 0.0), Size::new(256.0, 256.0)).expand(PADDING),
            Rectangle::new(Point::new(512.0, 0.0), Size::new(256.0, 256.0)).expand(PADDING),
            Rectangle::new(Point::new(768.0, 0.0), Size::new(256.0, 256.0)).expand(PADDING),
            Rectangle::new(Point::new(0.0, 256.0), Size::new(512.0, 256.0)).expand(PADDING),
            Rectangle::new(Point::new(512.0, 256.0), Size::new(512.0, 256.0)).expand(PADDING),
            Rectangle::new(Point::new(0.0, 512.0), Size::new(341.0, 256.0)).expand(PADDING),
            Rectangle::new(Point::new(341.0, 512.0), Size::new(341.0, 256.0)).expand(PADDING),
            Rectangle::new(Point::new(682.0, 512.0), Size::new(342.0, 256.0)).expand(PADDING),
        ];
        Self {
            hovered_index: 0,
            animated_selection: Animated::spring(
                shapes[0],
                Motion::default().with_duration(Duration::from_millis(300)),
            ),
            shapes,
        }
    }
}

impl AppState {
    fn view(&self) -> Element<Message> {
        Animation::new(
            &self.animated_selection,
            Canvas::new(self).width(Fill).height(Fill),
        )
        .on_update(Message::UpdateAnimatedSelection)
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SelectIndex(index) => {
                // Update the hovered index, and also set the new target for the animation.
                self.hovered_index = index;
                self.animated_selection.set_target(self.shapes[index]);
            }
            Message::UpdateAnimatedSelection(event) => {
                self.animated_selection.update(event);
            }
        }
    }
}

impl canvas::Program<Message> for AppState {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let mut frame = Frame::new(renderer, bounds.size());

        // Fill in a white background
        frame.fill_rectangle(Point::ORIGIN, bounds.size(), Color::WHITE);

        // Fill in the background shapes
        for shape in &self.shapes {
            frame.fill(
                &Path::rounded_rectangle(shape.position(), shape.size(), Radius::new(8)),
                selected_color(*shape).scale_alpha(0.4),
            );
        }

        // Fill in the selected shape
        let selection = self.animated_selection.value();
        frame.fill(
            &Path::rounded_rectangle(selection.position(), selection.size(), Radius::new(8)),
            selected_color(*selection),
        );

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: &iced::Event,
        _bounds: Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        match event {
            canvas::Event::Mouse(mouse::Event::CursorMoved { position }) => {
                let Some(index) = self
                    .shapes
                    .iter()
                    .position(|shape| shape.contains(*position))
                else {
                    return None;
                };

                // Don't update the selection if it hasn't changed
                if index == self.hovered_index {
                    return None;
                }

                Some(Message::SelectIndex(index))
                    .map(canvas::Action::publish)
                    .map(canvas::Action::and_capture)
            }
            _ => None,
        }
    }
}

pub fn main() -> iced::Result {
    iced::application("Animated canvas", AppState::update, AppState::view)
        .window_size(Size::new(1024.0, 768.0))
        .run()
}

/// Makes the color of the selected shape change based on its position.
fn selected_color(rectangle: Rectangle) -> Color {
    let x = rectangle.x / 1024.0;
    let y = rectangle.y / 768.0;
    Color::from_rgb(x, y, 1.0 - x)
}
