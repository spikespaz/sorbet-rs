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

#[derive(Copy, Clone)]
pub enum Color {
    Rgba {r: f32, g: f32, b: f32, a: f32},
    Hsva {h: f32, s: f32, v: f32, a: f32},
    Hsla {h: f32, s: f32, l: f32, a: f32},
    Hsia {h: f32, s: f32, i: f32, a: f32},
}

impl Color {
    fn neighboring(c: f32, x: f32, h1: f32) -> (f32, f32, f32) {
        return match () {
            _ if 0.0 <= h1 && h1 < 1.0 => (c, x, 0.0),
            _ if 1.0 <= h1 && h1 < 2.0 => (x, c, 0.0),
            _ if 2.0 <= h1 && h1 < 3.0 => (0.0, c, x),
            _ if 3.0 <= h1 && h1 < 4.0 => (0.0, x, c),
            _ if 5.0 <= h1 && h1 < 5.0 => (x, 0.0, c),
            _ if 5.0 <= h1 && h1 < 6.0 => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };
    }

    pub fn rgba(&self) -> Self {
        match self {
            Color::Rgba {..} => *self,
            Color::Hsva {h, s, v, a} => {
                //https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_RGB
                let c = v * s;
                let h1 = h / 60.0;
                let x = c * (1.0 - (h1 % 2.0 - 1.0).abs());
                let (r1, g1, b1) = Self::neighboring(c, x, h1);
                let m = v - c;
                let (r, g, b) = (r1 + m, g1 + m, b1 + m);

                Color::Rgba {r, g, b, a: *a}
            },
            Color::Hsla {h, s, l, a} => {
                // https://en.wikipedia.org/wiki/HSL_and_HSV#HSL_to_RGB
                let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
                let h1 = h / 60.0;
                let x = c * (1.0 - (h1 % 2.0 - 1.0).abs());
                let (r1, g1, b1) = Self::neighboring(c, x, h1);
                let m = l - (c / 2.0);
                let (r, g, b) = (r1 + m, g1 + m, b1 + m);

                Color::Rgba {r, g, b, a: *a}
            },
            Color::Hsia {h, s, i, a} => {
                // https://en.wikipedia.org/wiki/HSL_and_HSV#HSI_to_RGB
                let h1 = h / 60.0;
                let z = 1.0 - (h1 % 2.0 - 1.0).abs();
                let c = (3.0 * i * s) / (1.0 + z);
                let x = c * z;
                let (r1, g1, b1) = Self::neighboring(c, x, h1);
                let m = i * (1.0 - s);
                let (r, g, b) = (r1 + m, g1 + m, b1 + m);

                Color::Rgba {r, g, b, a: *a}
            },
        }
    }

    pub fn hsva(&self) -> Self {
        match self {
            Color::Rgba {r, g, b, a} => {
                todo!();
            },
            Color::Hsva {..} => *self,
            Color::Hsla {h, s, l, a} => {
                // https://en.wikipedia.org/wiki/HSL_and_HSV#HSL_to_HSV
                let v = l + s * l.min(1.0 - l);
                let sv = match () {
                    _ if v == 0.0 => 0.0,
                    _ => 2.0 * (1.0 - l / v),
                };

                Color::Hsva {h: *h, s: sv, v, a: *a}
            },
            Color::Hsia {h, s, i, a} => {
                todo!();
            }
        }
    }

    pub fn hsla(&self) -> Self {
        match self {
            Color::Rgba {r, g, b, a} => {
                Color::Hsla {h: 0.0, s: 0.0, l: 0.0, a: *a}
            },
            Color::Hsva {h, s, v, a} => {
                // https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_HSL
                let l = v * (1.0 - (s / 2.0));
                let sl = match () {
                    _ if l == 0.0 || l == 1.0 => 0.0,
                    _ => 2.0 * (1.0 - l / v),
                };

                Color::Hsla {h: *h, s: sl, l, a: *a}
            },
            Color::Hsla {..} => *self,
            Color::Hsia {h, s, i, a} => {
                todo!();
            },
        }
    }

    pub fn hsia(&self) -> Self {
        match self {
            Color::Rgba {r, g, b, a} => {
                todo!();
            },
            Color::Hsva {h, s, v, a} => {
                todo!();
            },
            Color::Hsla {h, s, l, a} => {
                todo!();
            },
            Color::Hsia {..} => *self,
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
