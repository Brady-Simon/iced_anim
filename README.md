# Iced Animations

This package is designed to make it easy to animate between values using 
[the Iced framework](https://github.com/iced-rs/iced).

## Overview

Animations are powered by the `Animate` trait. You can pick between spring or
transition animations. Controlled animations using the `Animation` widget have
a few parts:

- Storing an `Animated<T>` value in your app state
- Including an update message, e.g. `Message::UpdateSize(iced_anim::Event<T>)`
- Piping animation events to your animated value
- Wrapping the UI you want to animate in an `Animation` widget

Let's follow an example where you have a `size` in your app state and want to
animate changes to it:

```rust
use iced_anim::{Animated, Event};

#[derive(Default)]
struct State {
    // Your animated value, wrapped in an `Animated` type.
    size: Animated<f32>,
}

#[derive(Clone)]
enum Message {
    // A message that lets you (and animation widgets) update the size.
    UpdateSize(Event<f32>),
}

```rust
impl State {
    fn update(&mut self, message: Message) {
        match message {
            // Let your animated value process the event
            Message::UpdateSize(event) => self.size.update(event),
        }
    }
}
```

Next, let's add a default impl using an ease transition that sets the initial
size at 50px:

```rust
use iced_anim::transition::Easing;

impl Default for State {
    fn default() -> Self {
        Self {
            size: Animated::transition(50.0, Easing::EASE),
        }
    }
}
```

Or, if you're feeling spicy, you can use a spring animation:

```rust
use iced_anim::spring::Motion;

impl Default for State {
    fn default() -> Self {
        Self {
            size: Animated::spring(50.0, Motion::BOUNCY),
        }
    }
}
```

Next, you need to use your animated size in a view. You can use the `Animation`
widget to make your UI animate when the `size` changes. You'll also want a way
to change the size so you can see the animation, so let's add some buttons too.

```rust
use iced::{
    widget::{button, column, container, row, text},
    Color, Element, Length,
};
use iced_anim::{animation::animation, transition::Easing};

impl State {
    pub fn view(&self) -> Element<Message> {
        let buttons = row![
            button(text("-50")).on_press(Message::UpdateSize(Event::Target(self.size.target() - 50.0))),
            button(text("+50")).on_press(Message::UpdateSize(Event::Target(self.size.target() + 50.0))),
        ]
        .spacing(8);

        let animated_box = animation(
            &self.size,
            container(text(*self.size.value() as isize))
                .style(|_| container::background(Color::from_rgb(1.0, 0.0, 0.0)))
                .center(*self.size.value()),
        )
        .on_update(Message::UpdateSize);

        column![buttons, animated_box]
            .spacing(8)
            .padding(8)
            .width(Length::Shrink)
            .into()
    }
}
```

Or, if you don't want to type of `Event::Target`, you can replace it with 
`.into()` since the `From` impl for `Event` is the `Event::Target` case.

```rust
let buttons = row![
    button(text("-50")).on_press(Message::UpdateSize((self.size.target() - 50.0).into())),
    button(text("+50")).on_press(Message::UpdateSize((self.size.target() + 50.0).into())),
]
.spacing(8);
```

The `Animation` widget works by watching your animated `size` and sending
updates through the message you pass to `.on_update` to trigger rebuilds.
Anytime you set a new target value for your animation, the `Animation` widget
will pick up the change and emit messages to update the animated value.

In the above example, you can set a new target by emitting the message we
created, e.g. `Message::UpdateSize(150.0.into())`. If you need to access the
current animated value or the target value (the value being animated towards),
you can use the `.value()` and `.target()` functions respectively.

If you have a very simple value you'd like to animate but don't want to store
in your app state, then consider the `AnimationBuilder` widget.

### `AnimationBuilder` widget

The `AnimationBuilder` widget lets you have uncontrolled animations that takes
some sort of value that implements `Animate` and a closure to build out a UI
based on the current interpolated value. The benefit is that you don't have to
emit messages for very simple animations but has a couple limitations mentioned
below. Typical usage might look something like this, where `self.size` is an 
`f32` in your app state:

```rust
AnimationBuilder::new(self.size, |size| {
    container(text(size as isize))
        .center(size)
        .into()
})
.animates_layout(true)
```

> NOTE: `.animates_layout(true)` is what allows re-rendering certain properties
like the app layout or text. This is off by default to avoid unnecessarily
invalidating the app layout, but will be necessary in certain situations.

The UI will automatically re-render when `self.size` is changed. The closure
provides the current animated value and will be called to generate the next
view as appropriate. This means you don't need to store an `Animated` value in
your app state or have a message dedicated to updating it. To see more for this
particular example, refer to the `animated_size` example.

#### Limitations

It might not be easy/possible to pass in non-clonable content like custom
elements to `AnimationBuilder`'s closure to due the closure being having to be
invoked multiple times to animate between values. Making reusable functions
that use this widget and also take a generic element might be difficult.

Nested animations also don't work if both properties are actively being 
animated. One animation at a time will function correctly, but trying to adjust
both at the same time leads to the inner property skipping to the final value.
Use the `Animation` widget if you need any of these properties.

### Should I use `Animation` or `AnimationBuilder`?

Generally, if you're animating a tiny value that might not be directly within
your state and the element won't contain any nested animated values, then use
`AnimationBuilder`. Otherwise, use the state-driven `Animation` to avoid the
limitations of widget-driven animations. Also, use `Animation` anytime you want
your state to contain the animated value.

### Should I use a `Spring` or a `Transition`?

It's largely personal preference, but springs are typically better at 
interactive animations because they're momentum-based and keep their existing
speed even when the target changes. This makes animations whose target value is
always changing to appear much more fluid. See the `animated_bubble` example
for a constantly-changing animation target.

If your animation isn't going to be constantly changing or it's something minor
like a color, then you can use a transition with an easing curve.

## Animated widgets

A subset of the standard `iced` widgets are exported under a `widgets` feature
flag. You can use these as drop-in replacements for the existing widgets and
control the animation mode via the `.animation()` builder function on them:

```rust
use iced::widget::text;
use iced_anim::widget::button;

let my_button = button(text("Animated button"))
    .on_press(Message::DoSomething);
```

## Types that implement `Animate`

Several types implement `Animate` by default, such as `f32`, `iced::Color`,
and `iced::Theme`, with support for others being added in the future. You can
also derive `Animate` on your own structs if you enable the `derive` feature
and all inner properties already implement `Animate`:

```rust
use iced_anim::Animate;

#[derive(Animate)]
struct CustomRectangle {
    width: f32,
    height: f32,
    color: iced::Color,
}
```

You can also animate multiple values at once by providing a tuple up to a
length of four:

```rust
AnimationBuilder::new((self.size, self.color), |(size, color)| {
    container(text(size as isize).color(color))
        .center(size)
        .into()
})
.animates_layout(true)
```

## Controlling the animation

You can customize the animation by passing in different values to the 
`.animation` function. You can pass anything that can be converted to an
`animated::Mode`, namely a `spring::Motion` or `transition::Easing`. There are
defaults like `Easing::EASE_OUT` and `Motion::BOUNCY`, but you can create your
own as well.

```rust
use std::time::Duration;
use iced::widget::{container, text};
use iced_anim::{AnimationBuilder, spring::Motion};

AnimationBuilder::new(self.size, |size| {
    container(text(size as isize))
        .center(size)
        .into()
})
.animates_layout(true)
.animation(Motion { 
    damping: 0.5,
    response: Duration::from_millis(500),
})
```

## Examples

Refer to the `examples` directory for a variety of ways to use this crate.
You can also run these examples locally with `cargo run --example <package>`,
e.g. `cargo run --example animated_color`.
