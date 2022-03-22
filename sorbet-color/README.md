This crate provides convenient types to represent colors in different formats/models/spaces.
All color structures have the [`Color`] trait which specifies bounds for conversion between
every other color structure.

The transparency/alpha component as structure fields is always fully spelled-out as `alpha`,
to differentiate it from any color models that may have an `a` component
(none handled by this library thus-far).

Most of the mathematics used here will be based on the algorithms found on Wikipedia or
other crowd-sourced references.

This library is incomplete and is missing important spaces such as CIE XYZ, LUV, and LAB.

Some interesting reading about the
[CIE 1931 color space can be found on Wikipedia](https://en.wikipedia.org/wiki/CIE_1931_color_space).

If you would like to utilize this crate and there is a format missing, please
[open a new issue on the GitHub repository](https://github.com/spikespaz/sorbet-rs).

Pull requests are very welcome.
