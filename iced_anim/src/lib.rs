//! A library for easily creating animations in Iced. Animations are designed around spring physics
//! to make animations feel natural and interactive.
//!
//! # Overview
//!
//! The primary widgets are `Animation` and `AnimationBuilder`, which are widgets that help you
//! animates a value when it changes. The `Animation` widget allows you to store the animated value
//! in your app state, while the `AnimationBuilder` widget maintains the animated value so your app
//! state is always up-to-date with the latest value.
//!
//! # Example
//!
//! You can animate anything that implements `Animate`. For example, you can animate an `f32` value
//! in your app state:
//!
//! ```rust
//! #[derive(Default)]
//! struct State {
//!     size: f32,
//! }
//! ```
//!
//! Then in your view, you can use an `AnimationBuilder` to animate the size of a container:
//!
//! ```rust
//! # use iced::{Element, widget::{container, text}};
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
//! more to come in the future. You can also derive `Animate` on your own types to animate them
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
//!     container(text(size as isize).color(color))
//!         .center(size)
//!         .into()
//! })
//! # .into()
//! #   }
//! # }
//! ```
//!
//! # Supported Iced versions
//!
//! This crate supports Iced 0.13 and newer.
//!
//! # `AnimationBuilder` Limitations
//!
//! It might not be easy or possible to pass in non-clonable content like custom
//! elements to `AnimationBuilder`'s closure to due the closure being having to be
//! invoked multiple times to animate between values. Making reusable functions
//! that use this widget and also take a generic element might be difficult.
//!
//! Nested animations also don't work if both properties are actively being
//! animated. One animation at a time will function correctly, but trying to adjust
//! both at the same time leads to the inner property skipping to the final value.
//! Use the `Animation` widget if you need any of these properties.
//!
//! If these limitations apply to you, consider using the `Animation` widget instead.

pub mod animate;
pub mod animation;
pub mod animation_builder;
pub mod spring;
pub mod spring_event;
pub mod spring_motion;

pub use animate::Animate;
pub use animation::Animation;
pub use animation_builder::*;
pub use spring::Spring;
pub use spring_event::SpringEvent;
pub use spring_motion::SpringMotion;

#[cfg(feature = "derive")]
pub use iced_anim_derive::Animate;
