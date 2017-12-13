// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Geometry
//!
//! This module provides basic geometry support structs and functions
//! for the types of math that user interfaces typically need to
//! support.
//!
//! It would be nice to see this go away in the future and use
//! something that is standard across the Rust ecosystem.

mod bounding_box;
mod dimensions;
mod line;
mod path;
mod point;
mod rectangle;
mod transform;

pub use self::bounding_box::BoundingBox;
pub use self::dimensions::Dimensions;
pub use self::line::Line;
pub use self::path::Path;
pub use self::point::Point;
pub use self::rectangle::Rectangle;
pub use self::transform::Transform;
