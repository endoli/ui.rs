// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! RGBA Colors

pub mod named;

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    components: [f32; 4],
}

impl Color {
    #[allow(missing_docs)]
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { components: [r, g, b, a] }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn red(&self) -> f32 {
        self.components[0]
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn green(&self) -> f32 {
        self.components[1]
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn blue(&self) -> f32 {
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

// TODO: impl From<hsla::Color> for Color.

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn color_new() {
        let c = Color::new(1.0f32, 1.0f32, 1.0f32, 1.0f32);
        assert_eq!(c.components, [1.0f32, 1.0f32, 1.0f32, 1.0f32]);
    }
}
