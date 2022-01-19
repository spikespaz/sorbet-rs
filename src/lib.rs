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

pub mod types;

use types::*;

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Rgba { inner: Rgb, alpha: Option<f64> },
    Hsva { inner: Hsv, alpha: Option<f64> },
    Hsla { inner: Hsl, alpha: Option<f64> },
}

impl Color {
    pub fn new(color: &str) -> Self {
        let color = match color.strip_prefix('#') {
            Some(string) => string,
            None => color,
        };

        let r = i16::from_str_radix(&color[0..2], 16).expect("invalid hexadecimal string");
        let g = i16::from_str_radix(&color[2..4], 16).expect("invalid hexadecimal string");
        let b = i16::from_str_radix(&color[4..6], 16).expect("invalid hexadecimal string");

        let alpha = if color.len() == 8 {
            Some(i16::from_str_radix(&color[6..8], 16).expect("invalid hexadecimal string"))
        } else {
            None
        };

        Self::Rgba {
            inner: Rgb {
                r: r as f64 / 255.0,
                g: g as f64 / 255.0,
                b: b as f64 / 255.0,
            },
            alpha: alpha.map(|a| a as f64 / 255.0),
        }
    }

    pub fn to_hex(&self) -> String {
        match self {
            Self::Rgba { inner, alpha } => {
                let r = (inner.r * 255.0) as i16;
                let g = (inner.g * 255.0) as i16;
                let b = (inner.b * 255.0) as i16;
                let a = alpha.map(|a| (a * 255.0) as i16);

                match a {
                    Some(a) => format!("#{:X}{:X}{:X}{:X}", r, g, b, a),
                    None => format!("#{:X}{:X}{:X}", r, g, b),
                }
            }
            _ => self.to_rgba().to_hex(),
        }
    }

    pub fn new_rgba<I: IntoIterator<Item = f64>>(color: I) -> Self {
        let mut color = color.into_iter();

        Self::Rgba {
            inner: Rgb {
                r: color.next().expect("could not find R value"),
                g: color.next().expect("could not find G value"),
                b: color.next().expect("could not find B value"),
            },
            alpha: color.next(),
        }
    }

    pub fn new_hsva<I: IntoIterator<Item = f64>>(color: I) -> Self {
        let mut color = color.into_iter();

        Self::Hsva {
            inner: Hsv {
                h: color.next().expect("could not find H value"),
                s: color.next().expect("could not find S value"),
                v: color.next().expect("could not find V value"),
            },
            alpha: color.next(),
        }
    }

    pub fn new_hsla<I: IntoIterator<Item = f64>>(color: I) -> Self {
        let mut color = color.into_iter();

        Self::Hsla {
            inner: Hsl {
                h: color.next().expect("could not find H value"),
                s: color.next().expect("could not find S value"),
                l: color.next().expect("could not find L value"),
            },
            alpha: color.next(),
        }
    }

    pub fn to_rgba(&self) -> Self {
        match self {
            Color::Rgba { .. } => *self,
            Color::Hsva { inner, alpha } => Color::Rgba {
                inner: inner.to_rgb(),
                alpha: *alpha,
            },
            Color::Hsla { inner, alpha } => Color::Rgba {
                inner: inner.to_rgb(),
                alpha: *alpha,
            },
        }
    }

    pub fn to_hsva(&self) -> Self {
        match self {
            Color::Rgba { inner, alpha } => Color::Hsva {
                inner: inner.to_hsv(),
                alpha: *alpha,
            },
            Color::Hsva { .. } => *self,
            Color::Hsla { inner, alpha } => Color::Hsva {
                inner: inner.to_hsv(),
                alpha: *alpha,
            },
        }
    }

    pub fn to_hsla(&self) -> Self {
        match self {
            Color::Rgba { inner, alpha } => Color::Hsla {
                inner: inner.to_hsl(),
                alpha: *alpha,
            },
            Color::Hsva { inner, alpha } => Color::Hsla {
                inner: inner.to_hsl(),
                alpha: *alpha,
            },
            Color::Hsla { .. } => *self,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
