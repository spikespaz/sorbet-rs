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
pub struct Hsva {
    pub h: f64,
    pub s: f64,
    pub v: f64,
    pub alpha: f64,
}

impl Eq for Hsva {}

#[allow(clippy::derive_hash_xor_eq)]
impl hash::Hash for Hsva {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.h.to_bits().hash(state);
        self.s.to_bits().hash(state);
        self.v.to_bits().hash(state);
        self.alpha.to_bits().hash(state);
    }
}

//
// Implement to/from primitives
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

impl From<Hsva> for [f64; 4] {
    fn from(color: Hsva) -> Self {
        [color.h, color.s, color.v, color.alpha]
    }
}

//
// Implement From for all other Color types
//

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
