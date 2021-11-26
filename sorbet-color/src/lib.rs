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
    pub fn rgba(&self) -> Self {
        match self {
            Color::Rgba {..} => *self,
            Color::Hsva {h, s, v, a} => {
                Color::Rgba {r: 0.0, g: 0.0, b: 0.0, a: *a}
            },
            Color::Hsla {h, s, l, a} => {
                Color::Rgba {r: 0.0, g: 0.0, b: 0.0, a: *a}
            },
            Color::Hsia {h, s, i, a} => {
                Color::Rgba {r: 0.0, g: 0.0, b: 0.0, a: *a}
            },
        }
    }

    pub fn hsva(&self) -> Self {
        match self {
            Color::Rgba {r, g, b, a} => {
                Color::Hsva {h: 0.0, s: 0.0, v: 0.0, a: *a}
            },
            Color::Hsva {..} => *self,
            Color::Hsla {h, s, l, a} => {
                Color::Hsva {h: 0.0, s: 0.0, v: 0.0, a: *a}
            },
            Color::Hsia {h, s, i, a} => {
                Color::Hsva {h: 0.0, s: 0.0, v: 0.0, a: *a}
            }
        }
    }

    pub fn hsla(&self) -> Self {
        match self {
            Color::Rgba {r, g, b, a} => {
                Color::Hsla {h: 0.0, s: 0.0, l: 0.0, a: *a}
            },
            Color::Hsva {h, s, v, a} => {
                Color::Hsla {h: 0.0, s: 0.0, l: 0.0, a: *a}
            },
            Color::Hsla {..} => *self,
            Color::Hsia {h, s, i, a} => {
                Color::Hsla {h: 0.0, s: 0.0, l: 0.0, a: *a}
            },
        }
    }

    pub fn hsia(&self) -> Self {
        // https://en.wikipedia.org/wiki/HSL_and_HSV#HSI_to_RGB
        match self {
            Color::Rgba {r, g, b, a} => {
                Color::Hsia {h: 0.0, s: 0.0, i: 0.0, a: *a}
            },
            Color::Hsva {h, s, v, a} => {
                Color::Hsia {h: 0.0, s: 0.0, i: 0.0, a: *a}
            },
            Color::Hsla {h, s, l, a} => {
                Color::Hsia {h: 0.0, s: 0.0, i: 0.0, a: *a}
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
