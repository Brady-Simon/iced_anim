//! Helps manage animating styles for widgets.
//!
//! The [`AnimatedState`] internally keeps track of a widget's status and style, and then
//! updates the animated style based on these values. This struct can be built into a widget's
//! internal state and used to manage style animations. It uses interior mutability to lazily
//! update the animated style when the widget is drawn, and requests redraws when the widget status
//! changes in response to some event.
//!
//! This requires that that your `Style` implement [`Animate`] - you can normally derive this for
//! simple structs.
//!
//! There are a few steps to using this struct:
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
//! 2. Add a [`SpringMotion`] field to your widget so users can change how the animation behaves.
//!    Then, add a builder function for updating it, e.g.
//!    ```no_run
//!    # use iced_anim::SpringMotion;
//!    # struct Button { motion: SpringMotion }
//!    # impl Button {
//!    /// Sets the motion that will be used by animations.
//!    pub fn motion(mut self, motion: SpringMotion) -> Self {
//!        self.motion = motion;
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
//!    and motion into [`AnimatedState::new`] to create the animated state.
//! 5. Update [`iced::advanced::Widget::diff`] to call call [`AnimatedState::diff`] if the motion
//!    has changed externally.
//!    ```ignore
//!    fn diff(&self, tree: &mut Tree) {
//!        // Diff the animated state with a potentially new motion.
//!        let state = tree.state.downcast_mut::<State>();
//!        state.animated_state.diff(self.motion);
//!        // Diff the rest of your widget state as necessary.
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
//!    calling [`AnimatedState::tick`] to update the animated style with the current time. This
//!    is how the animated state can update the style over time.
use std::{
    cell::{Ref, RefCell},
    time::Instant,
};

use crate::{Animate, Spring, SpringMotion};

/// Helps manage animating styles for widgets.
///
/// This maintains the current animated style for a widget, which depends on the `status`.
#[derive(Debug, Clone, PartialEq)]
pub struct AnimatedState<Status, Style> {
    /// The current status of the widget, which may affect the styling.
    status: Status,
    /// The animated style used by the widget, and is updated when the style changes.
    /// This is in a `RefCell` so that the style can be updated in the `draw` function,
    /// where we have access to the current theme. The cell may contain `None` until the
    /// first render, when the style is created.
    animated_style: RefCell<Option<Spring<Style>>>,
    /// The motion used by the animated style.
    motion: SpringMotion,
}

impl<Status, Style> AnimatedState<Status, Style>
where
    Status: PartialEq,
    Style: Animate + Clone + PartialEq,
{
    pub fn new(status: Status, motion: SpringMotion) -> Self {
        Self {
            status,
            animated_style: RefCell::new(None),
            motion,
        }
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    /// Updates this animated state based on a potentially new `style` received by the widget.
    pub fn diff(&mut self, motion: SpringMotion) {
        if self.motion != motion {
            self.motion = motion;
            let mut animated_style = self.animated_style.borrow_mut();
            if let Some(style) = animated_style.as_mut() {
                style.set_motion(motion);
            }
        }
    }

    /// Determines whether the widget needs to be redrawn based on events, updating the status and
    /// animated style as necessary. Generally called in a widget's `on_event` function.
    pub fn needs_redraw(&mut self, status: Status) -> bool {
        let animated_style = self.animated_style.borrow();
        if self.status != status {
            self.status = status;
            true
        } else if let Some(animated_style) = animated_style.as_ref() {
            animated_style.has_energy()
        } else {
            // No animated style yet.
            false
        }
    }

    /// Update the animated style with the current time.
    /// Call this for `RedrawRequested` events.
    pub fn tick(&mut self, now: Instant) {
        let mut animated_style = self.animated_style.borrow_mut();
        if let Some(animated_style) = animated_style.as_mut() {
            animated_style.tick(now);
        }
    }

    /// Gets a reference to the animated style to use in a widget's `draw` function,
    /// using interior mutability to update the animation as necessary.
    ///
    /// The animation target will change if the `new_style` function returns a different style
    /// than the current target.
    pub fn current_style(&self, new_style: impl Fn(&Status) -> Style) -> Ref<'_, Style> {
        // Update the latest style if it has changed and indicate a redraw is needed.
        let new_style = new_style(&self.status);

        // Scoping the mutable borrow of the animated style.
        {
            let mut animated_style_ref = self.animated_style.borrow_mut();
            if let Some(animated_style) = animated_style_ref.as_mut() {
                if animated_style.target() != &new_style {
                    animated_style.interrupt(new_style);
                }
            } else {
                // Create a new animated style if one doesn't exist.
                let animated_style = Spring::new(new_style.clone())
                    .with_motion(self.motion)
                    .with_target(new_style);
                animated_style_ref.replace(animated_style);
            }
        }

        Ref::map(self.animated_style.borrow(), |style| {
            style
                .as_ref()
                .expect("Animated style should have been lazily created")
                .value()
        })
    }
}
