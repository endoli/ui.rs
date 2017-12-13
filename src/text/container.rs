// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Text Container

use geometry::Dimensions;
use super::AttributedText;

/// A display container for text.
///
/// The `TextContainer` contains `AttributedText` and handles tracking
/// the geometry that it will be displayed within. It collaborates with
/// the `TextLayout` which will position characters within the container.
#[allow(dead_code)]
pub struct TextContainer {
    dimensions: Dimensions,
    content: AttributedText,
}

impl TextContainer {
    /// Create a new `TextContainer` with the specified dimensions and content.
    pub fn new(dimensions: Dimensions, content: AttributedText) -> Self {
        TextContainer {
            dimensions,
            content,
        }
    }
}
