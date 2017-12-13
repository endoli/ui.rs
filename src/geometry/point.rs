// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// A point on a 2D plane.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    /// The `x` coordinate of the point.
    pub x: f32,
    /// The `y` coordinate of the point.
    pub y: f32,
}

impl Point {
    /// Create a new `Point`.
    pub fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }
}
