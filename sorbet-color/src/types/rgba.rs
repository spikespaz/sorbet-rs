/*
 * Copyright 2022 Jacob Birkett
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::{css, types::*, Color};

/// This structure represents colors in the RGBA color space with
/// red, green, blue, and alpha channels..
/// See the [Wikipedia reference](<https://en.wikipedia.org/wiki/RGB_color_model>) for details.
///
/// If you don't need transparency, see [`crate::types::Rgb`].
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rgba {
    /// Red channel.
    /// Ranged `0.0..1.0`.
    pub r: f64,
    /// Green channel.
    /// Ranged `0.0..1.0`.
    pub g: f64,
    /// Blue channel.
    /// Ranged `0.0..1.0`.
    pub b: f64,
    /// Alpha/transparency channel.
    /// Ranged `0.0..1.0`.
    pub alpha: f64,
}

impl Color for Rgba {
    fn hex(&self) -> String {
        format!("#{:08X}", u32::from(*self))
    }
}

impl Eq for Rgba {}

#[allow(clippy::derive_hash_xor_eq)]
impl std::hash::Hash for Rgba {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.r.to_bits().hash(state);
        self.g.to_bits().hash(state);
        self.b.to_bits().hash(state);
        self.alpha.to_bits().hash(state);
    }
}

//
// Implement to/from primitives
//

impl From<[u8; 4]> for Rgba {
    fn from(array: [u8; 4]) -> Self {
        Self {
            r: array[0] as f64 / 255.0,
            g: array[1] as f64 / 255.0,
            b: array[2] as f64 / 255.0,
            alpha: array[3] as f64 / 255.0,
        }
    }
}

impl From<Rgba> for [u8; 4] {
    fn from(color: Rgba) -> Self {
        [
            (color.r * 255.0).round() as u8,
            (color.g * 255.0).round() as u8,
            (color.b * 255.0).round() as u8,
            (color.alpha * 255.0).round() as u8,
        ]
    }
}

impl From<u32> for Rgba {
    fn from(int: u32) -> Self {
        let r = (int >> 24) as u8;
        let g = (int >> 16) as u8;
        let b = (int >> 8) as u8;
        let alpha = int as u8;

        Self::from([r, g, b, alpha])
    }
}

impl From<Rgba> for u32 {
    fn from(color: Rgba) -> u32 {
        let [r, g, b, alpha]: [u8; 4] = color.into();

        ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (alpha as u32)
    }
}

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

impl From<Rgba> for [f64; 4] {
    fn from(color: Rgba) -> Self {
        [color.r, color.g, color.b, color.alpha]
    }
}

impl From<&str> for Rgba {
    /// Takes a hexadecimal string optionally prefixed with a `#` and returns an [`Rgba`] structure.
    /// The input is unchecked and will panic if the input has invalid hexadecimal characters.
    /// For a more robust constructor that won't panic, see [`crate::Color::new`].
    fn from(string: &str) -> Self {
        let string = string.strip_prefix('#').unwrap_or(string);

        const EXPECT_MSG: &str = "invalid hexadecimal string";

        let Rgb { r, g, b } = Rgb::from(string);
        let alpha = u8::from_str_radix(&string[6..8], 16).expect(EXPECT_MSG);

        Self {
            r: r as f64 / 255.0,
            g: g as f64 / 255.0,
            b: b as f64 / 255.0,
            alpha: alpha as f64 / 255.0,
        }
    }
}

//
// Implement From for all other Color types
//

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
// Implement to/from CssColorNotation
//

impl TryFrom<&css::CssColorNotation> for Rgba {
    type Error = css::Error;

    fn try_from(other: &css::CssColorNotation) -> css::Result<Self> {
        match other.format {
            css::CssColorType::Rgb => Ok(Self::from(Rgb::try_from(other)?)),
            css::CssColorType::Rgba => {
                let mut this = Self::from(Rgb::try_from(other)?);

                this.alpha = css::css_number_to_float(
                    other.values.get(3).ok_or(css::Error::InvalidCssParams)?,
                );

                Ok(this)
            }
            _ => Err(css::Error::WrongCssFormat),
        }
    }
}

impl From<Rgba> for css::CssColorNotation {
    fn from(other: Rgba) -> Self {
        Self {
            format: css::CssColorType::Rgba,
            values: vec![
                css::CssNumber::Float(other.r * 255.0),
                css::CssNumber::Float(other.g * 255.0),
                css::CssNumber::Float(other.b * 255.0),
                css::CssNumber::Percent(other.alpha),
            ],
        }
    }
}

//
// Implement to/from wgpu::Color
//

#[cfg(feature = "wgpu")]
impl From<wgpu::Color> for Rgba {
    fn from(other: wgpu::Color) -> Rgba {
        Rgba::from([other.r, other.g, other.b, other.a])
    }
}

#[cfg(feature = "wgpu")]
impl From<Rgba> for wgpu::Color {
    fn from(other: Rgba) -> wgpu::Color {
        wgpu::Color {
            r: other.r,
            g: other.g,
            b: other.b,
            a: other.alpha,
        }
    }
}
