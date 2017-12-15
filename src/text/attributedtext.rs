// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Text Storage

use super::Attribute;

use std::ops::Range;

/// A body of text which has been marked up with attributes for style,
/// layout, accessibility, and other features.
#[allow(dead_code)]
pub struct AttributedText {
    attributes: Vec<(Range<usize>, Attribute)>,
    content: String,
}

impl AttributedText {
    /// Create a new `AttributedText` with the given content.
    pub fn new<T: Into<String>>(content: T) -> Self {
        AttributedText {
            attributes: vec![],
            content: content.into(),
        }
    }

    /// Create a new `AttributedText` with the given content and attributes.
    pub fn new_with_attributes<T: Into<String>>(content: T, attributes: &[(Range<usize>, Attribute)]) -> Self {
        AttributedText {
            attributes: attributes.to_vec(),
            content: content.into(),
        }
    }

    #[allow(missing_docs)]
    pub fn add_attribute(&mut self, range: Range<usize>, attribute: Attribute) {
        self.attributes.push((range, attribute));
    }
}
