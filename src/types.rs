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

use std::hash::Hash;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Hsv {
    pub h: f64,
    pub s: f64,
    pub v: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Hsl {
    pub h: f64,
    pub s: f64,
    pub l: f64,
}

impl Eq for Rgb {}
impl Eq for Hsv {}
impl Eq for Hsl {}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for Rgb {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.r.to_bits().hash(state);
        self.g.to_bits().hash(state);
        self.b.to_bits().hash(state);
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for Hsv {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.h.to_bits().hash(state);
        self.s.to_bits().hash(state);
        self.v.to_bits().hash(state);
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for Hsl {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.h.to_bits().hash(state);
        self.s.to_bits().hash(state);
        self.l.to_bits().hash(state);
    }
}

impl Rgb {
    pub fn to_hsv(&self) -> Hsv {
        // https://en.wikipedia.org/wiki/HSL_and_HSV#From_RGB
        let xmax = self.r.max(self.g.max(self.b));
        let xmin = self.r.min(self.g.min(self.b));
        let c = xmax - xmin;
        let mut h = match () {
            _ if c == 0.0 => 0.0,
            _ if xmax == self.r => 60.0 * ((self.g - self.b) / c),
            _ if xmax == self.g => 60.0 * ((self.b - self.r) / c + 2.0),
            _ if xmax == self.b => 60.0 * ((self.r - self.g) / c + 4.0),
            _ => panic!(),
        };
        if h < 0.0 {
            h += 360.0
        };
        let s = match () {
            _ if xmax == 0.0 => 0.0,
            _ => c / xmax,
        };

        Hsv { h, s, v: xmax }
    }

    pub fn to_hsl(&self) -> Hsl {
        // https://en.wikipedia.org/wiki/HSL_and_HSV#From_RGB
        let xmax = self.r.max(self.g.max(self.b));
        let xmin = self.r.min(self.g.min(self.b));
        let c = xmax - xmin;
        let mut h = match () {
            _ if c == 0.0 => 0.0,
            _ if xmax == self.r => 60.0 * ((self.g - self.b) / c),
            _ if xmax == self.g => 60.0 * ((self.b - self.r) / c + 2.0),
            _ if xmax == self.b => 60.0 * ((self.r - self.g) / c + 4.0),
            _ => panic!(),
        };
        if h < 0.0 {
            h += 360.0
        };
        let l = (xmax + xmin) / 2.0;
        let s = match () {
            _ if l == 0.0 || l == 1.0 => 0.0,
            _ => c / (1.0 - (2.0 * xmax - c - 1.0).abs()),
        };

        Hsl { h, s, l }
    }
}

impl Hsv {
    pub fn to_rgb(&self) -> Rgb {
        //https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_RGB
        let c = self.v * self.s;
        let h1 = self.h / 60.0;
        let x = c * (1.0 - (h1 % 2.0 - 1.0).abs());
        let (r1, g1, b1) = neighboring(c, x, h1);
        let m = self.v - c;
        let (r, g, b) = (r1 + m, g1 + m, b1 + m);

        Rgb { r, g, b }
    }

    pub fn to_hsl(&self) -> Hsl {
        // https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_HSL
        let l = self.v * (1.0 - (self.s / 2.0));
        let sl = match () {
            _ if l == 0.0 || l == 1.0 => 0.0,
            _ => 2.0 * (1.0 - l / self.v),
        };

        Hsl {
            h: self.h,
            s: sl,
            l,
        }
    }
}

impl Hsl {
    pub fn to_rgb(&self) -> Rgb {
        // https://en.wikipedia.org/wiki/HSL_and_HSV#HSL_to_RGB
        let c = (1.0 - (2.0 * self.l - 1.0).abs()) * self.s;
        let h1 = self.h / 60.0;
        let x = c * (1.0 - (h1 % 2.0 - 1.0).abs());
        let (r1, g1, b1) = neighboring(c, x, h1);
        let m = self.l - (c / 2.0);
        let (r, g, b) = (r1 + m, g1 + m, b1 + m);

        Rgb { r, g, b }
    }

    pub fn to_hsv(&self) -> Hsv {
        // https://en.wikipedia.org/wiki/HSL_and_HSV#HSL_to_HSV
        let v = self.l + self.s * self.l.min(1.0 - self.l);
        let sv = match () {
            _ if v == 0.0 => 0.0,
            _ => 2.0 * (1.0 - self.l / v),
        };

        Hsv {
            h: self.h,
            s: sv,
            v,
        }
    }
}

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
