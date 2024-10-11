use std::{
    cell::{Ref, RefCell},
    time::Instant,
};

use crate::{Animate, Spring};

/// Helps manage animating styles for widgets.
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
    /// Whether the widget has been rendered yet. Necessary because we don't have access to the
    /// correct theme until the widget has been rendered since the `Theme` is only provided in the
    /// `draw` function.
    has_rendered: bool,
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
            has_rendered: false,
        }
    }

    pub fn theme(&self) -> Ref<Theme> {
        self.theme.borrow()
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    /// Updates this animated state based on a potentially new `style` received by the widget.
    pub fn diff(&mut self, style: Style) {
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
        if !self.has_rendered {
            // Request a redraw if the widget has not been rendered yet
            // to get the real theme from `draw`.
            self.has_rendered = true;
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
            self.theme.replace(theme.clone());
        }

        // Update the latest style if it has changed.
        let new_style = new_style(theme, &self.status);
        if new_style != *self.style.borrow() {
            self.style.replace(new_style.clone());
        }

        self.animated_style.value()
    }
}
