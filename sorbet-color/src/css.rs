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

//! This module contains functions and types relating to the parsing of CSS colors
//! in functional notation. This module typically will not be used directly.
//! See the [`crate::Color::new`] constructor for more information.

use std::str::FromStr;

use thiserror::Error;

/// Variants of this enum are used when the [`crate::Color::new`] constructor fails to parse an input string.
/// View the source code for the descriptions of these variants.
#[allow(missing_docs)]
#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error(
        "the input string was prefixed with a pound but was not either six or eight characters"
    )]
    InvalidHexLength,
    #[error("the input string was prefixed with a pound but had characters outside of hexadecimal range")]
    InvalidHexChars,
    #[error("the input string began with a format identifier but was missing parenthesis")]
    MissingCssParens,
    #[error(
        "the input string had a segment that was assumed to be an integer but failed to parse"
    )]
    InvalidCssFloat,
    #[error("the input string had a number that ended with a percent symbol but failed to parse as a float")]
    InvalidCssPercent,
    #[error("the input string was assumed to be CSS functional notation but did not the correct number of values")]
    InvalidCssParams,
    #[error("tried to parse into a color structure but failed because the input string had the wrong format")]
    WrongCssFormat,
    #[error("the input string had a prefix indicating a format that is not supported")]
    UnknownCssFormat,
}

/// The [`std::result::Result`] alias returned from parsing operations from this module.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents a number from a color channel parsed from CSS functional notation.
/// An integer and a float type will both be parsed as a float in this case,
/// because the value ranges are going to be the same, just with different precisions.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CssNumber {
    /// When a CSS value ends with a `%` symbol, that character will be removed and the
    /// result parsed as a float. To make the result easier to use as a multiplier, when parsed
    /// it will be divided by `100.0`.
    ///
    /// The internal [`f64`] will have a range of `0.0..1.0` but this
    /// is not enforced. Callee-code is responsible for either truncating or wrapping.
    Percent(f64),
    /// When a CSS value is either an integer or a float, it will be parsed as a float.
    ///
    /// The range here would be `0.0..255.0` for RGB values,
    /// however if an HSL color is represented as CSS the range would
    /// instead be `0.0..360.0`.
    Float(f64),
}

/// This enumerable represents the names of the CSS color functions supported by the crate.
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, strum::EnumString, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum CssColorType {
    Rgb,
    Rgba,
    Hsv,
    Hsva,
    Hsl,
    Hsla,
}

/// This structure is what CSS color functions will be parsed into.
/// It is an intermediate step between the CSS string and, for example, [`crate::Rgba`].
#[derive(Clone, Debug, PartialEq)]
pub struct CssColorNotation {
    /// See the documentation on the type itself.
    pub format: CssColorType,
    /// The length of this vector is going to be either three or four, depending on
    /// whether or not the color format has an alpha/transparency channel.
    ///
    /// These values are either going to be a percentage/multiplier,
    /// or a float with an undefined range.
    /// See the documentation on the type itself.
    pub values: Vec<CssNumber>,
}

/// With [`ToString`] and [`std::fmt::Display`], [`float_to_nice_string`] is used internally.
/// See the documentation for that function to see the representation that you will receive.
///
/// When the value is a [`CssNumber::Percent`] you will receive a number in the range `0.0..100.0`.
impl std::fmt::Display for CssNumber {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::Percent(percent) => {
                formatter.write_fmt(format_args!("{}%", float_to_nice_string(percent * 100.0)))
            }
            Self::Float(float) => formatter.write_str(&float_to_nice_string(float)),
        }
    }
}

impl FromStr for CssNumber {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self> {
        Ok(if let Some(string) = string.strip_suffix('%') {
            Self::Percent(string.parse::<f64>().or(Err(Error::InvalidCssPercent))? / 100.0)
        } else {
            Self::Float(string.parse().or(Err(Error::InvalidCssFloat))?)
        })
    }
}

impl std::fmt::Display for CssColorNotation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_fmt(format_args!(
            "{}({})",
            self.format,
            self.values
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        ))
    }
}

impl FromStr for CssColorNotation {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self> {
        let string = string.replace(' ', "");
        let (format, mut values) = string.split_once('(').ok_or(Error::MissingCssParens)?;
        values = values.strip_suffix(')').ok_or(Error::MissingCssParens)?;

        let format = CssColorType::from_str(format).or(Err(Error::UnknownCssFormat))?;
        let values = values
            .split(',')
            .map(CssNumber::from_str)
            .collect::<Result<Vec<_>>>()?;

        if values.len()
            != match format {
                CssColorType::Rgb | CssColorType::Hsv | CssColorType::Hsl => 3,
                CssColorType::Rgba | CssColorType::Hsva | CssColorType::Hsla => 4,
            }
        {
            Err(Error::InvalidCssParams)
        } else {
            Ok(Self { format, values })
        }
    }
}

/// Converts a float to a display string with three decimal places,
/// if the rounded decimal is zero, it is truncated entirely.
pub fn float_to_nice_string(float: f64) -> String {
    let mut string = format!("{:.3}", float);
    string.truncate(string.trim_end_matches('0').trim_end_matches('.').len());
    string
}

pub(crate) fn css_number_to_rgb_channel(number: &CssNumber) -> f64 {
    match *number {
        CssNumber::Percent(percent) => percent,
        CssNumber::Float(float) => float / 255.0,
    }
}

pub(crate) fn css_number_to_float(number: &CssNumber) -> f64 {
    match *number {
        CssNumber::Percent(percent) => percent,
        CssNumber::Float(float) => float,
    }
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;
    use test_case::test_case;

    use super::*;

    // Demonstrates that the decimal will be removed if the float is rational
    #[test_case(99.0 => "99")]
    // Demonstrates that the decimal will be preserved the float is irrational
    #[test_case(99.9 => "99.9")]
    // Demonstrates that decimals up to the thousandths will be preserved
    #[test_case(99.999 => "99.999")]
    // Demonstrates that decimals with a higher precision than thousandths will be rounded
    #[test_case(99.9994 => "99.999")]
    // Ignored due to a floating-point rounding error (should round up)
    #[test_case(99.9995 => ignore "100")]
    // This is the next value in the thousandths place that does not have the rounding error
    #[test_case(99.9996 => "100")]
    fn test_float_to_nice_string(float: f64) -> String {
        float_to_nice_string(float)
    }

    // Demonstrates that parsing numbers as float works
    #[test_case("99" => CssNumber::Float(99.0))]
    #[test_case("101.1" => CssNumber::Float(101.1))]
    // Demonstrates that zeros before the decimal are not trimmed
    #[test_case("100" => CssNumber::Float(100.0))]
    // Demonstrates that parsing numbers as percent works
    #[test_case("99%" => CssNumber::Percent(0.99))]
    // Demonstrates that the hundredths decimal is preserved
    #[test_case("99.5%" => CssNumber::Percent(0.995))]
    // Ignored because this causes a rounding error and that is inconsequential
    #[test_case("99.9%" => ignore CssNumber::Percent(0.999))]
    fn test_parse_css_number(string: &str) -> CssNumber {
        string.parse::<CssNumber>().unwrap()
    }

    // Repeat tests from [`test_float_to_nice_string`]
    #[test_case(&CssNumber::Float(99.0) => "99")]
    #[test_case(&CssNumber::Float(99.9) => "99.9")]
    #[test_case(&CssNumber::Float(99.999) => "99.999")]
    #[test_case(&CssNumber::Float(99.9994) => "99.999")]
    #[test_case(&CssNumber::Float(99.9995) => ignore "100")]
    #[test_case(&CssNumber::Float(99.9996) => "100")]
    // Repeat the same tests but for percents
    #[test_case(&CssNumber::Percent(0.990) => "99%")]
    #[test_case(&CssNumber::Percent(0.999) => "99.9%")]
    #[test_case(&CssNumber::Percent(0.99999) => "99.999%")]
    #[test_case(&CssNumber::Percent(0.999994) => "99.999%")]
    #[test_case(&CssNumber::Percent(0.999995) => ignore "100%")]
    #[test_case(&CssNumber::Percent(0.999996) => "100%")]
    fn test_display_css_number(number: &CssNumber) -> String {
        number.to_string()
    }

    static CSS_COLOR_NOTATIONS: Lazy<Vec<(&str, CssColorNotation)>> = Lazy::new(|| {
        vec![
            (
                // 0: Rational and irrational floats
                "rgb(127.5, 255, 0)",
                CssColorNotation {
                    format: CssColorType::Rgb,
                    values: vec![
                        CssNumber::Float(127.5),
                        CssNumber::Float(255.0),
                        CssNumber::Float(0.0),
                    ],
                },
            ),
            (
                // 1: Rational and irrational percent values
                "rgb(100%, 50%, 75.5%)",
                CssColorNotation {
                    format: CssColorType::Rgb,
                    values: vec![
                        CssNumber::Percent(1.0),
                        CssNumber::Percent(0.5),
                        CssNumber::Percent(0.755),
                    ],
                },
            ),
            (
                // 2: Floats and percent values
                "rgb(127.5, 120, 95%)",
                CssColorNotation {
                    format: CssColorType::Rgb,
                    values: vec![
                        CssNumber::Float(127.5),
                        CssNumber::Float(120.0),
                        CssNumber::Percent(0.95),
                    ],
                },
            ),
            (
                // 3: RGBA with alpha as a percent value
                "rgba(127.5, 120, 95%, 30%)",
                CssColorNotation {
                    format: CssColorType::Rgba,
                    values: vec![
                        CssNumber::Float(127.5),
                        CssNumber::Float(120.0),
                        CssNumber::Percent(0.95),
                        CssNumber::Percent(0.3),
                    ],
                },
            ),
            (
                // 4: RGBA with alpha as a float
                "rgba(127.5, 120, 95%, 0.3)",
                CssColorNotation {
                    format: CssColorType::Rgba,
                    values: vec![
                        CssNumber::Float(127.5),
                        CssNumber::Float(120.0),
                        CssNumber::Percent(0.95),
                        CssNumber::Float(0.3),
                    ],
                },
            ),
            (
                // 5: HSVA with maximum hue
                "hsva(360, 30%, 60%, 0.7)",
                CssColorNotation {
                    format: CssColorType::Hsva,
                    values: vec![
                        CssNumber::Float(360.0),
                        CssNumber::Percent(0.3),
                        CssNumber::Percent(0.6),
                        CssNumber::Float(0.7),
                    ],
                },
            ),
            (
                // 6: HSLA with hue as an irrational float
                "hsla(240.5, 30%, 60%, 0.7)",
                CssColorNotation {
                    format: CssColorType::Hsla,
                    values: vec![
                        CssNumber::Float(240.5),
                        CssNumber::Percent(0.3),
                        CssNumber::Percent(0.6),
                        CssNumber::Float(0.7),
                    ],
                },
            ),
        ]
    });

    // Tests for parsing [`CssColorNotation`] from a string
    #[test_case(CSS_COLOR_NOTATIONS[0].0 => CSS_COLOR_NOTATIONS[0].1)]
    #[test_case(CSS_COLOR_NOTATIONS[1].0 => CSS_COLOR_NOTATIONS[1].1)]
    #[test_case(CSS_COLOR_NOTATIONS[2].0 => CSS_COLOR_NOTATIONS[2].1)]
    #[test_case(CSS_COLOR_NOTATIONS[3].0 => CSS_COLOR_NOTATIONS[3].1)]
    #[test_case(CSS_COLOR_NOTATIONS[4].0 => CSS_COLOR_NOTATIONS[4].1)]
    #[test_case(CSS_COLOR_NOTATIONS[5].0 => CSS_COLOR_NOTATIONS[5].1)]
    #[test_case(CSS_COLOR_NOTATIONS[6].0 => CSS_COLOR_NOTATIONS[6].1)]
    fn test_parse_css_color_notation(string: &str) -> CssColorNotation {
        string.parse::<CssColorNotation>().unwrap()
    }

    // Tests for [`std::fmt::Display`] and [`ToString`] for [`CssColorNotation`]
    #[test_case(&CSS_COLOR_NOTATIONS[0].1 => CSS_COLOR_NOTATIONS[0].0)]
    #[test_case(&CSS_COLOR_NOTATIONS[1].1 => CSS_COLOR_NOTATIONS[1].0)]
    #[test_case(&CSS_COLOR_NOTATIONS[2].1 => CSS_COLOR_NOTATIONS[2].0)]
    #[test_case(&CSS_COLOR_NOTATIONS[3].1 => CSS_COLOR_NOTATIONS[3].0)]
    #[test_case(&CSS_COLOR_NOTATIONS[4].1 => CSS_COLOR_NOTATIONS[4].0)]
    #[test_case(&CSS_COLOR_NOTATIONS[5].1 => CSS_COLOR_NOTATIONS[5].0)]
    #[test_case(&CSS_COLOR_NOTATIONS[6].1 => CSS_COLOR_NOTATIONS[6].0)]
    fn test_display_css_color_notation(color: &CssColorNotation) -> String {
        color.to_string()
    }
}
