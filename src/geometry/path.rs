// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::Point;

/// A sequence of line segments.
#[derive(Clone, Debug, PartialEq)]
pub struct Path {
    /// The points that make up the line segments.
    ///
    /// The first 2 points describe the first line
    /// segment. Then points 2 and 3 describe the
    /// second segment, and so on.
    pub points: Vec<Point>,
}
