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

use std::fmt;
use std::hash;

pub trait Color:
    Copy
    + Clone
    + fmt::Debug
    + PartialEq
    + Eq
    + hash::Hash
    + From<Rgb>
    + From<Rgba>
    + From<Hsv>
    + From<Hsva>
    + From<Hsl>
    + From<Hsla>
{
    fn new(color: &str) -> Self {
        let color = match color.strip_prefix('#') {
            Some(string) => string,
            None => color,
        };

        let r = u8::from_str_radix(&color[0..2], 16).expect("invalid hexadecimal string");
        let g = u8::from_str_radix(&color[2..4], 16).expect("invalid hexadecimal string");
        let b = u8::from_str_radix(&color[4..6], 16).expect("invalid hexadecimal string");

        let alpha = if color.len() == 8 {
            u8::from_str_radix(&color[6..8], 16).expect("invalid hexadecimal string")
        } else {
            1
        };

        Self::from(Rgba {
            r: r as f64 / 255.0,
            g: g as f64 / 255.0,
            b: b as f64 / 255.0,
            alpha: alpha as f64 / 255.0,
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rgba {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub alpha: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Hsv {
    pub h: f64,
    pub s: f64,
    pub v: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Hsva {
    pub h: f64,
    pub s: f64,
    pub v: f64,
    pub alpha: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Hsl {
    pub h: f64,
    pub s: f64,
    pub l: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Hsla {
    pub h: f64,
    pub s: f64,
    pub l: f64,
    pub alpha: f64,
}

impl Eq for Rgb {}
impl Eq for Rgba {}
impl Eq for Hsv {}
impl Eq for Hsva {}
impl Eq for Hsl {}
impl Eq for Hsla {}

impl Color for Rgb {}
impl Color for Rgba {}
impl Color for Hsv {}
impl Color for Hsva {}
impl Color for Hsl {}
impl Color for Hsla {}

#[allow(clippy::derive_hash_xor_eq)]
impl hash::Hash for Rgb {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.r.to_bits().hash(state);
        self.g.to_bits().hash(state);
        self.b.to_bits().hash(state);
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl hash::Hash for Rgba {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.r.to_bits().hash(state);
        self.g.to_bits().hash(state);
        self.b.to_bits().hash(state);
        self.alpha.to_bits().hash(state);
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl hash::Hash for Hsv {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.h.to_bits().hash(state);
        self.s.to_bits().hash(state);
        self.v.to_bits().hash(state);
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl hash::Hash for Hsva {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.h.to_bits().hash(state);
        self.s.to_bits().hash(state);
        self.v.to_bits().hash(state);
        self.alpha.to_bits().hash(state);
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl hash::Hash for Hsl {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.h.to_bits().hash(state);
        self.s.to_bits().hash(state);
        self.l.to_bits().hash(state);
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl hash::Hash for Hsla {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.h.to_bits().hash(state);
        self.s.to_bits().hash(state);
        self.l.to_bits().hash(state);
        self.alpha.to_bits().hash(state);
    }
}

//
// impl From<*> for Rgb
//

impl From<[f64; 3]> for Rgb {
    fn from(array: [f64; 3]) -> Self {
        Self {
            r: array[0],
            g: array[1],
            b: array[2],
        }
    }
}

impl From<Hsv> for Rgb {
    fn from(other: Hsv) -> Self {
        //https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_RGB
        let c = other.v * other.s;
        let h1 = other.h / 60.0;
        let x = c * (1.0 - (h1 % 2.0 - 1.0).abs());
        let (r1, g1, b1) = neighboring(c, x, h1);
        let m = other.v - c;
        let (r, g, b) = (r1 + m, g1 + m, b1 + m);

        Self { r, g, b }
    }
}

impl From<Hsl> for Rgb {
    fn from(other: Hsl) -> Self {
        // https://en.wikipedia.org/wiki/HSL_and_HSV#HSL_to_RGB
        let c = (1.0 - (2.0 * other.l - 1.0).abs()) * other.s;
        let h1 = other.h / 60.0;
        let x = c * (1.0 - (h1 % 2.0 - 1.0).abs());
        let (r1, g1, b1) = neighboring(c, x, h1);
        let m = other.l - (c / 2.0);
        let (r, g, b) = (r1 + m, g1 + m, b1 + m);

        Self { r, g, b }
    }
}

impl From<Rgba> for Rgb {
    fn from(other: Rgba) -> Self {
        Self {
            r: other.r,
            g: other.g,
            b: other.b,
        }
    }
}

impl From<Hsva> for Rgb {
    fn from(other: Hsva) -> Self {
        Self::from(Hsv::from(other))
    }
}

impl From<Hsla> for Rgb {
    fn from(other: Hsla) -> Self {
        Self::from(Hsl::from(other))
    }
}

//
// impl From<*> for Rgba
//

impl From<[f64; 4]> for Rgba {
    fn from(array: [f64; 4]) -> Self {
        Self {
            r: array[0],
            g: array[1],
            b: array[2],
            alpha: array[3],
        }
    }
}

impl From<Rgb> for Rgba {
    fn from(other: Rgb) -> Self {
        Self {
            r: other.r,
            g: other.g,
            b: other.b,
            alpha: 1.0,
        }
    }
}

impl From<Hsv> for Rgba {
    fn from(other: Hsv) -> Self {
        Self::from(Rgb::from(other))
    }
}

impl From<Hsl> for Rgba {
    fn from(other: Hsl) -> Self {
        Self::from(Rgb::from(other))
    }
}

impl From<Hsva> for Rgba {
    fn from(other: Hsva) -> Self {
        let Rgb { r, g, b } = Rgb::from(other);

        Self {
            r,
            g,
            b,
            alpha: other.alpha,
        }
    }
}

impl From<Hsla> for Rgba {
    fn from(other: Hsla) -> Self {
        let Rgb { r, g, b } = Rgb::from(other);

        Self {
            r,
            g,
            b,
            alpha: other.alpha,
        }
    }
}

//
// impl From<*> for Hsv
//

impl From<[f64; 3]> for Hsv {
    fn from(array: [f64; 3]) -> Self {
        Self {
            h: array[0],
            s: array[1],
            v: array[2],
        }
    }
}

impl From<Rgb> for Hsv {
    fn from(other: Rgb) -> Self {
        // https://en.wikipedia.org/wiki/HSL_and_HSV#From_RGB
        let xmax = other.r.max(other.g.max(other.b));
        let xmin = other.r.min(other.g.min(other.b));
        let c = xmax - xmin;
        let mut h = match () {
            _ if c == 0.0 => 0.0,
            _ if xmax == other.r => 60.0 * ((other.g - other.b) / c),
            _ if xmax == other.g => 60.0 * ((other.b - other.r) / c + 2.0),
            _ if xmax == other.b => 60.0 * ((other.r - other.g) / c + 4.0),
            _ => panic!(),
        };
        if h < 0.0 {
            h += 360.0
        };
        let s = match () {
            _ if xmax == 0.0 => 0.0,
            _ => c / xmax,
        };

        Self { h, s, v: xmax }
    }
}

impl From<Hsl> for Hsv {
    fn from(other: Hsl) -> Self {
        // https://en.wikipedia.org/wiki/HSL_and_HSV#HSL_to_HSV
        let v = other.l + other.s * other.l.min(1.0 - other.l);
        let sv = match () {
            _ if v == 0.0 => 0.0,
            _ => 2.0 * (1.0 - other.l / v),
        };

        Self {
            h: other.h,
            s: sv,
            v,
        }
    }
}

impl From<Rgba> for Hsv {
    fn from(other: Rgba) -> Self {
        Self::from(Rgb::from(other))
    }
}

impl From<Hsva> for Hsv {
    fn from(other: Hsva) -> Self {
        Self {
            h: other.h,
            s: other.s,
            v: other.v,
        }
    }
}

impl From<Hsla> for Hsv {
    fn from(other: Hsla) -> Self {
        Self::from(Hsl::from(other))
    }
}

//
// impl From<*> for Hsva
//

impl From<[f64; 4]> for Hsva {
    fn from(array: [f64; 4]) -> Self {
        Self {
            h: array[0],
            s: array[1],
            v: array[2],
            alpha: array[3],
        }
    }
}

impl From<Rgb> for Hsva {
    fn from(other: Rgb) -> Self {
        Self::from(Hsv::from(other))
    }
}

impl From<Hsv> for Hsva {
    fn from(other: Hsv) -> Self {
        Self {
            h: other.h,
            s: other.s,
            v: other.v,
            alpha: 1.0,
        }
    }
}

impl From<Hsl> for Hsva {
    fn from(other: Hsl) -> Self {
        Self::from(Hsv::from(other))
    }
}

impl From<Rgba> for Hsva {
    fn from(other: Rgba) -> Self {
        let Hsv { h, s, v } = Hsv::from(other);

        Self {
            h,
            s,
            v,
            alpha: other.alpha,
        }
    }
}

impl From<Hsla> for Hsva {
    fn from(other: Hsla) -> Self {
        let Hsv { h, s, v } = Hsv::from(other);

        Self {
            h,
            s,
            v,
            alpha: other.alpha,
        }
    }
}

//
// impl From<*> for Hsl
//

impl From<[f64; 3]> for Hsl {
    fn from(array: [f64; 3]) -> Self {
        Self {
            h: array[0],
            s: array[1],
            l: array[2],
        }
    }
}

impl From<Rgb> for Hsl {
    fn from(other: Rgb) -> Self {
        // https://en.wikipedia.org/wiki/HSL_and_HSV#From_RGB
        let xmax = other.r.max(other.g.max(other.b));
        let xmin = other.r.min(other.g.min(other.b));
        let c = xmax - xmin;
        let mut h = match () {
            _ if c == 0.0 => 0.0,
            _ if xmax == other.r => 60.0 * ((other.g - other.b) / c),
            _ if xmax == other.g => 60.0 * ((other.b - other.r) / c + 2.0),
            _ if xmax == other.b => 60.0 * ((other.r - other.g) / c + 4.0),
            _ => panic!(),
        };
        if h < 0.0 {
            h += 360.0
        };
        let l = (xmax + xmin) / 2.0;
        let s = match () {
            _ if l == 0.0 || l == 1.0 => 0.0,
            _ => c / (1.0 - (2.0 * xmax - c - 1.0).abs()),
        };

        Self { h, s, l }
    }
}

impl From<Hsv> for Hsl {
    fn from(other: Hsv) -> Self {
        // https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_HSL
        let l = other.v * (1.0 - (other.s / 2.0));
        let sl = match () {
            _ if l == 0.0 || l == 1.0 => 0.0,
            _ => 2.0 * (1.0 - l / other.v),
        };

        Self {
            h: other.h,
            s: sl,
            l,
        }
    }
}

impl From<Rgba> for Hsl {
    fn from(other: Rgba) -> Self {
        Self::from(Rgb::from(other))
    }
}

impl From<Hsla> for Hsl {
    fn from(other: Hsla) -> Self {
        Self {
            h: other.h,
            s: other.s,
            l: other.l,
        }
    }
}

impl From<Hsva> for Hsl {
    fn from(other: Hsva) -> Self {
        Self::from(Hsv::from(other))
    }
}

// impl From<*> for Hsla

impl From<[f64; 4]> for Hsla {
    fn from(array: [f64; 4]) -> Self {
        Self {
            h: array[0],
            l: array[1],
            s: array[2],
            alpha: array[3],
        }
    }
}

impl From<Rgb> for Hsla {
    fn from(other: Rgb) -> Self {
        Self::from(Rgba::from(other))
    }
}

impl From<Hsv> for Hsla {
    fn from(other: Hsv) -> Self {
        Self::from(Hsva::from(other))
    }
}

impl From<Hsl> for Hsla {
    fn from(other: Hsl) -> Self {
        Self {
            h: other.h,
            s: other.s,
            l: other.l,
            alpha: 1.0,
        }
    }
}

impl From<Rgba> for Hsla {
    fn from(other: Rgba) -> Self {
        let Hsl { h, s, l } = Hsl::from(other);

        Self {
            h,
            s,
            l,
            alpha: other.alpha,
        }
    }
}

impl From<Hsva> for Hsla {
    fn from(other: Hsva) -> Self {
        let Hsl { h, s, l } = Hsl::from(other);

        Self {
            h,
            s,
            l,
            alpha: other.alpha,
        }
    }
}

//
//
//

fn neighboring(c: f64, x: f64, h1: f64) -> (f64, f64, f64) {
    match () {
        _ if (0.0..1.0).contains(&h1) => (c, x, 0.0),
        _ if (1.0..2.0).contains(&h1) => (x, c, 0.0),
        _ if (2.0..3.0).contains(&h1) => (0.0, c, x),
        _ if (3.0..4.0).contains(&h1) => (0.0, x, c),
        _ if (4.0..5.0).contains(&h1) => (x, 0.0, c),
        _ if (5.0..6.0).contains(&h1) => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    }
}

// #[cfg(test)]
// mod test {
//     #[test]
//     fn test_rgb_from() {
//         Rgb::from(Rgb{0.1, 0.2,})
//     }
// }
