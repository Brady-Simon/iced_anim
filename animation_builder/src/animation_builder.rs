//! Implicitly animate between value changes.
use std::time::Instant;

use iced::{
    advanced::{
        graphics::core::event,
        layout,
        widget::{tree, Tree},
        Widget,
    },
    Element,
};

use crate::{animate::Animate, Spring, SpringMotion};

/// A widget that implicitly animates a value anytime it changes.
///
/// This is useful for animating changes to a widget's appearance or layout
/// based on some state change. Animations happen automatically when the value
/// changes, and the widget will use spring interpolation to animate the value
/// over a specified duration.
pub struct AnimationBuilder<'a, T, Message, Theme, Renderer>
where
    T: 'static + Animate,
{
    /// The function that builds the element using the animated value.
    builder: Box<dyn Fn(T) -> Element<'a, Message, Theme, Renderer> + 'a>,
    /// The spring that animates the value of this widget.
    spring: Spring<T>,
    /// Whether the layout will be affected by the animated value.
    animates_layout: bool,
    /// The cached element built using the most recent animated value and `builder`.
    cached_element: Element<'a, Message, Theme, Renderer>,
}

impl<'a, T, Message, Theme, Renderer> AnimationBuilder<'a, T, Message, Theme, Renderer>
where
    T: 'static + Animate,
{
    /// Creates a new `AnimationBuilder` with the given value and builder function.
    pub fn new(
        value: T,
        builder: impl Fn(T) -> Element<'a, Message, Theme, Renderer> + 'a,
    ) -> Self {
        let element = (builder)(value.clone());
        Self {
            builder: Box::new(builder),
            cached_element: element,
            spring: Spring::new(value),
            animates_layout: false,
        }
    }

    /// Defines the way the spring will animate the value.
    pub fn motion(mut self, motion: SpringMotion) -> Self {
        self.spring = self.spring.with_motion(motion);
        self
    }

    /// Indicates whether this widget should invalidate the application layout
    /// when animating between changes.
    ///
    /// This is set to `false` by default for performance reasons, but you may
    /// want to set this to `true` if you're animating a value that affects the
    /// layout of the widget (e.g. its size, text, position, etc).
    pub fn animates_layout(mut self, animates_layout: bool) -> Self {
        self.animates_layout = animates_layout;
        self
    }
}

impl<'a, T, Message, Theme, Renderer> From<AnimationBuilder<'a, T, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    T: 'static + Animate,
    Message: Clone + 'a,
    Theme: 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn from(animation: AnimationBuilder<'a, T, Message, Theme, Renderer>) -> Self {
        Self::new(animation)
    }
}

impl<'a, T, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for AnimationBuilder<'a, T, Message, Theme, Renderer>
where
    T: 'static + Animate,
    Renderer: iced::advanced::Renderer,
{
    fn size(&self) -> iced::Size<iced::Length> {
        self.cached_element.as_widget().size()
    }

    fn state(&self) -> tree::State {
        tree::State::new(self.spring.clone())
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Spring<T>>()
    }

    fn diff(&self, tree: &mut Tree) {
        // Update the spring's target if it has changed
        let state = tree.state.downcast_mut::<Spring<T>>();
        if state.target() != self.spring.value() {
            state.interrupt(self.spring.target().clone());
        }

        if state.motion() != self.spring.motion() {
            state.set_motion(self.spring.motion())
        }

        tree.diff_children(std::slice::from_ref(&self.cached_element));
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.cached_element
            .as_widget()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn operate(
        &self,
        state: &mut Tree,
        layout: layout::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced::advanced::widget::Operation<()>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.cached_element.as_widget().operate(
                &mut state.children[0],
                layout,
                renderer,
                operation,
            );
        })
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: layout::Layout<'_>,
        renderer: &Renderer,
        translation: iced::Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, Renderer>> {
        self.cached_element.as_widget_mut().overlay(
            &mut tree.children[0],
            layout,
            renderer,
            translation,
        )
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: layout::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
        renderer: &Renderer,
    ) -> iced::advanced::mouse::Interaction {
        self.cached_element.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.cached_element)]
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        self.cached_element.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        )
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) -> event::Status {
        let status = self.cached_element.as_widget_mut().on_event(
            &mut tree.children[0],
            event.clone(),
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        let now = Instant::now();

        // TODO: Figure out if there's a way to get `RedrawRequested` working.
        // It causes animation lag on the `animated_bubble` example due to constant interruptions
        // leading to a 0ms rebuild time instead of the usual ~8/16 ms for a single frame.
        let spring = tree.state.downcast_mut::<Spring<T>>();

        // Request a redraw if the spring has remaining energy
        if spring.has_energy() {
            shell.request_redraw(iced::window::RedrawRequest::NextFrame);
            // Only invalidate the layout if the user indicates to do so
            if self.animates_layout {
                shell.invalidate_layout();
            }

            // Update the animation and request a redraw
            spring.update(now);
            self.cached_element = (self.builder)(spring.value().clone());

            // TODO: Figure out why uncommenting this fixes the `nested_animations` example
            // but breaks the `preview_motion` example.
            // return self.cached_element.as_widget_mut().on_event(
            //     &mut tree.children[0],
            //     event,
            //     layout,
            //     cursor,
            //     renderer,
            //     clipboard,
            //     shell,
            //     viewport,
            // );
        }

        status
    }
}

/// A helper function to create an `AnimationBuilder` with a given value and builder function.
pub fn animation_builder<'a, T, Message, Theme, Renderer>(
    value: T,
    builder: impl Fn(T) -> Element<'a, Message, Theme, Renderer> + 'a,
) -> AnimationBuilder<'a, T, Message, Theme, Renderer>
where
    T: 'static + Animate,
{
    AnimationBuilder::new(value, builder)
}
