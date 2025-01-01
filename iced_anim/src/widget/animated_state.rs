//! Helps manage animating values for widgets, such as animating a button's style. This can be used
//! to manage animations within a widget for anything, but is particularly useful for animating
//! style changes based on widget status.
//!
//! The [`AnimatedState`] can internally keep track of a widget's status and style, and then allow
//! updates to the animated style based on these values. This struct can be built into a widget's
//! internal state and used to manage style animations. It uses interior mutability to lazily
//! update the animated style when the widget is drawn, and requests redraws when the widget status
//! changes in response to some event.
//!
//! This requires that that your animated `Value` implement [`Animate`] - you can normally derive
//! this for simple structs.
//!
//! There are a few steps to using this struct if you want to animate a widget's style:
//!
//! 1. Implement `get_initial_status` and `get_status` functions for your widget. You'll use these
//!    to give the initial and current status to the [`AnimatedState`] so it can properly update
//!    the animated style. You don't _technically_ need dedicated functions for these, but it can
//!    help keep your Widget implementations cleaner. For example, a button might have an initial
//!    status fn look like this:
//!    ```no_run
//!    # struct Button { on_press: Option<fn()> }
//!    # enum Status { Active, Disabled }
//!    # impl Button {
//!
//!    fn get_initial_status(&self) -> Status {
//!        if self.on_press.is_some() {
//!            Status::Active
//!        } else {
//!            Status::Disabled
//!        }
//!    }
//!    # }
//!    ```
//!    For the current status, you can include any other fields that may be useful for determining
//!    the current status. For a button, this would be something like the internal widget state
//!    for tracking whether the button is pressed, and the cursor + layout for hover states.
//! 2. Add an [`Mode`] to the widget so users can change how the animation behaves.
//!    Then, add a builder function for updating it, e.g.
//!    ```no_run
//!    # use iced_anim::animated::Mode;
//!    # struct Button { mode: Mode }
//!    # impl Button {
//!    /// Sets the animation mode for this widget.
//!    pub fn animation(mut self, mode: impl Into<Mode>) -> Self {
//!        self.mode = mode.into();
//!        self
//!    }
//!    # }
//!    ```
//! 3. Add an `AnimatedState<Status, Style>` field to your widget's state. For example,
//!    a button state might look like this:
//!    ```no_run
//!    # use iced_anim::widget::AnimatedState;
//!    # type Status = iced::widget::button::Status;
//!    # type Style = iced::widget::button::Style;
//!    #[derive(Debug)]
//!    struct State {
//!        is_pressed: bool,
//!        animated_state: AnimatedState<Status, Style>,
//!    }
//!    ```
//! 4. Update [`iced::advanced::Widget::state`] to get the initial status, then pass that status
//!    and animation mode into [`AnimatedState::new`] to create the animated state.
//! 5. Update [`iced::advanced::Widget::diff`] to call call [`AnimatedState::diff`] if the mode
//!    has changed externally.
//!    ```ignore
//!    fn diff(&self, tree: &mut Tree) {
//!        // Diff the animated state with a potentially new animation mode.
//!        let state = tree.state.downcast_mut::<State>();
//!        state.animated_state.diff(self.animation);
//!        // Diff the rest of your widget state as necessary, e.g.
//!        tree.diff_children(std::slice::from_ref(&self.content));
//!    }
//!    ```
//! 6. Update [`iced::advanced::Widget::draw`] to get the current style from the animated state
//!    instead of manually calculating the style on each draw. Use [`AnimatedState::current_style`]
//!    and pass in a callback to generate the style based on the theme and status, and it'll return
//!    a reference to the latest animated style. The inner animated style will be updated if the
//!    closure produces a style different from the current target.
//! 7. Update [`iced::advanced::Widget::on_event`] to call [`AnimatedState::needs_redraw`] to
//!    determine if the widget needs to redraw. If a redraw is needed, then use the shell to
//!    request a redraw on the next frame. This may vary based on how your widget, but it will
//!    generally look like this:
//!    ```ignore
//!    # use iced::window;
//!    // Redraw anytime the status changes and would trigger a style change.
//!    let state = tree.state.downcast_mut::<State>();
//!    let status = self.get_status(&state, cursor, layout);
//!    let needs_redraw = state.animated_state.needs_redraw(status);
//!
//!    if needs_redraw {
//!        shell.request_redraw(window::RedrawRequest::NextFrame);
//!    }
//!    ```
//! 8. Finally, ensure your widget handles [`iced::window::Event::RedrawRequested`] events by
//!    calling [`AnimatedState::tick`] to update the animated value with the current time. This
//!    is how the animated state can update the value over time.
use std::{
    cell::{Ref, RefCell},
    time::Instant,
};

use crate::{animated::Mode, Animate, Animated};

/// Helps manage animating values for widgets.
///
/// This maintains the current animated value for a widget, which depends on the `status`.
/// - `Status`: The status of the widget which affects some `Value`, e.g. `button::Status`.
/// - `Value`: The value that will be animated, e.g. `button::Style`.
#[derive(Debug, Clone, PartialEq)]
pub struct AnimatedState<Status, Value> {
    /// The current status of the widget, which may affect the styling.
    status: Status,
    /// The animated value used by the widget, and is updated when the value changes.
    /// This is in a `RefCell` so that the value can be updated in the `draw` function,
    /// where we have access to the current theme. The cell may contain `None` until the
    /// first render, when the value is created.
    animated_value: RefCell<Option<Animated<Value>>>,
    /// The animation mode to use for the animated value.
    mode: Mode,
}

impl<Status, Value> AnimatedState<Status, Value>
where
    Status: PartialEq,
    Value: Animate,
{
    /// Creates a new [`AnimatedState`] with the given `status` and animation `mode`.
    pub fn new(status: Status, mode: Mode) -> Self {
        Self {
            status,
            animated_value: RefCell::new(None),
            mode,
        }
    }

    /// Returns the current status of the widget.
    pub fn status(&self) -> &Status {
        &self.status
    }

    /// Updates this animated state based on a potentially new `value` received by the widget.
    pub fn diff(&mut self, mode: Mode) {
        if self.mode != mode {
            self.mode = mode;
            let mut animated_value = self.animated_value.borrow_mut();
            if let Some(animation) = animated_value.as_mut() {
                animation.apply(mode);
            }
        }
    }

    /// Determines whether the widget needs to be redrawn based on events, updating the status and
    /// animated value as necessary. Generally called in a widget's `on_event` function.
    pub fn needs_redraw(&mut self, status: Status) -> bool {
        let animated_value = self.animated_value.borrow();
        if self.status != status {
            self.status = status;
            true
        } else if let Some(animated_value) = animated_value.as_ref() {
            animated_value.is_animating()
        } else {
            // No animated value yet.
            false
        }
    }

    /// Update the animated value with the current time.
    /// Call this for `RedrawRequested` events.
    pub fn tick(&mut self, now: Instant) {
        let mut animated_value = self.animated_value.borrow_mut();
        if let Some(animated_value) = animated_value.as_mut() {
            animated_value.tick(now);
        }
    }

    /// Causes the animation to immediately jump to the target value.
    pub fn settle(&mut self) {
        let mut animated_value = self.animated_value.borrow_mut();
        if let Some(animated_value) = animated_value.as_mut() {
            animated_value.settle();
        }
    }

    /// Gets a reference to the animated value, typically to use in a widget's `draw` function.
    ///
    /// The animation target will change if the `new_value` function returns a different value
    /// than the current target.
    pub fn current_value(&self, new_value: impl Fn(&Status) -> Value) -> Ref<'_, Value> {
        // Update the latest style if it has changed and indicate a redraw is needed.
        let new_value = new_value(&self.status);

        // Scoping the mutable borrow of the animated style.
        {
            let mut animated_value_ref = self.animated_value.borrow_mut();
            if let Some(animated_value) = animated_value_ref.as_mut() {
                if animated_value.target() != &new_value {
                    animated_value.set_target(new_value);
                }
            } else {
                // Create a new animated style if one doesn't exist.
                let animated_value = Animated::new(new_value.clone(), self.mode);
                animated_value_ref.replace(animated_value);
            }
        }

        Ref::map(self.animated_value.borrow(), |style| {
            style
                .as_ref()
                .expect("Animated style should have been lazily created")
                .value()
        })
    }
}
