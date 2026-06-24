//! # iso7064 — ISO 7064 MOD 97-10 check characters
//!
//! Compute and validate ISO 7064 **MOD 97-10** check characters — the algorithm behind
//! [LEI](https://en.wikipedia.org/wiki/Legal_Entity_Identifier) codes, IBAN check digits, and
//! more. A faithful Rust port of the
//! [`iso-7064`](https://www.npmjs.com/package/iso-7064) npm package, plus the standard
//! validation and check-digit-generation helpers.
//!
//! ```
//! use iso7064::{compute, generate_check_digits, is_valid};
//!
//! // The MOD 97 of a string (the reference's `compute`):
//! assert_eq!(compute("969500KSV493XWY0PS").unwrap(), 54);
//!
//! // Generate the two check digits for a base, and validate the whole code:
//! assert_eq!(generate_check_digits("969500KSV493XWY0PS").unwrap(), "33");
//! assert!(is_valid("969500KSV493XWY0PS33"));
//! ```
//!
//! **Zero dependencies** and `#![no_std]`.

#![no_std]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/iso7064/0.1.0")]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use core::fmt;

// Compile-test the README's examples as part of `cargo test`.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

/// An error from a checked operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The input was empty or contained a character outside `[0-9A-Z]`.
    InvalidFormat,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid data format; expected a non-empty string of [0-9A-Z]")
    }
}

impl core::error::Error for Error {}

/// `^[0-9A-Z]{1,}$` — a non-empty run of digits and uppercase letters.
fn is_valid_format(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || byte.is_ascii_uppercase())
}

/// The running MOD 97 of a string, treating `0-9` as 0–9 and `A-Z` as 10–35 (the reference's
/// `mod97`). Inputs are processed as UTF-16 code units, matching `charCodeAt`.
fn mod97(value: &str) -> i64 {
    let mut buffer: i64 = 0;
    for code in value.encode_utf16() {
        let code = i64::from(code);
        // `>= 'A'` selects the two-digit (letter) contribution.
        let (multiplier, offset) = if code >= i64::from(b'A') {
            (100, i64::from(b'A') - 10)
        } else {
            (10, i64::from(b'0'))
        };
        buffer = (buffer * multiplier - offset + code) % 97;
    }
    buffer
}

/// Compute the MOD 97 of `value`, after checking it matches `[0-9A-Z]+`.
///
/// # Errors
/// Returns [`Error::InvalidFormat`] if `value` is empty or contains a character outside
/// `[0-9A-Z]`.
///
/// ```
/// # use iso7064::compute;
/// assert_eq!(compute("7245005WBNJAFHBD0S").unwrap(), 55);
/// ```
pub fn compute(value: &str) -> Result<i64, Error> {
    if is_valid_format(value) {
        Ok(mod97(value))
    } else {
        Err(Error::InvalidFormat)
    }
}

/// Compute the MOD 97 of `value` without validating its format.
///
/// Matches the reference's `computeWithoutCheck`; use [`compute`] unless you have already
/// validated the input.
#[must_use]
pub fn compute_without_check(value: &str) -> i64 {
    mod97(value)
}

/// Whether `value` is a valid ISO 7064 MOD 97-10 code (its MOD 97 equals 1).
///
/// ```
/// # use iso7064::is_valid;
/// assert!(is_valid("969500KSV493XWY0PS33"));
/// assert!(!is_valid("969500KSV493XWY0PS34"));
/// ```
#[must_use]
pub fn is_valid(value: &str) -> bool {
    is_valid_format(value) && mod97(value) == 1
}

/// Generate the two MOD 97-10 check digits for `base` (the code without its check digits).
///
/// Appending the returned two digits to `base` produces a string for which [`is_valid`]
/// returns `true`.
///
/// # Errors
/// Returns [`Error::InvalidFormat`] if `base` is empty or contains a character outside
/// `[0-9A-Z]`.
pub fn generate_check_digits(base: &str) -> Result<String, Error> {
    if !is_valid_format(base) {
        return Err(Error::InvalidFormat);
    }
    // The check `c` must satisfy `(mod97(base) * 100 + c) % 97 == 1`.
    let remainder = (mod97(base) * 100) % 97;
    let check = (98 - remainder) % 97;
    Ok(format!("{check:02}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_matches_reference() {
        assert_eq!(compute("969500KSV493XWY0PS").unwrap(), 54);
        assert_eq!(compute("7245005WBNJAFHBD0S").unwrap(), 55);
        assert_eq!(compute("0").unwrap(), 0);
        assert_eq!(compute("A").unwrap(), 10);
        assert_eq!(compute("Z").unwrap(), 35);
    }

    #[test]
    fn compute_without_check_is_lenient() {
        assert_eq!(compute_without_check("abc"), 66);
        assert_eq!(compute_without_check("a!b"), 94);
        assert_eq!(compute_without_check("969500KSV493XWY0PS"), 54);
    }

    #[test]
    fn invalid_format() {
        assert_eq!(compute(""), Err(Error::InvalidFormat));
        assert_eq!(compute("abc"), Err(Error::InvalidFormat)); // lowercase
        assert_eq!(compute("A B"), Err(Error::InvalidFormat));
        assert_eq!(generate_check_digits(""), Err(Error::InvalidFormat));
    }

    #[test]
    fn validate_and_generate() {
        assert_eq!(generate_check_digits("969500KSV493XWY0PS").unwrap(), "33");
        assert!(is_valid("969500KSV493XWY0PS33"));
        assert!(!is_valid("969500KSV493XWY0PS00"));
        assert!(!is_valid("lowercase")); // bad format
    }

    #[test]
    fn generate_round_trips() {
        for base in [
            "5493001KJTIIGC8Y1R",
            "529900T8BM49AURSDO",
            "ABC123",
            "0",
            "ZZZZ",
        ] {
            let check = generate_check_digits(base).unwrap();
            let full = alloc::format!("{base}{check}");
            assert!(is_valid(&full), "{full} should be valid");
        }
    }
}
