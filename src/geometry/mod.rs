// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]

//! Geometry for User Interfaces
//!
//! This module provides 2 coordinate spaces and point, rectangle,
//! size and 2D vector types for each coordinate space. The coordinate
//! space is part of the type so that values in one space can't be used
//! unintentionally within a different context.
//!

use euclid;

/// Coordinate system of the render target in physical pixels.
pub struct DeviceSpace;
/// A point in `DeviceSpace`.
pub type DevicePoint = euclid::TypedPoint2D<f32, DeviceSpace>;
/// A rectangle in `DeviceSpace`.
pub type DeviceRect = euclid::TypedRect<f32, DeviceSpace>;
/// A size in `DeviceSpace`.
pub type DeviceSize = euclid::TypedSize2D<f32, DeviceSpace>;
/// A 2D vector in `DeviceSpace`.
pub type DeviceVector2D = euclid::TypedVector2D<f32, DeviceSpace>;

/// Coordinate system in which layout occurs. These do not have the
/// device pixel ratio applied.
pub struct LogicalSpace;
/// A point in `LogicalSpace`.
pub type LogicalPoint = euclid::TypedPoint2D<f32, LogicalSpace>;
/// A rectangle in `LogicalSpace`.
pub type LogicalRect = euclid::TypedRect<f32, LogicalSpace>;
/// A size in `LogicalSpace`.
pub type LogicalSize = euclid::TypedSize2D<f32, LogicalSpace>;
/// A 2D vector in `LogicalSpace`.
pub type LogicalVector2D = euclid::TypedVector2D<f32, LogicalSpace>;
