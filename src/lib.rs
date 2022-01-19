/*
    Copyright 2022 Jacob Birkett

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

pub mod types;

use types::*;

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Rgba { inner: Rgb, alpha: Option<f64> },
    Hsva { inner: Hsv, alpha: Option<f64> },
    Hsla { inner: Hsl, alpha: Option<f64> },
}

impl Color {
    pub fn new_rgba<I: IntoIterator<Item = f64>>(color: I) -> Self {
        let mut color = color.into_iter();

        Self::Rgba {
            inner: Rgb {
                r: color.next().expect("could not find R value"),
                g: color.next().expect("could not find G value"),
                b: color.next().expect("could not find B value"),
            },
            alpha: color.next(),
        }
    }

    pub fn new_hsva<I: IntoIterator<Item = f64>>(color: I) -> Self {
        let mut color = color.into_iter();

        Self::Hsva {
            inner: Hsv {
                h: color.next().expect("could not find H value"),
                s: color.next().expect("could not find S value"),
                v: color.next().expect("could not find V value"),
            },
            alpha: color.next(),
        }
    }

    pub fn new_hsla<I: IntoIterator<Item = f64>>(color: I) -> Self {
        let mut color = color.into_iter();

        Self::Hsla {
            inner: Hsl {
                h: color.next().expect("could not find H value"),
                s: color.next().expect("could not find S value"),
                l: color.next().expect("could not find L value"),
            },
            alpha: color.next(),
        }
    }

    // fn neighboring(c: f64, x: f64, h1: f64) -> (f64, f64, f64) {
    //     match () {
    //         _ if 0.0 <= h1 && h1 < 1.0 => (c, x, 0.0),
    //         _ if 1.0 <= h1 && h1 < 2.0 => (x, c, 0.0),
    //         _ if 2.0 <= h1 && h1 < 3.0 => (0.0, c, x),
    //         _ if 3.0 <= h1 && h1 < 4.0 => (0.0, x, c),
    //         _ if 4.0 <= h1 && h1 < 5.0 => (x, 0.0, c),
    //         _ if 5.0 <= h1 && h1 < 6.0 => (c, 0.0, x),
    //         _ => (0.0, 0.0, 0.0),
    //     }
    // }

    // pub fn to_rgba(&self) -> Self {
    //     match self {
    //         Color::Rgba { .. } => *self,
    //         Color::Hsva { h, s, v, a } => {
    //             //https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_RGB
    //             let c = v * s;
    //             let h1 = h / 60.0;
    //             let x = c * (1.0 - (h1 % 2.0 - 1.0).abs());
    //             let (r1, g1, b1) = Self::neighboring(c, x, h1);
    //             let m = v - c;
    //             let (r, g, b) = (r1 + m, g1 + m, b1 + m);

    //             Color::Rgba { r, g, b, a: *a }
    //         }
    //         Color::Hsla { h, s, l, a } => {
    //             // https://en.wikipedia.org/wiki/HSL_and_HSV#HSL_to_RGB
    //             let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    //             let h1 = h / 60.0;
    //             let x = c * (1.0 - (h1 % 2.0 - 1.0).abs());
    //             let (r1, g1, b1) = Self::neighboring(c, x, h1);
    //             let m = l - (c / 2.0);
    //             let (r, g, b) = (r1 + m, g1 + m, b1 + m);

    //             Color::Rgba { r, g, b, a: *a }
    //         }
    //     }
    // }

    // pub fn to_hsva(&self) -> Self {
    //     match self {
    //         Color::Rgba { r, g, b, a } => {
    //             // https://en.wikipedia.org/wiki/HSL_and_HSV#From_RGB
    //             let xmax = r.max(g.max(*b));
    //             let xmin = r.min(g.min(*b));
    //             let c = xmax - xmin;
    //             let mut h = match () {
    //                 _ if c == 0.0 => 0.0,
    //                 _ if xmax == *r => 60.0 * ((g - b) / c),
    //                 _ if xmax == *g => 60.0 * ((b - r) / c + 2.0),
    //                 _ if xmax == *b => 60.0 * ((r - g) / c + 4.0),
    //                 _ => panic!(),
    //             };
    //             if h < 0.0 {
    //                 h += 360.0
    //             };
    //             let s = match () {
    //                 _ if xmax == 0.0 => 0.0,
    //                 _ => c / xmax,
    //             };

    //             Color::Hsva {
    //                 h,
    //                 s,
    //                 v: xmax,
    //                 a: *a,
    //             }
    //         }
    //         Color::Hsva { .. } => *self,
    //         Color::Hsla { h, s, l, a } => {
    //             // https://en.wikipedia.org/wiki/HSL_and_HSV#HSL_to_HSV
    //             let v = l + s * l.min(1.0 - l);
    //             let sv = match () {
    //                 _ if v == 0.0 => 0.0,
    //                 _ => 2.0 * (1.0 - l / v),
    //             };

    //             Color::Hsva {
    //                 h: *h,
    //                 s: sv,
    //                 v,
    //                 a: *a,
    //             }
    //         }
    //     }
    // }

    // pub fn to_hsla(&self) -> Self {
    //     match self {
    //         Color::Rgba { r, g, b, a } => {
    //             // https://en.wikipedia.org/wiki/HSL_and_HSV#From_RGB
    //             let xmax = r.max(g.max(*b));
    //             let xmin = r.min(g.min(*b));
    //             let c = xmax - xmin;
    //             let mut h = match () {
    //                 _ if c == 0.0 => 0.0,
    //                 _ if xmax == *r => 60.0 * ((g - b) / c),
    //                 _ if xmax == *g => 60.0 * ((b - r) / c + 2.0),
    //                 _ if xmax == *b => 60.0 * ((r - g) / c + 4.0),
    //                 _ => panic!(),
    //             };
    //             if h < 0.0 {
    //                 h += 360.0
    //             };
    //             let l = (xmax + xmin) / 2.0;
    //             let s = match () {
    //                 _ if l == 0.0 || l == 1.0 => 0.0,
    //                 _ => c / (1.0 - (2.0 * xmax - c - 1.0).abs()),
    //             };

    //             Color::Hsla { h, s, l, a: *a }
    //         }
    //         Color::Hsva { h, s, v, a } => {
    //             // https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_HSL
    //             let l = v * (1.0 - (s / 2.0));
    //             let sl = match () {
    //                 _ if l == 0.0 || l == 1.0 => 0.0,
    //                 _ => 2.0 * (1.0 - l / v),
    //             };

    //             Color::Hsla {
    //                 h: *h,
    //                 s: sl,
    //                 l,
    //                 a: *a,
    //             }
    //         }
    //         Color::Hsla { .. } => *self,
    //     }
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
