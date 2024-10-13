//! Svg widgets display vector graphics in your application.
use super::AnimatedState;
use crate::SpringMotion;
use iced::advanced::{
    layout, renderer,
    widget::{tree, Tree},
};
use iced::{
    advanced::{svg, Layout, Widget},
    mouse::{self, Cursor},
    window, ContentFit, Element, Event, Length, Point, Rectangle, Rotation, Size, Vector,
};
use std::path::PathBuf;

// Re-export the widget types for convenience
pub use iced::widget::svg::{Catalog, Handle, Status, Style, StyleFn};

/// A vector graphics image.
///
/// An [`Svg`] image resizes smoothly without losing any quality.
///
/// [`Svg`] images can have a considerable rendering cost when resized,
/// specially when they are complex.
#[allow(missing_debug_implementations)]
pub struct Svg<'a, Theme = iced::Theme>
where
    Theme: Catalog,
{
    handle: Handle,
    width: Length,
    height: Length,
    content_fit: ContentFit,
    class: Theme::Class<'a>,
    rotation: Rotation,
    opacity: f32,
    motion: SpringMotion,
}

#[derive(Debug, Clone, PartialEq)]
struct State<Theme> {
    animated_state: AnimatedState<Status, Theme, Style>,
}

impl<'a, Theme> Svg<'a, Theme>
where
    Theme: Catalog,
{
    /// Creates a new [`Svg`] from the given [`Handle`].
    pub fn new(handle: impl Into<Handle>) -> Self {
        Self {
            handle: handle.into(),
            width: Length::Fill,
            height: Length::Shrink,
            content_fit: ContentFit::Contain,
            class: Theme::default(),
            rotation: Rotation::default(),
            opacity: 1.0,
            motion: SpringMotion::default(),
        }
    }

    /// Creates a new [`Svg`] that will display the contents of the file at the
    /// provided path.
    #[must_use]
    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        Self::new(Handle::from_path(path))
    }

    /// Sets the width of the [`Svg`].
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Svg`].
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the [`ContentFit`] of the [`Svg`].
    ///
    /// Defaults to [`ContentFit::Contain`]
    #[must_use]
    pub fn content_fit(self, content_fit: ContentFit) -> Self {
        Self {
            content_fit,
            ..self
        }
    }

    /// Sets the style of the [`Svg`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style class of the [`Svg`].
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    /// Applies the given [`Rotation`] to the [`Svg`].
    pub fn rotation(mut self, rotation: impl Into<Rotation>) -> Self {
        self.rotation = rotation.into();
        self
    }

    /// Sets the opacity of the [`Svg`].
    ///
    /// It should be in the [0.0, 1.0] range—`0.0` meaning completely transparent,
    /// and `1.0` meaning completely opaque.
    pub fn opacity(mut self, opacity: impl Into<f32>) -> Self {
        self.opacity = opacity.into();
        self
    }

    /// Sets the motion that will be used by animations.
    pub fn motion(mut self, motion: SpringMotion) -> Self {
        self.motion = motion;
        self
    }

    /// The initial status that this widget will have based on its properties.
    ///
    /// This will be used as the initial state value.
    fn get_initial_status(&self) -> Status {
        Status::Idle
    }

    /// Gets the status of the [`Svg`] based on the current [`State`].
    fn get_status(&self, cursor: Cursor, layout: Layout<'_>) -> Status {
        let is_mouse_over = cursor.is_over(layout.bounds());
        if is_mouse_over {
            Status::Hovered
        } else {
            Status::Idle
        }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Svg<'a, Theme>
where
    Renderer: iced::advanced::svg::Renderer,
    Theme: 'static + Catalog + Default + Clone + PartialEq,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State<Theme>>()
    }

    fn state(&self) -> tree::State {
        let theme = <Theme as Default>::default();
        let status = self.get_initial_status();
        let style = theme.style(&self.class, status);
        // Initialize the state with the current style.
        let state = State::<Theme> {
            animated_state: AnimatedState::new(status, theme, style).with_motion(self.motion),
        };

        tree::State::new(state)
    }

    fn diff(&self, tree: &mut Tree) {
        // If the style changes from outside, then immediately update the style.
        let state = tree.state.downcast_mut::<State<Theme>>();
        let new_style = state
            .animated_state
            .theme()
            .style(&self.class, *state.animated_state.status());
        state.animated_state.diff(self.motion, new_style);
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        // The raw w/h of the underlying image
        let Size { width, height } = renderer.measure_svg(&self.handle);
        let image_size = Size::new(width as f32, height as f32);

        // The rotated size of the svg
        let rotated_size = self.rotation.apply(image_size);

        // The size to be available to the widget prior to `Shrink`ing
        let raw_size = limits.resolve(self.width, self.height, rotated_size);

        // The uncropped size of the image when fit to the bounds above
        let full_size = self.content_fit.fit(rotated_size, raw_size);

        // Shrink the widget to fit the resized image, if requested
        let final_size = Size {
            width: match self.width {
                Length::Shrink => f32::min(raw_size.width, full_size.width),
                _ => raw_size.width,
            },
            height: match self.height {
                Length::Shrink => f32::min(raw_size.height, full_size.height),
                _ => raw_size.height,
            },
        };

        layout::Node::new(final_size)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let Size { width, height } = renderer.measure_svg(&self.handle);
        let image_size = Size::new(width as f32, height as f32);
        let rotated_size = self.rotation.apply(image_size);

        let bounds = layout.bounds();
        let adjusted_fit = self.content_fit.fit(rotated_size, bounds.size());
        let scale = Vector::new(
            adjusted_fit.width / rotated_size.width,
            adjusted_fit.height / rotated_size.height,
        );

        let final_size = image_size * scale;

        let position = match self.content_fit {
            ContentFit::None => Point::new(
                bounds.x + (rotated_size.width - adjusted_fit.width) / 2.0,
                bounds.y + (rotated_size.height - adjusted_fit.height) / 2.0,
            ),
            _ => Point::new(
                bounds.center_x() - final_size.width / 2.0,
                bounds.center_y() - final_size.height / 2.0,
            ),
        };

        let drawing_bounds = Rectangle::new(position, final_size);
        let state = tree.state.downcast_ref::<State<Theme>>();
        let style = state
            .animated_state
            .current_style(theme, |theme, status| theme.style(&self.class, *status));

        let render = |renderer: &mut Renderer| {
            renderer.draw_svg(
                svg::Svg {
                    handle: self.handle.clone(),
                    color: style.color,
                    rotation: self.rotation.radians(),
                    opacity: self.opacity,
                },
                drawing_bounds,
            );
        };

        if adjusted_fit.width > bounds.width || adjusted_fit.height > bounds.height {
            renderer.with_layer(bounds, render);
        } else {
            render(renderer);
        }
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: iced::Event,
        layout: Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        // Redraw anytime the status changes and would trigger a style change.
        let state = tree.state.downcast_mut::<State<Theme>>();
        let status = self.get_status(cursor, layout);
        let needs_redraw = state
            .animated_state
            .needs_redraw(status, |theme, status| theme.style(&self.class, *status));

        if needs_redraw {
            shell.request_redraw(window::RedrawRequest::NextFrame);
        }

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            state.animated_state.tick(now);
        }

        iced::event::Status::Ignored
    }
}

impl<'a, Message, Theme, Renderer> From<Svg<'a, Theme>> for Element<'a, Message, Theme, Renderer>
where
    Theme: 'static + Catalog + Default + Clone + PartialEq,
    Renderer: svg::Renderer + 'a,
{
    fn from(icon: Svg<'a, Theme>) -> Element<'a, Message, Theme, Renderer> {
        Element::new(icon)
    }
}

/// Creates a new [`Svg`] widget from the given [`Handle`].
///
/// Svg widgets display vector graphics in your application.
pub fn svg<'a, Theme>(handle: impl Into<Handle>) -> Svg<'a, Theme>
where
    Theme: 'static + Catalog + Default + Clone + PartialEq,
{
    Svg::new(handle)
}
