# Iced Animations

This package is designed to make it easy to animate between values using 
[the Iced framework](https://github.com/iced-rs/iced).

## Overview

The main exposed widget is `AnimationBuilder`, which takes some sort of value
that implements `Animate` and a closure to build out a UI based on the current
interpolated value. The animations are driven via spring physics instead of
easing functions to allow for more natural and interruptible animations.
Typical usage might look something like this, where `self.size` is an `f32` in
your app state:

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
defaults like `SpringMotion::Smooth` and `SpringMotion::Bouncy`, but you can 
provide a custom response and damping fraction with `SpringMotion::Custom`:

```rust
AnimationBuilder::new(self.size, |size| {
    container(text(size as isize))
        .center(size)
        .into()
})
.animates_layout(true)
.motion(SpringMotion::Custom { 
    response: Duration::from_millis(500),
    damping: 0.5 
})
```

## Examples

Refer to the `examples` directory for a variety of ways to use this crate.
You can also run these examples locally with `cargo run --example <package>`,
e.g. `cargo run --example animated_color`.

## Limitations

It might not be easy/possible to pass in non-clonable content like custom
elements to `AnimationBuilder`'s closure to due the closure being having to be
invoked multiple times to animate between values. Making reusable functions
that use this widget and also take a generic element might be difficult. This
particular problem might be better solved by a new widget in a future version
that keeps the animated value in the app state rather than within the widget.
