//! Identifier conversion helpers.
//!
//! Rust type names are taken verbatim from the schema definition key (already
//! PascalCase, e.g. `OCSPRequestDataType`). File/field names are derived with an
//! acronym-aware snake_case. Renames are decided by replaying serde's exact
//! `camelCase` rule and comparing to the original JSON key.

/// Field-name overrides where acronym-aware snake_case still disagrees with the
/// hand-written code (domain acronyms written with an internal lowercase letter,
/// e.g. `SoC` = State-of-Charge). Discovered by diffing generated field names
/// against the existing `src/v2_0_1` code.
const FIELD_SNAKE_OVERRIDES: &[(&str, &str)] = &[
    ("fullSoC", "full_soc"),
    ("bulkSoC", "bulk_soc"),
    (
        "iso15118CertificateHashData",
        "iso_15118_certificate_hash_data",
    ),
    ("iso15118SchemaVersion", "iso_15118_schema_version"),
];

/// File-stem overrides where the existing hand-written module name disagrees
/// with acronym-aware snake_case (idiosyncratic digit/acronym splits). Keyed by
/// the type name (datatypes/enums) or message base name. Required so the
/// hand-written test `use` paths keep resolving.
const STEM_OVERRIDES: &[(&str, &str)] = &[
    (
        "Iso15118EVCertificateStatusEnumType",
        "iso15118ev_certificate_status_enum_type",
    ),
    ("DataTransfer", "datatransfer"),
    ("Get15118EVCertificate", "get_15118ev_certificate"),
    ("GetDisplayMessages", "get_display_message"),
];

/// Acronym-aware snake_case. Handles uppercase runs (acronyms) and digit
/// boundaries: `OCSPRequestDataType` -> `ocsp_request_data_type`,
/// `responderURL` -> `responder_url`, `iso15118SchemaVersion` ->
/// `iso15118_schema_version`.
pub fn to_snake(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len() + 4);
    for (i, &c) in chars.iter().enumerate() {
        if c == '_' || c == '-' || c == '.' {
            if !out.ends_with('_') {
                out.push('_');
            }
            continue;
        }
        if c.is_uppercase() {
            let prev = if i > 0 { Some(chars[i - 1]) } else { None };
            let next = chars.get(i + 1).copied();
            let boundary = match prev {
                None => false,
                Some(p) => {
                    p.is_lowercase()
                        || p.is_ascii_digit()
                        || (p.is_uppercase() && next.is_some_and(|n| n.is_lowercase()))
                }
            };
            if boundary && !out.ends_with('_') {
                out.push('_');
            }
            for lc in c.to_lowercase() {
                out.push(lc);
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// snake_case for a JSON field key, applying domain overrides.
pub fn field_snake(json_key: &str) -> String {
    for (k, v) in FIELD_SNAKE_OVERRIDES {
        if *k == json_key {
            return (*v).to_string();
        }
    }
    to_snake(json_key)
}

/// File stem for a Rust type name or message base name: acronym-aware
/// snake_case with the redundant `_enum_type` / `_type` suffix dropped (the
/// containing directory already conveys it). Message base names rarely carry
/// the suffix, so they pass through unchanged.
pub fn file_stem(type_name: &str) -> String {
    let base = STEM_OVERRIDES
        .iter()
        .find(|(k, _)| *k == type_name)
        .map(|(_, v)| (*v).to_string())
        .unwrap_or_else(|| to_snake(type_name));
    drop_type_suffix(&base)
}

fn drop_type_suffix(stem: &str) -> String {
    if let Some(s) = stem.strip_suffix("_enum_type") {
        s.to_string()
    } else if let Some(s) = stem.strip_suffix("_type") {
        s.to_string()
    } else {
        stem.to_string()
    }
}

/// serde's `RenameRule::CamelCase` applied to a Rust snake_case identifier:
/// first word lowercase, each subsequent word capitalized on its first letter,
/// remaining letters untouched.
pub fn serde_camel_case(snake: &str) -> String {
    let mut out = String::with_capacity(snake.len());
    let mut capitalize_next = false;
    let mut first_word = true;
    for c in snake.chars() {
        if c == '_' {
            capitalize_next = !first_word;
            // a leading underscore keeps us on the first word
            continue;
        }
        if capitalize_next {
            for uc in c.to_uppercase() {
                out.push(uc);
            }
            capitalize_next = false;
        } else {
            out.push(c);
        }
        first_word = false;
    }
    out
}

/// Rust keywords that need escaping/renaming when they appear as field names.
const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false", "fn",
    "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
    "return", "self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use",
    "where", "while", "async", "await", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield",
];

/// Maps a JSON field key to its Rust field name, resolving reserved words.
/// The reserved word `type` is always mapped to `kind` (project convention).
pub fn rust_field_name(json_key: &str) -> String {
    let snake = field_snake(json_key);
    if snake == "type" {
        return "kind".to_string();
    }
    if RUST_KEYWORDS.contains(&snake.as_str()) {
        return format!("{snake}_");
    }
    snake
}

/// Returns `Some(json_key)` when an explicit `#[serde(rename = ...)]` is needed,
/// i.e. when serde's camelCase of the Rust field name does not reproduce the
/// original JSON key.
pub fn rename_if_needed(rust_field: &str, json_key: &str) -> Option<String> {
    let trimmed = rust_field.strip_suffix('_').unwrap_or(rust_field);
    if serde_camel_case(trimmed) == json_key {
        None
    } else {
        Some(json_key.to_string())
    }
}

/// Sanitizes an enum variant name from a JSON enum value. Returns the Rust
/// identifier plus an optional rename (when the value isn't a clean PascalCase
/// identifier, e.g. `Current.Export`, `L2-N`, `SHA256`).
pub fn enum_variant(json_value: &str) -> (String, Option<String>) {
    // Build a PascalCase identifier from the value, splitting on non-alnum.
    let mut ident = String::new();
    let mut capitalize = true;
    for c in json_value.chars() {
        if c.is_alphanumeric() {
            if capitalize {
                for uc in c.to_uppercase() {
                    ident.push(uc);
                }
                capitalize = false;
            } else {
                ident.push(c);
            }
        } else {
            capitalize = true;
        }
    }
    if ident.is_empty() || ident.chars().next().unwrap().is_ascii_digit() {
        ident = format!("V{ident}");
    }
    let rename = if ident == json_value {
        None
    } else {
        Some(json_value.to_string())
    };
    (ident, rename)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_acronyms() {
        assert_eq!(to_snake("OCSPRequestDataType"), "ocsp_request_data_type");
        assert_eq!(to_snake("responderURL"), "responder_url");
        assert_eq!(to_snake("IdTokenType"), "id_token_type");
        assert_eq!(to_snake("APNType"), "apn_type");
        assert_eq!(to_snake("VPNType"), "vpn_type");
        assert_eq!(to_snake("iso15118SchemaVersion"), "iso15118_schema_version");
        assert_eq!(to_snake("stateOfCharge"), "state_of_charge");
    }

    #[test]
    fn field_overrides() {
        assert_eq!(field_snake("fullSoC"), "full_soc");
        assert_eq!(field_snake("bulkSoC"), "bulk_soc");
    }

    #[test]
    fn camel_roundtrip() {
        assert_eq!(serde_camel_case("state_of_charge"), "stateOfCharge");
        assert_eq!(serde_camel_case("full_soc"), "fullSoc");
        assert_eq!(serde_camel_case("responder_url"), "responderUrl");
        assert_eq!(serde_camel_case("id_token"), "idToken");
    }

    #[test]
    fn renames() {
        assert_eq!(rename_if_needed("state_of_charge", "stateOfCharge"), None);
        assert_eq!(
            rename_if_needed("full_soc", "fullSoC"),
            Some("fullSoC".to_string())
        );
        assert_eq!(
            rename_if_needed("responder_url", "responderURL"),
            Some("responderURL".to_string())
        );
        assert_eq!(rename_if_needed("kind", "type"), Some("type".to_string()));
    }

    #[test]
    fn keywords() {
        assert_eq!(rust_field_name("type"), "kind");
    }

    #[test]
    fn variants() {
        assert_eq!(enum_variant("Accepted"), ("Accepted".to_string(), None));
        assert_eq!(
            enum_variant("Current.Export"),
            (
                "CurrentExport".to_string(),
                Some("Current.Export".to_string())
            )
        );
        assert_eq!(enum_variant("SHA256"), ("SHA256".to_string(), None));
        assert_eq!(
            enum_variant("eMAID"),
            ("EMAID".to_string(), Some("eMAID".to_string()))
        );
    }
}
