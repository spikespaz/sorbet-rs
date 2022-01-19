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
    use super::Color;
    use once_cell::sync::Lazy;

    static TEST_COLORS: Lazy<Vec<&str>> = Lazy::new(|| {
        "#353B48, #666666, #444852, #FCFCFC, #434343, #90939B, #353537, #2B303B, #B6B8C0, #241F31, #303440, #000000, #9398A2, #DFDFDF, #F0F1F2, #CFCFCF, #D3D8E2, #505666, #808080, #8A939F, #282B36, #AFB8C6, #383838, #4DADD4, #353A48, #838383, #202229, #7A7F8A, #7A7F8B, #2E3340, #70788D, #66A1DC, #17191F, #D7D7D7, #545860, #39404D, #161A26, #BE3841, #3C4049, #2F3A42, #F0F2F5, #4E4EFF, #262934, #1D1F26, #404552, #353945, #383C45, #8F939D, #F7EF45, #A4AAB7, #B2CDF1, #444A58, #BAC3CF, #FF00FF, #F46067, #5C6070, #C7CACF, #525762, #FF0B00, #323644, #F75A61, #464646, #ECEDF0, #171717, #E01B24, #1B1B1B, #797D87, #15171C, #8C919D, #4D4F52, #5B627B, #728495, #454C5C, #4080FB, #E2E2E2, #D1D3DA, #C0E3FF, #3580E4, #B7C0D3, #232428, #2D323F, #6E6E6E, #DCDCDC, #B9BCC2, #CC575D, #A1A1A1, #52555E, #353A47, #7C818C, #979DAC, #2F343F, #DDE3E9, #828282, #C5DCF7, #001AFF, #722563, #AFB8C5, #222529, #8ABFDD, #666A74, #F68086, #EDF5FB, #4B5162, #A9ACB2, #786613, #C7C7C7, #EEEFF1, #2B2E37, #F70505, #292C36, #3E434F, #5C616C, #F57900, #2D303B, #F5F6F7, #5F697F, #2E3436, #808791, #F08437, #CBD2E3, #E5A50A, #EEEEEE, #252932, #E7E8EB, #3E4350, #FF1111, #EF2929, #FC4138, #FCFDFD, #7A7A7A, #21242B, #BEBEBE, #FFFFFF, #252A35, #5252FF, #767B87, #535353, #3E3E3E, #AA5555, #5F6578, #C4C7CC, #383C4A, #102B68, #21252B, #F3AF0B, #CFD6E6, #D7787D, #FF7A80, #FDFDFD, #398DD3, #A51D2D, #73D216, #F8F8F9, #262932, #2F343B, #2B2E39, #2D3036, #F04A50, #006098, #3F4453, #AD4242, #1B1C21, #B9BFCE, #FF1616, #E5E5E5, #ED686F, #EAEBED, #FBFCFC, #398CD3, #262933, #5294E2, #0000FF, #D7D8DD, #2B2F3B, #F13039, #999999, #1F1F1F, #50DBB5, #525252, #FF2121, #F27835, #91949C, #ADAFB5, #3B3C3E, #D3D4D8, #525D76, #434652, #CACACA, #2D323D, #F9FAFB, #617C95, #EDEDED, #1A1A1A, #D8354A, #90949E, #313541, #A8A8A8, #DBDFE3, #CECECE, #0F0F0F, #1D242A, #B8BABF, #0F1116, #EEF4FC, #E2E7EF, #D3DAE3".split(", ").collect()
    });

    #[test]
    fn test_default_constructor() {
        for color in TEST_COLORS.iter() {
            Color::new(color);
        }
    }

    #[test]
    fn test_to_from_hsl() {
        for color in TEST_COLORS.iter() {
            let mutated = &Color::new(color).to_hsla().to_hex();

            if mutated == color {
                println!("{} == {}", mutated, color);
            } else {
                assert_eq!(mutated, color);
            }
        }
    }
}
