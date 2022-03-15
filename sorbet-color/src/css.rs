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
#[derive(Clone, Debug)]
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
        formatter.write_str(&match *self {
            Self::Percent(percent) => float_to_nice_string(percent * 100.0),
            Self::Float(float) => float_to_nice_string(float),
        })
    }
}

impl FromStr for CssNumber {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self> {
        Ok(if let Some(string) = string.strip_suffix('%') {
            Self::Percent(f64::clamp(
                string.parse::<f64>().or(Err(Error::InvalidCssPercent))? / 100.0,
                0.0,
                1.0,
            ))
        } else {
            Self::Float(f64::clamp(
                string.parse().or(Err(Error::InvalidCssFloat))?,
                0.0,
                1.0,
            ))
        })
    }
}

impl std::fmt::Display for CssColorNotation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(&format!(
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
