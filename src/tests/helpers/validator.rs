//! Tests for the generated field validators.
//!
//! Specified by CG-R-006..009 in `docs/specs/codegen/requirements.md`.
//!
//! Charset vectors are derived from the `identifierString` definition in the OCPP
//! specification, not from the emitted regex — a test read off the implementation
//! would agree with a wrong implementation.

macro_rules! identifier_string_tests {
    ($modname:ident, $validator:path) => {
        mod $modname {
            use $validator as validate_identifier_string;

            #[test]
            /// CG-R-006 — The emitted identifier-string validator shall accept a string
            /// composed solely of characters from `a-z`, `A-Z`, `0-9`, `*`, `+`, `=`, `:`,
            /// `|`, `@`, `.`, `_`, `-`.
            fn it_identifier_accepts_full_charset() {
                let every_allowed_character =
                    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789*+=:|@._-";

                assert!(validate_identifier_string(every_allowed_character).is_ok());

                for realistic in ["RFID-1234", "emaid:DE*ABC*C12345678*1", "a@b.c", "0"] {
                    assert!(
                        validate_identifier_string(realistic).is_ok(),
                        "expected {realistic:?} to be a valid identifierString"
                    );
                }
            }

            #[test]
            /// CG-R-006 — The empty string is a valid identifierString; see the "Empty
            /// identifier string" entry in `docs/specs/codegen/edge-cases.md`.
            fn it_identifier_accepts_empty() {
                assert!(validate_identifier_string("").is_ok());
            }

            #[test]
            /// CG-R-007 — The emitted identifier-string validator shall reject a string
            /// containing any character outside that set — including a space, `/`, `#`, or
            /// any non-ASCII character — by returning a validation error, never a panic.
            fn it_identifier_rejects_outside_charset() {
                for rejected in [
                    "has space",
                    "slash/separated",
                    "hash#tag",
                    "quote\"d",
                    "semi;colon",
                    "brace{s}",
                    "non-ascii-ä",
                    "emoji-🔌",
                    "new\nline",
                    "tab\tchar",
                ] {
                    assert!(
                        validate_identifier_string(rejected).is_err(),
                        "expected {rejected:?} to be rejected"
                    );
                }
            }

            #[test]
            /// CG-R-007 — A single disallowed character anywhere in an otherwise valid
            /// string is enough to reject it, including at the very end where an
            /// end-of-line-tolerant anchor would let it pass.
            fn it_identifier_rejects_trailing_disallowed_character() {
                assert!(validate_identifier_string("valid-tail/").is_err());
                assert!(validate_identifier_string("valid-tail\n").is_err());
            }
        }
    };
}

#[cfg(feature = "v2_0_1")]
identifier_string_tests!(
    v2_0_1,
    crate::v2_0_1::helpers::validator::validate_identifier_string
);

#[cfg(feature = "v2_1")]
identifier_string_tests!(
    v2_1,
    crate::v2_1::helpers::validator::validate_identifier_string
);

/// Decimal range validators are emitted per distinct bound pair found in the
/// schemas, so v2.0.1 has none — see the "v2.0.1 emits no decimal range
/// validators" entry in `docs/specs/codegen/edge-cases.md`.
#[cfg(feature = "v2_1")]
mod decimal_range_v2_1 {
    use crate::v2_1::helpers::validator::{validate_decimal_max_0, validate_decimal_min_0_max_100};
    use rust_decimal_macros::dec;

    #[test]
    /// CG-R-008 — A range validator emitted for a schema-declared `minimum` and/or
    /// `maximum` shall accept a value inside the range and reject a value outside it,
    /// returning a validation error that distinguishes below-minimum from
    /// above-maximum.
    fn it_decimal_range_rejects_outside_bounds() {
        assert!(validate_decimal_min_0_max_100(&dec!(50)).is_ok());

        let below = validate_decimal_min_0_max_100(&dec!(-0.0001))
            .expect_err("a value under the minimum is rejected");
        assert_eq!(below.code, "value below minimum");

        let above = validate_decimal_min_0_max_100(&dec!(100.0001))
            .expect_err("a value over the maximum is rejected");
        assert_eq!(above.code, "value above maximum");
    }

    #[test]
    /// CG-R-008 — A validator emitted for a `maximum` alone imposes no lower bound:
    /// an arbitrarily small value is accepted and only the upper bound rejects.
    fn it_decimal_max_only_bounds_one_side() {
        assert!(validate_decimal_max_0(&dec!(-99999.5)).is_ok());

        let above =
            validate_decimal_max_0(&dec!(0.0001)).expect_err("a positive value is rejected");
        assert_eq!(above.code, "value above maximum");
    }

    #[test]
    /// CG-R-009 — A range validator shall treat both declared bounds as inclusive: a
    /// value exactly equal to the minimum or the maximum shall be accepted.
    fn it_decimal_range_accepts_exact_bounds() {
        assert!(validate_decimal_min_0_max_100(&dec!(0)).is_ok());
        assert!(validate_decimal_min_0_max_100(&dec!(100)).is_ok());
        assert!(validate_decimal_max_0(&dec!(0)).is_ok());
    }

    #[test]
    /// CG-R-009 — Inclusivity holds for bound values written with trailing decimal
    /// places, which `Decimal` represents with a different scale than the integer form.
    fn it_decimal_range_accepts_exact_bounds_at_other_scales() {
        assert!(validate_decimal_min_0_max_100(&dec!(0.0000)).is_ok());
        assert!(validate_decimal_min_0_max_100(&dec!(100.0000)).is_ok());
        assert!(validate_decimal_max_0(&dec!(-0.0)).is_ok());
    }
}
