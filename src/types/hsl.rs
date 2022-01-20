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

use std::hash;

use crate::types::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Hsl {
    pub h: f64,
    pub s: f64,
    pub l: f64,
}

impl Eq for Hsl {}

#[allow(clippy::derive_hash_xor_eq)]
impl hash::Hash for Hsl {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.h.to_bits().hash(state);
        self.s.to_bits().hash(state);
        self.l.to_bits().hash(state);
    }
}

//
// Implement From for all other Color types
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
