# iso7064

[![All Contributors](https://img.shields.io/badge/all_contributors-1-orange.svg?style=flat-square)](#contributors-)

[![crates.io](https://img.shields.io/crates/v/iso7064.svg)](https://crates.io/crates/iso7064)
[![docs.rs](https://docs.rs/iso7064/badge.svg)](https://docs.rs/iso7064)
[![CI](https://github.com/trananhtung/iso7064/actions/workflows/ci.yml/badge.svg)](https://github.com/trananhtung/iso7064/actions/workflows/ci.yml)
[![license](https://img.shields.io/crates/l/iso7064.svg)](#license)

**ISO 7064 MOD 97-10 check characters.**

Compute and validate ISO 7064 **MOD 97-10** check characters — the algorithm behind
[LEI](https://en.wikipedia.org/wiki/Legal_Entity_Identifier) codes, IBAN check digits, and
more. A faithful Rust port of the [`iso-7064`](https://www.npmjs.com/package/iso-7064) npm
package, plus standard validation and check-digit-generation helpers.

- **Zero dependencies**, **`#![no_std]`**
- `compute` / `compute_without_check` (faithful to the reference)
- `is_valid` and `generate_check_digits`
- Differential-tested against the reference `iso-7064` implementation (60k cases)

## Install

```toml
[dependencies]
iso7064 = "0.1"
```

## Usage

```rust
use iso7064::{compute, compute_without_check, is_valid, generate_check_digits};

// MOD 97 of a string:
assert_eq!(compute("969500KSV493XWY0PS").unwrap(), 54);

// Generate the two check digits and validate the full code:
assert_eq!(generate_check_digits("969500KSV493XWY0PS").unwrap(), "33");
assert!(is_valid("969500KSV493XWY0PS33"));

// `compute_without_check` skips format validation (use it on pre-validated input):
assert_eq!(compute_without_check("969500KSV493XWY0PS"), 54);
```

Digits `0-9` count as 0–9 and letters `A-Z` as 10–35. A code is valid when its MOD 97 equals
`1`.

## Contributors ✨

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind are welcome — code, docs, bug reports, ideas, reviews! See the [emoji key](https://allcontributors.org/docs/en/emoji-key) for how each contribution is recognized, and open a PR or issue to get involved.

Thanks goes to these wonderful people:

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/trananhtung"><img src="https://avatars.githubusercontent.com/u/30992229?v=4?s=100" width="100px;" alt="Tung Tran"/><br /><sub><b>Tung Tran</b></sub></a><br /><a href="https://github.com/trananhtung/./commits?author=trananhtung" title="Code">💻</a> <a href="#maintenance-trananhtung" title="Maintenance">🚧</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
