//! Integration tests exercising the public API of `iso7064`.

use iso7064::{compute, generate_check_digits, is_valid, Error};

#[test]
fn lei_examples() {
    // Real LEIs (18-char base + 2 check digits) validate, and we can regenerate the check.
    for lei in ["969500KSV493XWY0PS33", "5493001KJTIIGC8Y1R12", "MAINSARDELLA32434312"] {
        assert!(is_valid(lei), "{lei} should validate");
        let (base, check) = lei.split_at(18);
        assert_eq!(generate_check_digits(base).unwrap(), check);
    }
}

#[test]
fn rejects_wrong_check() {
    assert!(!is_valid("969500KSV493XWY0PS34"));
    assert!(!is_valid("969500KSV493XWY0PS00"));
}

#[test]
fn compute_basics() {
    assert_eq!(compute("0").unwrap(), 0);
    assert_eq!(compute("1").unwrap(), 1);
    assert_eq!(compute("A").unwrap(), 10);
    assert!(matches!(compute("foo!"), Err(Error::InvalidFormat)));
}

#[test]
fn generate_then_validate_roundtrip() {
    for base in ["BASE", "12345", "A1B2C3", "529900T8BM49AURSDO"] {
        let check = generate_check_digits(base).unwrap();
        assert!(is_valid(&format!("{base}{check}")));
    }
}
