// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Text Management and Layout

mod attributes;
mod container;
mod layout;
mod attributedtext;

pub use self::attributes::Attribute;
pub use self::container::TextContainer;
pub use self::layout::TextLayout;
pub use self::attributedtext::AttributedText;
