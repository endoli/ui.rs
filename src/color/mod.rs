// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Shared color definitions.
//!
//! Colors may be specified in various coordinate systems and then
//! converted into other coordinate systems for display or manipulation.
//!
//! Colors may also be specified by name.
//!
//! ## Integration
//!
//! Integration with the color formats used by other libraries may be
//! available.
//!
//! ### iOS and macOS
//!
//! This will be forthcoming. Pull requests are welcome.
//!
//! ### WebRender
//!
//! WebRender integration is optionally available which allows for
//! converting `rgba::Color` values into WebRender's color structures.
//! This is currently not functioning and will be fixed in the near
//! future.
//!
//! ### GTK+
//!
//! This will be forthcoming. Pull requests are welcome.

pub mod cmyk;
pub mod hsla;
pub mod hsva;
pub mod rgba;

#[cfg(feature = "webrender")]
mod webrender;

#[allow(missing_docs)]
pub trait Color {
    fn components(&self) -> &[f32];
    fn component_count(&self) -> usize;
    // TODO: What else?
}

/// Create a color from HSL components.
///
/// The alpha component will be set to fully opaque (100% or 1.0).
///
/// ```
/// let c = ui::color::hsl(120.0, 1.0, 0.5);
/// ```
pub fn hsl(h: f32, s: f32, l: f32) -> hsla::Color {
    hsla::Color::new(h, s, l, 1.0)
}

/// Create a color from HSL components with alpha.
///
/// ```
/// let c = ui::color::hsla(120.0, 1.0, 0.5, 0.9);
/// ```
pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> hsla::Color {
    hsla::Color::new(h, s, l, a)
}

#[allow(missing_docs)]
pub fn rgb(r: f32, g: f32, b: f32) -> rgba::Color {
    rgba::Color::new(r, g, b, 1.0)
}

#[allow(missing_docs)]
pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> rgba::Color {
    rgba::Color::new(r, g, b, a)
}
