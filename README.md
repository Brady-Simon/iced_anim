# Iced Animations

This package is designed to make it easy to animate between values using 
[the Iced framework](https://github.com/iced-rs/iced).

## Overview

There are two main widgets exposed: `AnimationBuilder` and `Animation`. Both
work off the core `Animate` trait defining how a value is animated, but differ
on where the animated state is kept. The `AnimationBuilder` stores the animated
value within the widget itself while the `Animation` widget takes the animated
value from props and allows you to emit a message to update the value.

Both widgets animate values using spring physics instead of easing functions to
allow for more natural and interruptible animations.

### Animated widgets

A subset of the standard `iced` widgets are exported under a `widgets` feature
flag. You can use these as drop-in replacements for the existing widgets:

```rust
use iced::widget::text;
use iced_anim::widget::button;

let my_button = button(text("Animated button"))
    .on_press(Message::DoSomething);
```

### `AnimationBuilder` widget

The `AnimationBuilder` widget takes some sort of value that implements 
`Animate` and a closure to build out a UI based on the current interpolated
value. The benefit is that you don't have to emit messages for very simple 
animations but has a couple limitations mentioned below. Typical usage might
look something like this, where `self.size` is an `f32` in your app state:

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
view as appropriate. To see more for this particular example, refer to the
`animated_size` example.

#### Limitations

It might not be easy/possible to pass in non-clonable content like custom
elements to `AnimationBuilder`'s closure to due the closure being having to be
invoked multiple times to animate between values. Making reusable functions
that use this widget and also take a generic element might be difficult.

Nested animations also don't work if both properties are actively being 
animated. One animation at a time will function correctly, but trying to adjust
both at the same time leads to the inner property skipping to the final value.
Use the `Animation` widget if you need any of these properties.

### `Animation` widget

The `Animation` widget works by taking a `Spring<T>` value and some element,
then emitting a message when the value needs to change. Your app state and 
message would look something like this:

```rust
use iced_anim::{Animation, Spring, Event};

#[derive(Default)]
struct State {
    size: Spring<f32>,
}

#[derive(Clone)]
enum Message {
    UpdateSize(Event<f32>),
}
```

Then, somewhere in your view, have a way to change the spring's target and
update the animated value. You can use `.into()` as a shorthand to create a
`Event::Target` for changing where the spring should animate towards.

```rust
use iced::widget::{Column, button, text};

Column::new()
    .push(
        button(text("+50"))
            .on_press(Message::UpdateSize((self.size.value() + 50.0).into()))
    )
    .push(
        Animation::new(self.size, text(self.size.value().to_string()))
            .on_update(Message::UpdateSize)
    )
```

Finally, your update function will forward the event to `self.size.update` to 
update the spring's inner state correctly. 

```rust
impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::UpdateSize(event) => self.size.update(event),
        }
    }
}
```

See the `stateful_animation` example for a complete example.

### Which one should I use?

Generally, if you're animating a tiny value that might not be directly within
your state and the element won't contain any nested animated values, then use
`AnimationBuilder`. Otherwise, use the state-driven `Animation` to avoid the
limitations of widget-driven animations. Also, use `Animation` anytime you want
your state to contain the animated value.

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

## Controlling the spring motion

The spring motion of an `AnimationBuilder` can be customized. There are a few
defaults like `Motion::SMOOTH` and `Motion::BOUNCY`, but you can also create your own.

```rust
AnimationBuilder::new(self.size, |size| {
    container(text(size as isize))
        .center(size)
        .into()
})
.animates_layout(true)
.motion(Motion { 
    damping: 0.5,
    response: Duration::from_millis(500),
})
```

## Examples

Refer to the `examples` directory for a variety of ways to use this crate.
You can also run these examples locally with `cargo run --example <package>`,
e.g. `cargo run --example animated_color`.
