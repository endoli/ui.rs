// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::{Dimensions, Point};

/// A rectangle.
///
/// Rectangles describe an area within a cartesian coordinate
/// system. They differ from `BoundingBox` in their assumptions
/// about the direction of the Y axis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    /// Corner of the rectangle with the lowest `x` and `y` values.
    pub min: Point,
    /// Corner of the rectangle with the highest `x` and `y` values.
    pub max: Point,
}

impl Rectangle {
    /// Create a new `Rectangle`.
    pub fn new(min: Point, max: Point) -> Rectangle {
        Rectangle { min: min, max: max }
    }

    /// Get the dimensions of this rectangle.
    pub fn dimensions(&self) -> Dimensions {
        Dimensions {
            width: self.max.x - self.min.x,
            height: self.max.y - self.min.y,
        }
    }
}
