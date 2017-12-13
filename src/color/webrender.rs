// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! WebRender integration

use std::convert::From;
use webrender_traits;

impl From<Color> for webrender_traits::ColorF {
    fn from(c: Color) -> Self {
        webrender_traits::ColorF {
            r: c.components[0],
            g: c.components[1],
            b: c.components[2],
            a: c.components[3],
        }
    }
}

impl From<Color> for webrender_traits::ColorU {
    fn from(c: Color) -> Self {
        webrender_traits::ColorF {
            r: c.components[0],
            g: c.components[1],
            b: c.components[2],
            a: c.components[3],
        }.into()
    }
}

impl From<webrender_traits::ColorF> for Color {
    fn from(c: webrender_traits::ColorF) -> Self {
        Color { components: [c.r, c.g, c.b, c.a] }
    }
}

impl From<webrender_traits::ColorU> for Color {
    fn from(c: webrender_traits::ColorU) -> Self {
        let cf: webrender_traits::ColorF = c.into();
        Color { components: [cf.r, cf.g, cf.b, cf.a] }
    }
}

#[cfg(test)]
mod tests {
    use super::Color;
    use webrender_traits;

    #[test]
    fn webrender_coloru_conversion() {
        let c = Color::new(1.0f32, 0.0, 0.0, 1.0f32);
        let wrtcu = webrender_traits::ColorU::new(255, 0, 0, 255);
        assert_eq!(wrtcu, c.into());
        assert_eq!(c, wrtcu.into());
    }
}
