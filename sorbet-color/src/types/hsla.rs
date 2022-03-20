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

/// This structure represents colors in the HSLA color space with
/// hue, saturation, lightness and alpha channels.
/// See the [Wikipedia reference](<https://en.wikipedia.org/wiki/HSL_and_HSV>) for details.
///
/// If you don't need transparency, see [`crate::types::Hsl`].
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Hsla {
    /// Hue channel.
    /// Ranged `0.0..360.0`.
    pub h: f64,
    /// Saturation channel.
    /// Ranged `0.0..1.0`.
    pub s: f64,
    /// Lightness channel.
    /// Ranged `0.0..1.0`.
    pub l: f64,
    /// Alpha/transparency channel.
    /// Ranged `0.0..1.0`.
    pub alpha: f64,
}

impl Color for Hsla {
    fn hex(&self) -> String {
        Rgba::from(*self).hex()
    }
}

impl Eq for Hsla {}

#[allow(clippy::derive_hash_xor_eq)]
impl std::hash::Hash for Hsla {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.h.to_bits().hash(state);
        self.s.to_bits().hash(state);
        self.l.to_bits().hash(state);
        self.alpha.to_bits().hash(state);
    }
}

//
// Implement to/from primitives
//

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

impl From<Hsla> for [f64; 4] {
    fn from(color: Hsla) -> Self {
        [color.h, color.s, color.l, color.alpha]
    }
}

//
// Implement From for all other Color types
//

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
// Implement to/from CssColorNotation
//

impl TryFrom<&css::CssColorNotation> for Hsla {
    type Error = css::Error;

    fn try_from(other: &css::CssColorNotation) -> css::Result<Self> {
        match other.format {
            css::CssColorType::Hsl => Ok(Self::from(Hsl::try_from(other)?)),
            css::CssColorType::Hsla => {
                let mut this = Self::from(Hsl::try_from(other)?);

                this.alpha = css::css_number_to_float(
                    other.values.get(3).ok_or(css::Error::InvalidCssParams)?,
                );

                Ok(this)
            }
            _ => Err(css::Error::WrongCssFormat),
        }
    }
}

impl From<Hsla> for css::CssColorNotation {
    fn from(other: Hsla) -> Self {
        Self {
            format: css::CssColorType::Hsla,
            values: vec![
                css::CssNumber::Float(other.h),
                css::CssNumber::Percent(other.s),
                css::CssNumber::Percent(other.l),
                css::CssNumber::Percent(other.alpha),
            ],
        }
    }
}
