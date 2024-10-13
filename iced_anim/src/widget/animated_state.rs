//! Helps manage animating styles for widgets.
//!
//! The [`AnimatedState`] internally keeps track of a widget's status, theme, and style, and then
//! updates the animated style based on these values. This struct can be built into a widget's
//! internal state and used to manage style animations.
//!
//! There are a few steps to using this struct:
//!
//! 1. Implement `get_initial_status` and `get_status` functions for your widget. You'll use these
//!    to give the initial and current status to the [`AnimatedState`] so it can properly update
//!    the animated style. For example, a button might have an initial status fn look like this:
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
//! 3. Add an `AnimatedState<Status, Theme, Style>` field to your widget's state, including the
//!    theme as a generic type. For example, a button state might look like this:
//!    ```no_run
//!    # use iced_anim::widget::AnimatedState;
//!    # type Status = iced::widget::button::Status;
//!    # type Style = iced::widget::button::Style;
//!    #[derive(Debug, Clone, PartialEq)]
//!    struct State<Theme> {
//!        is_pressed: bool,
//!        animated_state: AnimatedState<Status, Theme, Style>,
//!    }
//!    ```
//! 4. Update any other references to `State` to use the correct generics, like `State<Theme>`.
//!    Also make sure to include the following bounds on the `Theme` generic type:
//!    - 'static + Catalog + Default + Clone + PartialEq
//!
//!    We need `'static` because `Widget::tag` requires a static type, and unfortunately for now
//!    we must store the theme in the [`AnimatedState`] because we can't access it in `on_event`
//!    where we would want to update the animated style. We need `Default + Clone + PartialEq` to
//!    set a default value on first render, clone the theme when updating the animated style, and
//!    compare the theme for updates.
//!
//!    > Note: in the future, I hope that we can remove the extra theme field and bounds by having
//!    > access to the current theme in `on_event`, where we have mutable access to widget state.
//!    > but for now, we have to track the theme in the [`AnimatedState`] and update it in `draw`.
//! 5. Update [`iced::advanced::Widget::state`] to get the default theme and initial status, then
//!    build your initial style from them. Then you can pass those into [`AnimatedState::new`] with
//!    the [`AnimatedState::with_motion`] create a new instance for your widget state.
//! 6. Update [`iced::advanced::Widget::diff`] to create a new style based on the theme and status,
//!    then call [`AnimatedState::diff`] to update the motion and animated style if necessary.
//!    ```ignore
//!    fn diff(&self, tree: &mut Tree) {
//!        // Diff the animated state with a potentially new style.
//!        let state = tree.state.downcast_mut::<State<Theme>>();
//!        let new_style = state
//!            .animated_state
//!            .theme()
//!            .style(&self.class, state.animated_state.status().clone());
//!        state.animated_state.diff(self.motion, new_style);
//!        // Diff the rest of your widget state as necessary.
//!        tree.diff_children(std::slice::from_ref(&self.content));
//!    }
//!    ```
//! 7. Update [`iced::advanced::Widget::draw`] to get the current style from the animated state
//!    instead of manually calculating the style on each draw. Use [`AnimatedState::current_style`]
//!    and pass in the current theme and a callback to generate the style based on the theme and
//!    status, and it'll return a reference to the latest animated style.
//! 8. Update [`iced::advanced::Widget::on_event`] to call [`AnimatedState::needs_redraw`] to
//!    determine if the widget needs to redraw. If a redraw is needed, then use the shell to
//!    request a redraw on the next frame. This may vary based on your widget, but it will
//!    generally look like this:
//!    ```ignore
//!    # use iced::window;
//!    // Redraw anytime the status changes and would trigger a style change.
//!    let state = tree.state.downcast_mut::<State<Theme>>();
//!    let status = self.get_status(&state, cursor, layout);
//!    let needs_redraw = state
//!        .animated_state
//!        .needs_redraw(status, |theme, status| theme.style(&self.class, *status));
//!
//!    if needs_redraw {
//!        shell.request_redraw(window::RedrawRequest::NextFrame);
//!    }
//!    ```
//! 9. Finally, ensure your widget handles [`iced::window::Event::RedrawRequested`] events by
//!    calling [`AnimatedState::tick`] to update the animated style with the current time. This
//!    is how the animated state can update the style over time.
use std::{cell::RefCell, time::Instant};

use crate::{Animate, Spring, SpringMotion};

/// Helps manage animating styles for widgets.
///
/// This maintains the current animated style value for a widget, which updates based on the theme
/// and widget status.
#[derive(Debug, Clone, PartialEq)]
pub struct AnimatedState<Status, Style> {
    /// The current status of the widget, which may affect the styling.
    status: Status,
    style: RefCell<Option<Style>>,
    /// The animated style used by the widget, and is updated when the style or theme changes.
    animated_style: RefCell<Option<Spring<Style>>>,
    /// Whether the widget needs to be redrawn and have its animation target updated.
    rebuild: RefCell<Option<Rebuild>>,
    motion: SpringMotion,
}

/// How the animated theme should be updated.
///
/// Theme transitions are handled by immediately settling the animated style to the new style.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Rebuild {
    /// Causes the animated style to immediately settle at a new style.
    Settle,
    /// Changes the animated style's target to a new style.
    Interrupt,
}

impl<Status, Style> AnimatedState<Status, Style>
where
    Status: PartialEq,
    Style: Animate + Clone + PartialEq,
{
    pub fn new(status: Status, motion: SpringMotion) -> Self {
        Self {
            status,
            style: RefCell::new(None),
            animated_style: RefCell::new(None),
            rebuild: RefCell::new(None),
            motion,
        }
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    /// Sets the motion of the animated style.
    pub fn with_motion(mut self, motion: SpringMotion) -> Self {
        self.motion = motion;
        {
            let mut animated_style = self.animated_style.borrow_mut();
            if let Some(style) = animated_style.as_mut() {
                style.set_motion(motion);
            }
        }
        self
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

        // if *self.style.borrow() != style {
        //     self.animated_style.interrupt(style.clone());
        //     self.style.replace(style);
        // }
    }

    /// Determines whether the widget needs to be redrawn based on events, updating the status and
    /// animated style as necessary. Generally called in a widget's `on_event` function.
    pub fn needs_redraw(&mut self, status: Status) -> bool {
        let animated_style = self.animated_style.borrow();
        if self.status != status {
            println!("Status changed");
            self.status = status;
            // self.rebuild.replace(Some(Rebuild::Interrupt));
            true
        } else if let Some(animated_style) = animated_style.as_ref() {
            println!("Has energy: {}", animated_style.has_energy());
            animated_style.has_energy()
        } else {
            // No animated style yet.
            println!("No need to redraw");
            false
        }
    }

    /// Update the animated style with the current time.
    /// Call this for `RedrawRequested` events.
    pub fn tick(&mut self, now: Instant) {
        let mut animated_style = self.animated_style.borrow_mut();
        if let Some(animated_style) = animated_style.as_mut() {
            println!("Tick");
            animated_style.tick(now);
        }
    }

    /// Gets the current style to use in a widget's `draw` function, using interior mutability to
    /// update the style and theme as necessary.
    pub fn current_style(&self, new_style: impl Fn(&Status) -> Style) -> Style {
        // Update the latest style if it has changed and indicate a redraw is needed.
        let new_style = new_style(&self.status);

        let mut animated_style_ref = self.animated_style.borrow_mut();
        if let Some(animated_style) = animated_style_ref.as_mut() {
            if *animated_style.target() != new_style {
                println!("Interrupting with new style");
                animated_style.interrupt(new_style);
            } else {
                // Target style matches new style already.
                println!("Target style matches new style");
            }
            animated_style.value().clone()
        } else {
            // Create a new animated style if one doesn't exist.
            println!("Style doesn't exist yet");
            let animated_style = Spring::new(new_style.clone())
                .with_motion(self.motion)
                .with_target(new_style.clone());
            animated_style_ref.replace(animated_style);
            new_style
        }
    }
}
