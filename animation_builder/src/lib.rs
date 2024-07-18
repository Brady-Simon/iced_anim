//! A library for easily creating animations in Iced. Animations are designed around spring physics
//! to make animations feel natural and interactive.
//!
//! # Overview
//!
//! The primary widget is `AnimationBuilder`, which is a widget that implicitly animates a value
//! anytime it changes. Your state is always up-to-date with the latest value, and the widget will
//! use spring interpolation to animate the value over a specified duration.
//!
//! # Example
//!
//! You can animate anything that implements `Animate`. For example, you can animate an `f32` value
//! in your app state:
//!
//! ```rust
//! struct State {
//!     size: f32,
//! }
//! ```
//!
//! Then in your view, you can use an `AnimationBuilder` to animate the size of a container:
//!
//! ```rust
//! # use iced::{Element, Theme, Border, widget::{container, text}};
//! # use iced_anim::AnimationBuilder;
//! # struct State {
//! #     size: f32,
//! # }
//! # #[derive(Clone)]
//! # enum Message {}
//! # impl State {
//! #     fn view(&self) -> Element<Message> {
//! AnimationBuilder::new(self.size, |size| {
//!     container(text(size as isize))
//!         .style(move |theme: &Theme| container::Style {
//!             border: Border {
//!                 color: theme.extended_palette().secondary.strong.color,
//!                 width: 1.0,
//!                 radius: 6.0.into(),
//!             },
//!             background: Some(theme.extended_palette().secondary.weak.color.into()),
//!             ..Default::default()
//!         })
//!         .center(size)
//!         .into()
//! })
//! .animates_layout(true)
//! #   .into()
//! #     }
//! # }
//! ```
//!
//! The element built within the closure will animate to the new size automatically. A handful of
//! common types already implement `Animate` like `f32`, `iced::Color`, and `iced::Theme`, with
//! more to come in the future. You can also dersive `Animate` on your own types to animate them
//! as well.
//!
//! ```rust
//! use iced_anim::Animate;
//!
//! // Note that animate also requires `Clone` and `PartialEq` impls.
//! #[derive(Animate, Clone, PartialEq)]
//! struct MyType {
//!     size: f32,
//!     color: iced::Color,
//! }
//! ```
//!
//! You can also animate multiple values at once by using a tuple up to length of four:
//!
//! ```rust
//! # use iced::{Color, widget::{text, container}};
//! # use iced_anim::AnimationBuilder;
//! # #[derive(Default)]
//! # struct MyType {
//! #   size: f32,
//! #   color: Color,
//! # }
//! # #[derive(Clone)]
//! # enum Message {}
//! # impl MyType {
//! #   fn view(&self) -> iced::Element<Message> {
//! AnimationBuilder::new((self.size, self.color), |(size, color)| {
//!     // Build your element using the animated values
//!     container(text(size as isize).color(color))
//!         .center(size)
//!         .into()
//! })
//! # .into()
//! #   }
//! # }

//! ```
//!
//! # Iced versions
//!
//! This crate currently targets the master version of Iced (0.13-dev as of this writing).
//!
//! # Limitations
//!
//! It may be difficult to make generic wrappers around the `AnimationBuilder` due to it needing a
//! `Fn` closure to build the element and most elements aren't `Clone`. Covering this particular
//! use case will likely require a separate widget that stores the animated value directly in the
//! app state instead of within the `AnimationBuilder`.

pub mod animate;
pub mod animation_builder;
pub mod spring;
pub mod spring_motion;

pub use animate::Animate;
pub use animation_builder::*;
pub use spring::Spring;
pub use spring_motion::SpringMotion;

#[cfg(feature = "derive")]
pub use iced_anim_derive::Animate;
