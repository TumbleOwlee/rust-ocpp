//! Known OCPP primitive constraints that are *not* expressible in the JSON
//! schema (spec prose only). The schema carries no `pattern` keyword and does
//! not mark which `string` fields are `identifierString`, so the mapping of
//! field -> constraint is maintained here by hand.

/// `(type_name, json_field_key)` pairs whose value is an OCPP `identifierString`
/// (charset `^[a-zA-Z0-9*+=:|@._-]*$`, enforced by `validate_identifier_string`).
const IDENTIFIER_STRING_FIELDS: &[(&str, &str)] = &[("AdditionalInfoType", "additionalIdToken")];

pub fn is_identifier_string(type_name: &str, json_key: &str) -> bool {
    IDENTIFIER_STRING_FIELDS
        .iter()
        .any(|(t, k)| *t == type_name && *k == json_key)
}
