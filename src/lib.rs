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

    // pub fn to_rgba(&self) -> Self {
    //     match self {
    //         Color::Rgba { .. } => *self,
    //         Color::Hsva { h, s, v, a } => {
    //         }
    //         Color::Hsla { h, s, l, a } => {
    //         }
    //     }
    // }

    // pub fn to_hsva(&self) -> Self {
    //     match self {
    //         Color::Rgba { r, g, b, a } => {
    //         }
    //         Color::Hsva { .. } => *self,
    //         Color::Hsla { h, s, l, a } => {
    //         }
    //     }
    // }

    // pub fn to_hsla(&self) -> Self {
    //     match self {
    //         Color::Rgba { r, g, b, a } => {
    //         }
    //         Color::Hsva { h, s, v, a } => {
    //         }
    //         Color::Hsla { .. } => *self,
    //     }
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
