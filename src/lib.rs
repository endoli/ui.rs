// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Let a thousand flowers bloom.
//!
//! Many components of user interface frameworks can be shared.
//! This crate exists to share some common elements that are needed
//! by many, hopefully improving functionality, correctness, and
//! performance.
//!
//! Things that we should cover or address:
//!
//! * Colors
//! * Geometry and coordinate systems
//!   * Including beziers and paths
//! * Text styling and layout
//!   * Spellchecking integration
//! * Input events
//! * Animation assistants
//!
//! We should integrate with other UI frameworks as well as platform
//! native frameworks where possible. This can be as simple as providing
//! conversions from our types to those of other systems, or perhaps
//! more complicated or detailed integrations in the future.

#![warn(missing_docs)]
#![deny(trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]

pub mod color;
pub mod geometry;
pub mod text;
