// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::{BoundingBox, Point};

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    /// Create a new `Line`.
    pub fn new(start: Point, end: Point) -> Line {
        Line {
            start: start,
            end: end,
        }
    }

    /// Calculate the `BoundingBox` of this line.
    ///
    /// ```
    /// use ui::geometry::{Line, Point};
    ///
    /// let line = Line::new(Point::new(0.0, 0.0),
    ///                      Point::new(3.0, 4.0));
    /// let bounds = line.bounding_box();
    /// assert_eq!(bounds.left_top, Point::new(0.0, 4.0));
    /// assert_eq!(bounds.right_bottom, Point::new(3.0, 0.0));
    /// ```
    pub fn bounding_box(&self) -> BoundingBox {
        let left = f32::min(self.start.x, self.end.x);
        let right = f32::max(self.start.x, self.end.x);
        let top = f32::max(self.start.y, self.end.y);
        let bottom = f32::min(self.start.y, self.end.y);
        BoundingBox::new(Point::new(left, top), Point::new(right, bottom))
    }

    /// Is this `point` on this line?
    pub fn contains_point(&self, _point: &Point) -> bool {
        // let bounds = self.bounding_box();
        unimplemented!();
    }

    /// Is the other line on this line?
    pub fn contains_line(&self, other: &Line) -> bool {
        self.contains_point(&other.start) && self.contains_point(&other.end)
    }

    /// Does this line intersect another line?
    pub fn intersects_line(&self, _other: &Line) -> bool {
        unimplemented!();
    }
}
