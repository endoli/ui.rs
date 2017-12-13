// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! HSVA Colors

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    components: [f32; 4],
}

impl Color {
    #[allow(missing_docs)]
    pub fn new(h: f32, s: f32, l: f32, a: f32) -> Self {
        Color { components: [h, s, l, a] }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn hue(&self) -> f32 {
        self.components[0]
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn saturation(&self) -> f32 {
        self.components[1]
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn value(&self) -> f32 {
        self.components[2]
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn alpha(&self) -> f32 {
        self.components[3]
    }
}

impl super::Color for Color {
    fn components(&self) -> &[f32] {
        &self.components
    }

    fn component_count(&self) -> usize {
        4
    }
}

// TODO: impl From<rgba::Color> for Color.
// TODO: impl From<hsla::Color> for Color.
// TODO: impl From<cmyk::Color> for Color.
// TODO: impl From for webrender_traits::ColorF and so on?

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn color_new() {
        let c = Color::new(1.0f32, 1.0f32, 1.0f32, 1.0f32);
        assert_eq!(c.components, [1.0f32, 1.0f32, 1.0f32, 1.0f32]);
    }
}
