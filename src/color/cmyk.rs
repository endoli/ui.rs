// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! CMYKA Colors

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    components: [f32; 5],
}

impl Color {
    #[allow(missing_docs)]
    pub fn new(c: f32, m: f32, y: f32, k: f32, a: f32) -> Self {
        Color { components: [c, m, y, k, a] }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn cyan(&self) -> f32 {
        self.components[0]
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn magenta(&self) -> f32 {
        self.components[1]
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn yellow(&self) -> f32 {
        self.components[2]
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn black(&self) -> f32 {
        self.components[3]
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn alpha(&self) -> f32 {
        self.components[4]
    }
}

impl super::Color for Color {
    fn components(&self) -> &[f32] {
        &self.components
    }

    fn component_count(&self) -> usize {
        5
    }
}

// TODO: impl From<rgba::Color> for Color
// TODO: impl From<webrender::ColorF> for Color
// TODO: impl From<cmyk::Color> for webrender::ColorF
// TODO: impl From<hsla::Color> for Color.

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn color_new() {
        let c = Color::new(1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32);
        assert_eq!(c.components, [1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32]);
    }
}
