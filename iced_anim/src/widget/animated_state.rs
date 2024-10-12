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
use std::{
    cell::{Ref, RefCell},
    time::Instant,
};

use crate::{Animate, Spring, SpringMotion};

/// Helps manage animating styles for widgets.
///
/// This maintains the current animated style value for a widget, which updates based on the theme
/// and widget status.
#[derive(Debug, Clone, PartialEq)]
pub struct AnimatedState<Status, Theme, Style> {
    /// The current status of the widget, which may affect the styling.
    status: Status,
    /// The current theme of the widget. Used to update the style and tracked to ensure the styling
    /// is always up-to-date. This is a `RefCell` because we only have access to the theme in
    /// `draw`, but we have update the animated style in `on_event`, which doesn't have a `theme`.
    theme: RefCell<Theme>,
    style: RefCell<Style>,
    /// The animated style used by the widget, and is updated when the style or theme changes.
    animated_style: Spring<Style>,
    /// Whether the widget needs to be redrawn and have its animation target updated.
    rebuild: RefCell<Option<Rebuild>>,
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

impl<Status, Theme, Style> AnimatedState<Status, Theme, Style>
where
    Status: PartialEq,
    Style: Animate + Clone + PartialEq,
    Theme: Clone + PartialEq,
{
    pub fn new(status: Status, theme: Theme, style: Style) -> Self {
        let animated_style = Spring::new(style.clone());
        Self {
            status,
            theme: RefCell::new(theme),
            style: RefCell::new(style),
            animated_style,
            rebuild: RefCell::new(None),
        }
    }

    pub fn theme(&self) -> Ref<Theme> {
        self.theme.borrow()
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    /// Sets the motion of the animated style.
    pub fn with_motion(mut self, motion: SpringMotion) -> Self {
        self.animated_style.set_motion(motion);
        self
    }

    /// Updates this animated state based on a potentially new `style` received by the widget.
    pub fn diff(&mut self, motion: SpringMotion, style: Style) {
        if self.animated_style.motion() != motion {
            self.animated_style.set_motion(motion);
        }

        if *self.style.borrow() != style {
            self.animated_style.interrupt(style.clone());
            self.style.replace(style);
        }
    }

    /// Determines whether the widget needs to be redrawn based on events, updating the status and
    /// animated style as necessary. Generally called in a widget's `on_event` function.
    pub fn needs_redraw(
        &mut self,
        status: Status,
        new_style: impl Fn(&Theme, &Status) -> Style,
    ) -> bool {
        let rebuild = self.rebuild.borrow().clone();
        if let Some(rebuild) = rebuild {
            self.rebuild.replace(None);
            let theme = self.theme.borrow().clone();
            match rebuild {
                Rebuild::Interrupt => {
                    self.animated_style
                        .interrupt(new_style(&theme, &self.status));
                }
                Rebuild::Settle => {
                    self.animated_style
                        .settle_at(new_style(&theme, &self.status));
                }
            }
            true
        } else if self.status != status {
            // Status changes means the style is likely changing.
            self.status = status;
            self.animated_style
                .interrupt(new_style(&self.theme.borrow(), &self.status));
            true
        } else if self.animated_style.has_energy() {
            // Request a redraw the spring still has energy remaining.
            true
        } else {
            false
        }
    }

    /// Update the animated style with the current time.
    /// Call this for `RedrawRequested` events.
    pub fn tick(&mut self, now: Instant) {
        self.animated_style.tick(now);
    }

    /// Gets the current style to use in a widget's `draw` function, using interior mutability to
    /// update the style and theme as necessary.
    pub fn current_style(
        &self,
        theme: &Theme,
        new_style: impl Fn(&Theme, &Status) -> Style,
    ) -> &Style {
        // Ensure the theme is always up-to-date.
        if *self.theme.borrow() != *theme {
            self.rebuild.replace(Some(Rebuild::Settle));
            self.theme.replace(theme.clone());
        }

        // Update the latest style if it has changed and indicate a redraw is needed.
        let new_style = new_style(theme, &self.status);
        if new_style != *self.style.borrow() {
            if self.rebuild.borrow().is_none() {
                self.rebuild.replace(Some(Rebuild::Interrupt));
            }
            self.style.replace(new_style.clone());
        }

        self.animated_style.value()
    }
}
