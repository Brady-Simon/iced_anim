//! A widget that helps you animate a value over time from your state.
//!
//! The main difference between this widget and the `AnimationBuilder` is that
//! this widget allows you to directly change the value stored in your state
//! versus passively animating a value. This may also compose better with other
//! animations due to some limitations around widget-driven animations instead
//! of state-driven animations. Refer to the `AnimationBuilder` docs for more
//! details.
//!
//! # Example
//! ```rust
//! use iced::widget::{button, text, Row};
//! use iced_anim::{Animation, Spring, SpringEvent};
//!
//! #[derive(Default)]
//! struct State {
//!     size: Spring<f32>,
//! }
//!
//! #[derive(Clone)]
//! enum Message {
//!     UpdateSize(SpringEvent<f32>),
//! }
//!
//! impl State {
//!     fn update(&mut self, message: Message) {
//!         match message {
//!             Message::UpdateSize(event) => self.size.update(event),
//!         }
//!     }
//!
//!     fn view(&self) -> iced::Element<Message> {
//!         Row::new()
//!             .push(
//!                 button(text("Change target"))
//!                     .on_press(Message::UpdateSize((self.size.value() + 50.0).into()))
//!             )
//!             .push(
//!                 Animation::new(&self.size, text(self.size.value().to_string()))
//!                     .on_update(Message::UpdateSize)
//!             )
//!            .into()
//!     }
//! }
//! ```
use crate::{Animate, Spring, SpringEvent};
use iced_core::{event, widget::Tree, Element, Length, Size, Widget};
use std::time::Instant;

/// A widget that helps you animate a value over time from your state.
/// This is useful for animating changes to a widget's appearance or layout
/// where you want to directly change the value stored in your state versus
/// passively animating a value like the `AnimationBuilder`.
pub struct Animation<'a, T: Animate, Message, Theme, Renderer> {
    /// The spring that controls the animated value.
    spring: &'a Spring<T>,
    /// The content that will respond to the animation.
    content: Element<'a, Message, Theme, Renderer>,
    /// The function that will be called when the spring needs to be updated.
    on_update: Option<Box<dyn Fn(SpringEvent<T>) -> Message>>,
    /// Whether animations are disabled, in which case the value will be updated
    /// immediately without animating. Useful for reduced motion preferences.
    is_disabled: bool,
}

impl<'a, T, Message, Theme, Renderer> Animation<'a, T, Message, Theme, Renderer>
where
    T: 'static + Animate,
    Message: 'a + Clone,
{
    /// Creates a new `Animation` with the given `spring` and `content`.
    pub fn new(
        spring: &'a Spring<T>,
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self {
            spring,
            content: content.into(),
            on_update: None,
            is_disabled: false,
        }
    }

    /// Sets the function that will be called when the spring needs to be updated.
    pub fn on_update<F>(mut self, build_message: F) -> Self
    where
        F: Fn(SpringEvent<T>) -> Message + 'static,
    {
        self.on_update = Some(Box::new(build_message));
        self
    }

    /// Whether to disable animations and update the value immediately.
    /// Useful for reduced motion preferences.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

impl<'a, T, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Animation<'a, T, Message, Theme, Renderer>
where
    T: 'static + Animate,
    Message: 'a + Clone,
    Renderer: 'a + iced_core::Renderer,
{
    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.content.as_widget().size_hint()
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        viewport: &iced_core::Rectangle,
        renderer: &Renderer,
    ) -> iced_core::mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn operate(
        &self,
        state: &mut iced_core::widget::Tree,
        layout: iced_core::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced_core::widget::Operation<()>,
    ) {
        self.content
            .as_widget()
            .operate(&mut state.children[0], layout, renderer, operation);
    }

    fn state(&self) -> iced_core::widget::tree::State {
        iced_core::widget::tree::State::None
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut iced_core::widget::Tree,
        layout: iced_core::Layout<'_>,
        renderer: &Renderer,
        translation: iced_core::Vector,
    ) -> Option<iced_core::overlay::Element<'b, Message, Theme, Renderer>> {
        self.content
            .as_widget_mut()
            .overlay(&mut tree.children[0], layout, renderer, translation)
    }

    fn layout(
        &self,
        tree: &mut iced_core::widget::Tree,
        renderer: &Renderer,
        limits: &iced_core::layout::Limits,
    ) -> iced_core::layout::Node {
        self.content
            .as_widget()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn draw(
        &self,
        tree: &iced_core::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced_core::renderer::Style,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        viewport: &iced_core::Rectangle,
    ) {
        self.content.as_widget().draw(
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
        tree: &mut iced_core::widget::Tree,
        event: iced_core::Event,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced_core::Clipboard,
        shell: &mut iced_core::Shell<'_, Message>,
        viewport: &iced_core::Rectangle,
    ) -> event::Status {
        let status = self.content.as_widget_mut().on_event(
            &mut tree.children[0],
            event.clone(),
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        if !self.spring.has_energy() {
            return status;
        }

        if let Some(on_update) = &self.on_update {
            let event: SpringEvent<T> = if self.is_disabled {
                SpringEvent::Settle
            } else {
                let now = Instant::now();
                SpringEvent::Tick(now)
            };
            shell.publish(on_update(event));
        }

        status
    }
}

impl<'a, T, Message, Theme, Renderer> From<Animation<'a, T, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    T: 'static + Animate,
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: iced_core::Renderer + 'a,
{
    fn from(animation: Animation<'a, T, Message, Theme, Renderer>) -> Self {
        Self::new(animation)
    }
}
