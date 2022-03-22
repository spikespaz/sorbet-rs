# Sorbet Color - We did your homework.

[![GitHub issues](https://img.shields.io/github/issues/spikespaz/sorbet-rs?logo=github&style=flat-square)](https://github.com/spikespaz/sorbet-rs/issues)
[![GitHub forks](https://img.shields.io/github/forks/spikespaz/sorbet-rs?logo=github&style=flat-square)](https://github.com/spikespaz/sorbet-rs/network)
[![GitHub stars](https://img.shields.io/github/stars/spikespaz/sorbet-rs?logo=github&style=flat-square)](https://github.com/spikespaz/sorbet-rs/stargazers)
[
![Crates.io downloads](https://img.shields.io/crates/d/sorbet-color?logo=rust&style=flat-square)
![Crates.io license](https://img.shields.io/crates/l/sorbet-color?style=flat-square)
![Crates.io version](https://img.shields.io/crates/v/sorbet-color?label=version&style=flat-square)
][200]

This crate is a component of the work-in-progress GUI library by the same name, [Sorbet][100].

It should be handy as a standalone library for all of your color-conversion needs.

[100]: https://github.com/spikespaz/sorbet-rs
[200]: https://crates.io/crates/sorbet-color

---

This crate provides convenient types to represent colors in different formats/models/spaces.
All color structures have the [`Color`][1001] trait which specifies bounds for conversion between
every other color structure.

[1001]: https://docs.rs/sorbet-color/latest/sorbet_color/trait.Color.html

## Details

The transparency/alpha component as structure fields is always fully spelled-out as `alpha`,
to differentiate it from any color models that may have an `a` component
(none handled by this library thus-far).

Most of the mathematics used here will be based on the algorithms found on Wikipedia or
other crowd-sourced references.

This library is incomplete and is missing important spaces such as CIE XYZ, LUV, and LAB.

Some interesting reading about the
[CIE 1931 color space can be found on Wikipedia][2001].

If you would like to utilize this crate and there is a format missing, please
[open a new issue on the GitHub repository][101] after searching to make sure a request hasn't already been submitted.

**Pull requests are very welcome.**

[101]: https://github.com/spikespaz/sorbet-rs/issues
[2001]: https://en.wikipedia.org/wiki/CIE_1931_color_space

## Versioning

See [Versioning][103] for Sorbet.

```toml
[dependencies]
## It is recommended to use the latest version while the public API is unstable.
sorbet-color = "*"
## Otherwise, if you would like to not worry about updating the dependency,
## feel free to use the latest minor version and leave it at that.
sorbet-color = "0.X"
```

[102]: https://github.com/spikespaz/sorbet-rs/blob/master/README.md
[103]: https://github.com/spikespaz/sorbet-rs/blob/master/README.md#versioning

## License

> Copyright &copy; 2022 Jacob Birkett

This project is provided under the [Apache License, Version 2.0](https://opensource.org/licenses/Apache-2.0).

<table>
  <tr>
    <th>&#x1F7E2; Permissions</th>
    <th>&#x1F7E1; Conditions</th>
    <th>&#x1F534; Limitations</th>
  </tr>
  <tr>
    <td>
      <ul>
        <li>Commercial use</li>
        <li>Distribution</li>
        <li>Modification</li>
        <li>Patent use</li>
        <li>Private use</li>
      </ul>
    </td>
    <td>
      <ul>
        <li>License and copyright notice</li>
        <li>State changes</li>
      </ul>
    </td>
    <td>
      <ul>
        <li>Liability</li>
        <li>Trademark use</li>
        <li>Warranty</li>
      </ul>
    </td>
  </tr>
</table>
