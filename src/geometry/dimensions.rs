// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Width and height in a 2D space.
///
/// Used for the dimensions of a `Rectangle` and
/// `BoundingBox`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dimensions {
    /// Width, or distance along the `x` axis.
    pub width: f32,
    /// Height, or distance along the `y` axis.
    pub height: f32,
}

impl Dimensions {
    /// Create a new `Dimensions`.
    pub fn new(width: f32, height: f32) -> Dimensions {
        Dimensions {
            width: width,
            height: height,
        }
    }
}
