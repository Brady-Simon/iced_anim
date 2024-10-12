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
