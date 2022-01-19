#[derive(Copy, Clone, Debug)]
pub struct Rgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Hsv {
    pub h: f64,
    pub s: f64,
    pub v: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Hsl {
    pub h: f64,
    pub s: f64,
    pub l: f64,
}
