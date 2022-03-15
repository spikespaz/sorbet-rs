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

use std::{fmt, hash};

use crate::{css, types::*};

/// This structure represents colors in the RGB color space with
/// red, green, and blue channels.
/// See the [Wikipedia reference](<https://en.wikipedia.org/wiki/RGB_color_model>) for details.
///
/// This does not include the alpha/transparency component.
/// If you need transparency, see [`crate::types::Rgba`].
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rgb {
    /// Red channel.
    /// Ranged `0.0..1.0`.
    pub r: f64,
    /// Green channel.
    /// Ranged `0.0..1.0`.
    pub g: f64,
    /// Blue channel.
    /// Ranged `0.0..1.0`.
    pub b: f64,
}

impl Eq for Rgb {}

#[allow(clippy::derive_hash_xor_eq)]
impl hash::Hash for Rgb {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.r.to_bits().hash(state);
        self.g.to_bits().hash(state);
        self.b.to_bits().hash(state);
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "#{:06X}", u32::from(*self) >> 8)
    }
}

//
// Implement to/from primitives
//

impl From<[u8; 3]> for Rgb {
    fn from(array: [u8; 3]) -> Self {
        Self {
            r: array[0] as f64 / 255.0,
            g: array[1] as f64 / 255.0,
            b: array[2] as f64 / 255.0,
        }
    }
}

impl From<Rgb> for [u8; 3] {
    fn from(color: Rgb) -> Self {
        [
            (color.r * 255.0).round() as u8,
            (color.g * 255.0).round() as u8,
            (color.b * 255.0).round() as u8,
        ]
    }
}

impl From<u32> for Rgb {
    fn from(int: u32) -> Self {
        let r = (int >> 24) as u8;
        let g = (int >> 16) as u8;
        let b = (int >> 8) as u8;

        Self::from([r, g, b])
    }
}

impl From<Rgb> for u32 {
    fn from(color: Rgb) -> u32 {
        let [r, g, b]: [u8; 3] = color.into();

        ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8)
    }
}

impl From<[f64; 3]> for Rgb {
    fn from(array: [f64; 3]) -> Self {
        Self {
            r: array[0],
            g: array[1],
            b: array[2],
        }
    }
}

impl From<Rgb> for [f64; 3] {
    fn from(color: Rgb) -> Self {
        [color.r, color.g, color.b]
    }
}

impl From<&str> for Rgb {
    /// Takes a hexadecimal string optionally prefixed with a `#` and returns an [`Rgb`] structure.
    /// The input is unchecked and will panic if the input has invalid hexadecimal characters.
    /// For a more robust constructor that won't panic, see [`crate::Color::new`].
    fn from(string: &str) -> Self {
        let string = string.strip_prefix('#').unwrap_or(string);

        let r = u8::from_str_radix(&string[0..2], 16).expect("invalid hexadecimal string");
        let g = u8::from_str_radix(&string[2..4], 16).expect("invalid hexadecimal string");
        let b = u8::from_str_radix(&string[4..6], 16).expect("invalid hexadecimal string");

        Self {
            r: r as f64 / 255.0,
            g: g as f64 / 255.0,
            b: b as f64 / 255.0,
        }
    }
}

//
// Implement From for all other Color types
//

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
// Implement to/from CssColorNotation
//

impl TryFrom<&css::CssColorNotation> for Rgb {
    type Error = css::Error;

    fn try_from(other: &css::CssColorNotation) -> css::Result<Self> {
        match other.format {
            css::CssColorType::Rgb | css::CssColorType::Rgba => Ok(Self {
                r: css::css_number_to_rgb_channel(
                    other.values.get(0).ok_or(css::Error::InvalidCssParams)?,
                ),
                g: css::css_number_to_rgb_channel(
                    other.values.get(1).ok_or(css::Error::InvalidCssParams)?,
                ),
                b: css::css_number_to_rgb_channel(
                    other.values.get(2).ok_or(css::Error::InvalidCssParams)?,
                ),
            }),
            _ => Err(css::Error::WrongCssFormat),
        }
    }
}

impl From<Rgb> for css::CssColorNotation {
    fn from(other: Rgb) -> Self {
        Self {
            format: css::CssColorType::Rgb,
            values: vec![
                css::CssNumber::Float(other.r * 255.0),
                css::CssNumber::Float(other.g * 255.0),
                css::CssNumber::Float(other.b * 255.0),
            ],
        }
    }
}

//
// Implement to/from wgpu::Color
//

#[cfg(feature = "wgpu")]
impl From<wgpu::Color> for Rgb {
    fn from(other: wgpu::Color) -> Rgb {
        Rgb::from([other.r, other.g, other.b])
    }
}

#[cfg(feature = "wgpu")]
impl From<Rgb> for wgpu::Color {
    fn from(other: Rgb) -> wgpu::Color {
        wgpu::Color {
            r: other.r,
            g: other.g,
            b: other.b,
            a: 1.0,
        }
    }
}

//
// Math helpers
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
