# Sorbet, the Tasteful GUI Library

Sorbet is a GUI library written in Rust that was designed with the simple goal of being easy to use and maintain,
following comprehensive declarative patterns (that may be different from what you're accustomed to)
and takes a "rusty" (or "rustaceous" if you prefer) approach to ensure your code doesn't [rot][1001].

> Currently, the API is unstable (pre-1.0.0) and will make no attempt to maintain backwards compatibility.

> TODO: This README.

[1001]: https://en.wikipedia.org/wiki/Software_rot

## Versioning

**This project and all submodules/crates use [Semantic Versioning][2000], just like the vast majority of Rust crates.**

A *SemVer* has the following format: `<major>.<minor>.<patch>[-<prerelease>]`.

Here is a brief summary, borrowed from the above reference, with numbered hyperlinks in superscript indicating relevant reading.

| Segment        | Usage                                                                                                             |
|----------------|-------------------------------------------------------------------------------------------------------------------|
| `<major>`      | When there are public API changes that are not backwards-compatible. <sup>[[4]][2104][[5]][2105][[8]][2108]</sup> |
| `<minor>`      | When functionality is added in a backwards-compatible matter. <sup>[[7]][2107]</sup>                              |
| `<patch>`      | When there are backwards-compatible bug fixes. <sup>[[6]][2106]</sup>                                             |
| `<prerelease>` | An optional segment indicating a revision number for a pre-release. <sup>[[9]][2109]</sup>                        |                     |

### Unstable API

Since the above specification (see numbered hyperlinks) provides no stringent explanation how backwards-incompatible
changes should affect the version number **before** the first stable version, the scheme used by this project and children is
detailed below.

> There is a [suggestion for how to handle this][2001], but it doesn't specify what to do with the patch number.

- The minor digits signify the current revision of the public API.
  - This will be incremented when there is a backwards-incompatible change.
  - This will also be incremented when there is an addition to the public API.
  - This effectively combines the meaning of the first two segments into the minor digits.
- The patch digits will be treated the same as if the version were stable.
  - This particular statement is not guaranteed, and its usage is up to the distributing maintainer's discretion.
    For example, if there is an insignificant but backwards-incompatible change, such as a typo in a function name.

Once the API is considered stable, and there is general satisfaction concerning the capabilities provided,
the major digit will be used instead to indicate API changes where something is removed or modified,
while the minor digit will signify additions that may not be stabilized.

[2000]: https://semver.org/
[2001]: https://semver.org/#how-should-i-deal-with-revisions-in-the-0yz-initial-development-phase
[2104]: https://semver.org/#spec-item-4
[2105]: https://semver.org/#spec-item-5
[2106]: https://semver.org/#spec-item-6
[2107]: https://semver.org/#spec-item-7
[2108]: https://semver.org/#spec-item-8
[2109]: https://semver.org/#spec-item-9

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
