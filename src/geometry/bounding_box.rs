// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::{Dimensions, Point};

/// An axis-aligned bounding box.
///
/// Bounding boxes are in an inverted Cartesian coordinate
/// system where the positive Y axis goes downward as it does
/// in most window systems.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundingBox {
    /// Left, top corner of the bounding box.
    ///
    /// This will be the `min(x)` and `min(y)` values
    /// for this box.
    pub left_top: Point,

    /// Right, bottom corner of the bounding box.
    ///
    /// This will be the `max(x)` and `max(y)` values
    /// for this box.
    pub right_bottom: Point,
}

impl BoundingBox {
    /// Create a new `BoundingBox`.
    pub fn new(left_top: Point, right_bottom: Point) -> BoundingBox {
        BoundingBox {
            left_top: left_top,
            right_bottom: right_bottom,
        }
    }

    /// Get the dimensions of this bounding box.
    pub fn dimensions(&self) -> Dimensions {
        Dimensions {
            width: self.right_bottom.x - self.left_top.x,
            height: self.left_top.y - self.right_bottom.y,
        }
    }
}
