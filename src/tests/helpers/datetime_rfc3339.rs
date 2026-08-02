//! Tests for the generated RFC3339 date-time serde adapter.
//!
//! Specified by CG-R-001..005 and CG-R-010 in `docs/specs/codegen/requirements.md`.
//!
//! The adapter is exercised through real serde derives rather than by calling
//! `serialize`/`deserialize` directly — the field attribute is how consumers reach
//! it, and it is the attribute path that a regeneration could break.
//!
//! Every protocol version gets its own byte-identical copy of the helper, so the
//! test bodies are generated once per version by [`datetime_helper_tests`]. The
//! copies are separate code and are counted separately by coverage; asserting on
//! one of them says nothing about the other.

macro_rules! datetime_helper_tests {
    ($modname:ident, $required:literal, $optional:literal) => {
        mod $modname {
            use chrono::{DateTime, Utc};
            use serde::{Deserialize, Serialize};

            #[derive(Serialize, Deserialize, Debug, PartialEq)]
            struct Required {
                #[serde(with = $required)]
                ts: DateTime<Utc>,
            }

            #[derive(Serialize, Deserialize, Debug, PartialEq)]
            struct Optional {
                #[serde(with = $optional)]
                ts: Option<DateTime<Utc>>,
            }

            /// The message a deserialization error carries, with serde's positional
            /// `at line L column C` suffix removed. The position varies with input
            /// length, so comparing whole messages would let a canned error string
            /// masquerade as a forwarded one.
            fn parse_message(err: &serde_json::Error) -> String {
                let rendered = err.to_string();
                match rendered.split_once(" at line ") {
                    Some((message, _position)) => message.to_string(),
                    None => rendered,
                }
            }

            fn at(rfc3339: &str) -> DateTime<Utc> {
                DateTime::parse_from_rfc3339(rfc3339)
                    .expect("test vector is a valid RFC3339 timestamp")
                    .with_timezone(&Utc)
            }

            #[test]
            /// CG-R-001 — Serializing a UTC date-time through the emitted RFC3339 adapter
            /// shall produce an RFC3339 string with whole-second precision and a literal
            /// `Z` UTC designator, discarding any sub-second component.
            fn it_datetime_serializes_second_precision_with_z() {
                let value = Required {
                    ts: at("2023-01-15T14:30:00.512Z"),
                };

                let json = serde_json::to_string(&value).expect("serialization cannot fail");

                assert_eq!(json, r#"{"ts":"2023-01-15T14:30:00Z"}"#);
            }

            #[test]
            /// CG-R-002 — Deserializing through the emitted RFC3339 adapter shall accept any
            /// valid RFC3339 timestamp, including one carrying a non-UTC offset, and shall
            /// yield the equivalent instant expressed in UTC.
            fn it_datetime_deserializes_offset_as_utc() {
                let value: Required = serde_json::from_str(r#"{"ts":"2023-01-15T09:30:00-05:00"}"#)
                    .expect("a valid RFC3339 timestamp with an offset is accepted");

                assert_eq!(value.ts, at("2023-01-15T14:30:00Z"));
            }

            #[test]
            /// CG-R-003 — Deserializing a string that is not a valid RFC3339 timestamp shall
            /// fail with a deserialization error carrying the underlying parse message, never
            /// a panic.
            fn it_datetime_rejects_malformed_string() {
                let bad_character =
                    serde_json::from_str::<Required>(r#"{"ts":"2023-01-15T14:30:00X"}"#)
                        .expect_err("a non-RFC3339 string is rejected");
                let truncated = serde_json::from_str::<Required>(r#"{"ts":"2023-01-15T14:30"}"#)
                    .expect_err("a truncated timestamp is rejected");

                for err in [&bad_character, &truncated] {
                    assert_eq!(err.classify(), serde_json::error::Category::Data);
                    assert!(
                        !parse_message(err).is_empty(),
                        "expected a parse message, got an empty error"
                    );
                    assert!(
                        !err.to_string().contains("invalid type"),
                        "expected a parse failure, not serde's generic type mismatch: {err}"
                    );
                }

                // The two inputs fail for different reasons — a disallowed character versus
                // a string that ends early — so a forwarded parse message differs between
                // them. A single canned error string would not. This pins "carrying the
                // underlying parse message" without pinning chrono's wording, which is not
                // ours to depend on.
                assert_ne!(
                    parse_message(&bad_character),
                    parse_message(&truncated),
                    "expected the underlying parse message to reach the caller, \
                     but both inputs produced the same message"
                );
            }

            #[test]
            /// CG-R-004 — The optional variant of the adapter shall serialize an absent value
            /// as JSON `null`, and shall deserialize JSON `null` to an absent value.
            fn it_datetime_option_roundtrips_null() {
                let absent = Optional { ts: None };

                let json = serde_json::to_string(&absent).expect("serialization cannot fail");
                assert_eq!(json, r#"{"ts":null}"#);

                let parsed: Optional =
                    serde_json::from_str(r#"{"ts":null}"#).expect("null is accepted");
                assert_eq!(parsed, absent);
            }

            #[test]
            /// CG-R-010 — The optional variant of the adapter shall serialize and deserialize
            /// a present value with the same observable semantics as the non-optional
            /// variant.
            fn it_datetime_option_roundtrips_present_value() {
                let present = Optional {
                    ts: Some(at("2023-01-15T14:30:00Z")),
                };

                let json = serde_json::to_string(&present).expect("serialization cannot fail");
                assert_eq!(json, r#"{"ts":"2023-01-15T14:30:00Z"}"#);

                let parsed: Optional = serde_json::from_str(&json).expect("round-trip parses");
                assert_eq!(parsed, present);
            }

            #[test]
            /// CG-R-005 — Deserializing JSON `null` through the non-optional adapter shall fail
            /// with a deserialization error.
            fn it_datetime_required_rejects_null() {
                let err = serde_json::from_str::<Required>(r#"{"ts":null}"#)
                    .expect_err("null is not a timestamp");

                assert_eq!(err.classify(), serde_json::error::Category::Data);
            }
        }
    };
}

#[cfg(feature = "v2_0_1")]
datetime_helper_tests!(
    v2_0_1,
    "crate::v2_0_1::helpers::datetime_rfc3339",
    "crate::v2_0_1::helpers::datetime_rfc3339::option"
);

#[cfg(feature = "v2_1")]
datetime_helper_tests!(
    v2_1,
    "crate::v2_1::helpers::datetime_rfc3339",
    "crate::v2_1::helpers::datetime_rfc3339::option"
);
