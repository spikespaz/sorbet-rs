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

#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod css;
pub mod named;
pub mod types;

pub use types::*;

/// This trait marks structures that have the necessary [`From`] implementations for all other
/// color spaces. It also provides several constructors that facilitate creating the appropriate
/// values from known-formats such as hexadecimal notation or CSS-compatible functional notations.
pub trait Color:
    Copy
    + Clone
    + std::fmt::Debug
    + PartialEq
    + Eq
    + std::hash::Hash
    + From<Rgb>
    + From<Rgba>
    + From<Hsv>
    + From<Hsva>
    + From<Hsl>
    + From<Hsla>
    + Into<Rgb>
    + Into<Rgba>
    + Into<Hsv>
    + Into<Hsva>
    + Into<Hsl>
    + Into<Hsla>
{
    /// This constructor takes a CSS-compatible functional notation for a color, and coerces it to an
    /// explicit or inferred type. This will return [`css::Error`] variants if the parsing fails.
    ///
    /// Spaces are ignored but other whitespace is not.
    /// When providing a hexadecimal color, the `#` prefix is required, whereas the unchecked
    /// [`From<&str>`] on [`Rgb`] and [`Rgba`] has no such restriction.
    ///
    /// ~~Note that if any parameters inside the string are not within a channel's valid range,
    /// they will be clamped instead of wrapped.~~
    /// Numbers parsing to values outside the acceptable range for the fields of a color type
    /// may result in undefined behavior.
    ///
    /// See the [reference on W3 Schools](https://www.w3schools.com/cssref/css_colors_legal.asp)
    /// for valid input strings. Current supported prefixes match the type names for color structures
    /// supported by this crate.
    fn new<S>(string: S) -> css::Result<Self>
    where
        S: AsRef<str>,
    {
        let string = string.as_ref().replace(' ', "").to_ascii_lowercase();

        if let Some(string) = string.strip_prefix('#') {
            Self::from_hex(string)
        } else {
            // Here we don't just parse the string and use the [`Self::TryFrom`] implementation
            // directly because that may use the wrong one and throw an error.
            // For example, when the string is parsed as [`css::CssColorType::Rgb`] and `Self`
            // is [`Hsv`], there would be an error because [`Hsv`] doesn't have the ability to
            // take [`css::CssColorType::Rgb`].

            let interm = string.parse::<css::CssColorNotation>()?;

            Ok(match interm.format {
                css::CssColorType::Rgb => Rgb::try_from(&interm)?.into(),
                css::CssColorType::Rgba => Rgba::try_from(&interm)?.into(),
                css::CssColorType::Hsv => Hsv::try_from(&interm)?.into(),
                css::CssColorType::Hsva => Hsva::try_from(&interm)?.into(),
                css::CssColorType::Hsl => Hsl::try_from(&interm)?.into(),
                css::CssColorType::Hsla => Hsla::try_from(&interm)?.into(),
            })
        }
    }

    /// Takes a hexadecimal-encoded RGB or RGBA string,
    /// and coerces to an explicit or inferred color type.
    /// With this constructor the `#` prefix is optional, but this expects no whitespace.
    ///
    /// If you want an [`Rgb`] or [`Rgba`] type, it is recommended to use either
    /// `Rgb::from(string)` or `Rgba::from(string)`, which do not return a [`css::Result`]
    /// and will simply panic on invalid input.
    fn from_hex<S>(string: S) -> css::Result<Self>
    where
        S: AsRef<str>,
    {
        let mut string = string.as_ref();
        string = string.strip_prefix('#').unwrap_or(string);

        if !string.bytes().all(|b| b.is_ascii_hexdigit()) {
            Err(css::Error::InvalidHexChars)
        } else if string.len() == 6 {
            Ok(Rgb::from(string).into())
        } else if string.len() == 8 {
            Ok(Rgba::from(string).into())
        } else {
            Err(css::Error::InvalidHexLength)
        }
    }

    /// This constructor takes an unsigned 32-bit integer and coerces it to an
    /// explicit or inferred type. This should be used when using color constants
    /// from the [`named`] module, and the signature is named accordingly.
    fn from_int(int: u32) -> Self {
        Rgba::from(int).into()
    }

    /// Provides a color as an RGB or RGBA-encoded hexadecimal string, prefixed with a `#` character.
    fn hex(&self) -> String;

    /// Provides an 32-bit integer, encoded from RGBA 8-bit values.
    /// Because in Rust endianness is platform dependant, the byte-order of this may be different
    /// depending on your system.
    ///
    /// On most platforms the encoded integer will be in ABGR32 format, where alpha is the lowest
    /// byte and red is the highest.
    ///
    /// See [RGBA color model representation](https://en.wikipedia.org/wiki/RGBA_color_model#Representation)
    /// on Wikipedia.
    ///
    /// If you have an [`Rgb`] or [`Rgba`] type, prefer the `From<...> for u32` implementations
    /// on those types.
    fn int(&self) -> u32 {
        Into::<Rgba>::into(*self).into()
    }

    /// Returns a `[u8; 3]`  with red, green, and blue values as unsigned 8-bit integers.
    fn rgb_array(&self) -> [u8; 3] {
        Into::<Rgb>::into(*self).into()
    }

    /// Returns a `[u8; 4]`  with red, green, blue, and alpha values as unsigned 8-bit integers.
    fn rgba_array(&self) -> [u8; 4] {
        Into::<Rgba>::into(*self).into()
    }
}

macro_rules! impl_from_str_css {
    ( $( $t:ident, )+ ) => {
        impl_from_str_css!( $( $t ),* );
    };
    ( $( $t:ident ),+ ) => {
        $(
            /// Similar to [`Color::new`], this will create a color type from a valid CSS notation.
            /// The difference is, this can be used in a situation where you want to use
            /// [`str::parse`] instead. This will only work when the destination
            /// type matches the parsed [`css::CssColorType`] variant, otherwise a
            /// [`css::Error::WrongCssFormat`] is returned.
            impl ::std::str::FromStr for $t {
                type Err = $crate::css::Error;

                fn from_str(string: &str) -> $crate::css::Result<$t> {
                    $t::try_from(&string.parse::<$crate::css::CssColorNotation>()?)
                }
            }
        )*
    };
}

macro_rules! impl_display_css {
    ( $( $t:ident, )+ ) => {
        impl_display_css!( $( $t ),* );
    };
    ( $( $t:ident ),+ ) => {
        $(
            /// This implementation will convert a color type to a [`css::CssColorNotation`]
            /// and return a string from the resulting format.
            impl ::std::fmt::Display for $t {
                fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    formatter.write_str(&$crate::css::CssColorNotation::from(*self).to_string())
                }
            }
        )*
    };
}

impl_from_str_css!(Rgb, Rgba, Hsv, Hsva, Hsl, Hsla);
impl_display_css!(Rgb, Rgba, Hsv, Hsva, Hsl, Hsla);

// #[cfg(test)]
// mod tests {
//     use super::Color;
//     use once_cell::sync::Lazy;

//     static TEST_COLORS: Lazy<Vec<&str>> = Lazy::new(|| {
//         "#353B48, #666666, #444852, #FCFCFC, #434343, #90939B, #353537, #2B303B, #B6B8C0, #241F31, #303440, #000000, #9398A2, #DFDFDF, #F0F1F2, #CFCFCF, #D3D8E2, #505666, #808080, #8A939F, #282B36, #AFB8C6, #383838, #4DADD4, #353A48, #838383, #202229, #7A7F8A, #7A7F8B, #2E3340, #70788D, #66A1DC, #17191F, #D7D7D7, #545860, #39404D, #161A26, #BE3841, #3C4049, #2F3A42, #F0F2F5, #4E4EFF, #262934, #1D1F26, #404552, #353945, #383C45, #8F939D, #F7EF45, #A4AAB7, #B2CDF1, #444A58, #BAC3CF, #FF00FF, #F46067, #5C6070, #C7CACF, #525762, #FF0B00, #323644, #F75A61, #464646, #ECEDF0, #171717, #E01B24, #1B1B1B, #797D87, #15171C, #8C919D, #4D4F52, #5B627B, #728495, #454C5C, #4080FB, #E2E2E2, #D1D3DA, #C0E3FF, #3580E4, #B7C0D3, #232428, #2D323F, #6E6E6E, #DCDCDC, #B9BCC2, #CC575D, #A1A1A1, #52555E, #353A47, #7C818C, #979DAC, #2F343F, #DDE3E9, #828282, #C5DCF7, #001AFF, #722563, #AFB8C5, #222529, #8ABFDD, #666A74, #F68086, #EDF5FB, #4B5162, #A9ACB2, #786613, #C7C7C7, #EEEFF1, #2B2E37, #F70505, #292C36, #3E434F, #5C616C, #F57900, #2D303B, #F5F6F7, #5F697F, #2E3436, #808791, #F08437, #CBD2E3, #E5A50A, #EEEEEE, #252932, #E7E8EB, #3E4350, #FF1111, #EF2929, #FC4138, #FCFDFD, #7A7A7A, #21242B, #BEBEBE, #FFFFFF, #252A35, #5252FF, #767B87, #535353, #3E3E3E, #AA5555, #5F6578, #C4C7CC, #383C4A, #102B68, #21252B, #F3AF0B, #CFD6E6, #D7787D, #FF7A80, #FDFDFD, #398DD3, #A51D2D, #73D216, #F8F8F9, #262932, #2F343B, #2B2E39, #2D3036, #F04A50, #006098, #3F4453, #AD4242, #1B1C21, #B9BFCE, #FF1616, #E5E5E5, #ED686F, #EAEBED, #FBFCFC, #398CD3, #262933, #5294E2, #0000FF, #D7D8DD, #2B2F3B, #F13039, #999999, #1F1F1F, #50DBB5, #525252, #FF2121, #F27835, #91949C, #ADAFB5, #3B3C3E, #D3D4D8, #525D76, #434652, #CACACA, #2D323D, #F9FAFB, #617C95, #EDEDED, #1A1A1A, #D8354A, #90949E, #313541, #A8A8A8, #DBDFE3, #CECECE, #0F0F0F, #1D242A, #B8BABF, #0F1116, #EEF4FC, #E2E7EF, #D3DAE3".split(", ").collect()
//     });

//     #[test]
//     fn test_to_from_hsv() {
//         for color in TEST_COLORS.iter() {
//             let mutated = &Color::new(color).to_hsva().to_hex();

//             assert_eq!(mutated, color);
//         }
//     }

//     #[test]
//     fn test_to_from_hsl() {
//         for color in TEST_COLORS.iter() {
//             let mutated = &Color::new(color).to_hsla().to_hex();

//             assert_eq!(mutated, color);
//         }
//     }
// }
