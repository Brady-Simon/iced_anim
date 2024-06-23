use std::time::{Duration, Instant};

use iced::advanced::{
    graphics::core::{event, Element},
    layout,
    widget::{tree, Tree},
    Widget,
};

use crate::{animate::Animate, animation::Animation, curve::Curve};

pub struct State<T>
where
    T: 'static + Animate + Clone + PartialEq,
{
    final_value: T,
    initial_value: T,
    current_value: T,
    duration: Duration,
    animation: Animation,
}

impl<T> State<T>
where
    T: 'static + Animate + Clone + PartialEq,
{
    pub fn new(value: T) -> Self {
        Self {
            final_value: value.clone(),
            initial_value: value.clone(),
            current_value: value,
            duration: Duration::from_millis(300),
            animation: Animation::Stopped,
        }
    }

    pub fn restart_with(&mut self, value: T) {
        self.final_value = value.clone();
        self.initial_value = self.current_value.clone();
        self.animation = Animation::Running {
            start: Instant::now(),
            duration: self.duration,
        };
    }
}

pub struct AnimationBuilder<'a, T, Message, Theme, Renderer>
where
    T: 'static + Animate + Clone + PartialEq,
{
    value: T,
    builder: Box<dyn Fn(T) -> iced::Element<'a, Message, Theme, Renderer> + 'a>,
    /// The curve to apply to the animation.
    curve: Curve,
    /// How long it takes to complete an animation between values.
    duration: Duration,
    /// Whether the layout will be affected by the animated value.
    animates_layout: bool,
    cached_element: iced::Element<'a, Message, Theme, Renderer>,
}

impl<'a, T, Message, Theme, Renderer> AnimationBuilder<'a, T, Message, Theme, Renderer>
where
    T: 'static + Animate + Clone + PartialEq,
{
    pub fn new(
        value: T,
        builder: impl Fn(T) -> iced::Element<'a, Message, Theme, Renderer> + 'a,
    ) -> Self {
        let element = (builder)(value.clone());
        Self {
            value,
            builder: Box::new(builder),
            cached_element: element,
            curve: Curve::default(),
            duration: Duration::from_millis(300),
            animates_layout: false,
        }
    }

    pub fn curve(mut self, curve: Curve) -> Self {
        self.curve = curve;
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Indicates whether this widget should invalidate the application layout
    /// when animating between changes.
    pub fn animates_layout(mut self, animates_layout: bool) -> Self {
        self.animates_layout = animates_layout;
        self
    }
}

impl<'a, T, Message, Theme, Renderer> From<AnimationBuilder<'a, T, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    T: 'static + Animate + Clone + PartialEq,
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
    T: 'static + Animate + Clone + PartialEq,
    Renderer: iced::advanced::Renderer,
{
    fn size(&self) -> iced::Size<iced::Length> {
        self.cached_element.as_widget().size()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new(self.value.clone()))
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State<T>>()
    }

    fn diff(&self, tree: &mut Tree) {
        let state = tree.state.downcast_mut::<State<T>>();

        if state.final_value != self.value {
            // Value from the outside changed, adjust the target value
            state.restart_with(self.value.clone());
        }

        if state.duration != self.duration {
            state.duration = self.duration;
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
        tree: &mut Tree,
        layout: layout::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced::advanced::widget::Operation<Message>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.cached_element.as_widget().operate(
                &mut tree.children[0],
                layout,
                renderer,
                operation,
            );
        });
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
        if let event::Status::Captured = self.cached_element.as_widget_mut().on_event(
            &mut tree.children[0],
            event.clone(),
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        ) {
            return event::Status::Captured;
        }

        let state = tree.state.downcast_mut::<State<T>>();

        // Request a redraw if the current value doesn't match the final value.
        if state.current_value != state.final_value {
            shell.request_redraw(iced::window::RedrawRequest::NextFrame);
            if self.animates_layout {
                shell.invalidate_layout();
            }
        }

        if let iced::Event::Window(_, iced::window::Event::RedrawRequested(now)) = event {
            if state.current_value == state.final_value {
                state.animation.stop();
                state.initial_value = state.final_value.clone();
                return event::Status::Ignored;
            }

            // Instantiate the start of the animation if it isn't present yet
            if state.animation == Animation::Stopped {
                state.animation = Animation::Running {
                    start: now,
                    duration: state.duration,
                };
            };

            // Update the animation and request a redraw
            state.animation.update(now);
            let progress = state.animation.progress(now);
            let new_value = Animate::animate(
                &state.initial_value,
                &state.final_value,
                progress,
                self.curve,
            );
            state.current_value = new_value;
            self.cached_element = (self.builder)(state.current_value.clone());
        }

        event::Status::Ignored
    }
}
