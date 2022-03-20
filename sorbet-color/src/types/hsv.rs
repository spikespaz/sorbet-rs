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

use crate::{css, types::*, Color};

/// This structure represents colors in the HSV color space with
/// hue, saturation, and value channels.
/// See the [Wikipedia reference](<https://en.wikipedia.org/wiki/HSL_and_HSV>) for details.
///
/// This does not include the alpha/transparency component.
/// If you need transparency, see [`crate::types::Hsva`].
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Hsv {
    /// Hue channel.
    /// Ranged `0.0..360.0`.
    pub h: f64,
    /// Saturation channel.
    /// Ranged `0.0..1.0`.
    pub s: f64,
    /// Value channel.
    /// Ranged `0.0..1.0`.
    pub v: f64,
}

impl Color for Hsv {
    fn hex(&self) -> String {
        Rgb::from(*self).hex()
    }
}

impl Eq for Hsv {}

#[allow(clippy::derive_hash_xor_eq)]
impl std::hash::Hash for Hsv {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.h.to_bits().hash(state);
        self.s.to_bits().hash(state);
        self.v.to_bits().hash(state);
    }
}

//
// Implement to/from primitives
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

impl From<Hsv> for [f64; 3] {
    fn from(color: Hsv) -> Self {
        [color.h, color.s, color.v]
    }
}

//
// Implement From for all other Color types
//

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
            _ => unreachable!(),
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
// Implement to/from CssColorNotation
//

impl TryFrom<&css::CssColorNotation> for Hsv {
    type Error = css::Error;

    fn try_from(other: &css::CssColorNotation) -> css::Result<Self> {
        match other.format {
            css::CssColorType::Hsv | css::CssColorType::Hsva => Ok(Self {
                h: css::css_number_to_float(
                    other.values.get(0).ok_or(css::Error::InvalidCssParams)?,
                ) * 360.0,
                s: css::css_number_to_float(
                    other.values.get(1).ok_or(css::Error::InvalidCssParams)?,
                ),
                v: css::css_number_to_float(
                    other.values.get(2).ok_or(css::Error::InvalidCssParams)?,
                ),
            }),
            _ => Err(css::Error::WrongCssFormat),
        }
    }
}

impl From<Hsv> for css::CssColorNotation {
    fn from(other: Hsv) -> Self {
        Self {
            format: css::CssColorType::Hsv,
            values: vec![
                css::CssNumber::Float(other.h),
                css::CssNumber::Percent(other.s),
                css::CssNumber::Percent(other.v),
            ],
        }
    }
}
