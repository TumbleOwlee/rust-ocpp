//! Frontend for OCPP `definitions`-based OCPP schemas (v2.0.1 and v2.1):
//! schema JSON -> [`IrModel`].

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result};
use serde_json::Value;

use crate::ir::*;
use crate::naming;
use crate::primitives;

/// Parse every `*.json` schema in `dir` for the given `version` (e.g. `v2_0_1`)
/// into a single deduplicated [`IrModel`].
pub fn build_model(version: &str, dir: &Path) -> Result<IrModel> {
    let mut files: Vec<_> = std::fs::read_dir(dir)
        .with_context(|| format!("reading schema dir {}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().map(|e| e == "json").unwrap_or(false))
        .collect();
    files.sort();

    // Global, deduplicated definition map (name -> canonical body).
    let mut defs: BTreeMap<String, Value> = BTreeMap::new();
    // Message bodies grouped by base name -> (is_response, body, file).
    let mut messages: BTreeMap<String, MessagePair> = BTreeMap::new();

    for path in &files {
        let text = std::fs::read_to_string(path)?;
        let doc: Value =
            serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;

        if let Some(map) = doc.get("definitions").and_then(Value::as_object) {
            for (name, body) in map {
                merge_definition(&mut defs, name, body);
            }
        }

        let stem = path.file_stem().unwrap().to_string_lossy().to_string();
        let (base, is_response) = split_message(&stem);
        let entry = messages.entry(base.to_string()).or_default();
        if is_response {
            entry.response = Some((stem.clone(), doc.clone()));
        } else {
            entry.request = Some((stem.clone(), doc.clone()));
        }
    }

    let mut decimal_ranges: Vec<DecimalRange> = Vec::new();

    // Named types from the global definition map.
    let mut types: Vec<IrType> = Vec::new();
    for (name, body) in &defs {
        if is_enum(body) {
            types.push(IrType::Enum(build_enum(name, body)));
        } else {
            types.push(IrType::Struct(build_struct(
                name,
                body,
                &mut decimal_ranges,
            )));
        }
    }

    // Messages.
    let mut ir_messages: Vec<IrMessage> = Vec::new();
    for (base, pair) in &messages {
        let request = pair
            .request
            .as_ref()
            .map(|(n, b)| build_struct(n, b, &mut decimal_ranges));
        let response = pair
            .response
            .as_ref()
            .map(|(n, b)| build_struct(n, b, &mut decimal_ranges));
        ir_messages.push(IrMessage {
            file_stem: naming::file_stem(base),
            request,
            response,
        });
    }

    Ok(IrModel {
        version: version.to_string(),
        types,
        messages: ir_messages,
        decimal_ranges,
    })
}

#[derive(Default)]
struct MessagePair {
    request: Option<(String, Value)>,
    response: Option<(String, Value)>,
}

/// Splits a message file stem into `(base, is_response)`.
fn split_message(stem: &str) -> (&str, bool) {
    if let Some(b) = stem.strip_suffix("Response") {
        (b, true)
    } else if let Some(b) = stem.strip_suffix("Request") {
        (b, false)
    } else {
        (stem, false)
    }
}

/// Merge a definition into the global map, keeping the longest description as
/// the canonical doc. Shapes are identical across files (verified), so any
/// occurrence is structurally fine.
fn merge_definition(defs: &mut BTreeMap<String, Value>, name: &str, body: &Value) {
    match defs.get_mut(name) {
        None => {
            defs.insert(name.to_string(), body.clone());
        }
        Some(existing) => {
            let new_len = body
                .get("description")
                .and_then(Value::as_str)
                .map(str::len)
                .unwrap_or(0);
            let old_len = existing
                .get("description")
                .and_then(Value::as_str)
                .map(str::len)
                .unwrap_or(0);
            if new_len > old_len {
                if let Some(desc) = body.get("description") {
                    existing
                        .as_object_mut()
                        .unwrap()
                        .insert("description".to_string(), desc.clone());
                }
            }
        }
    }
}

fn is_enum(body: &Value) -> bool {
    body.get("type").and_then(Value::as_str) == Some("string") && body.get("enum").is_some()
}

fn clean_doc(body: &Value) -> Option<String> {
    body.get("description")
        .and_then(Value::as_str)
        .map(|s| {
            s.replace('\r', "")
                .replace('\t', " ")
                .trim_end()
                .to_string()
        })
        .filter(|s| !s.is_empty())
}

fn build_enum(name: &str, body: &Value) -> IrEnum {
    let values: Vec<String> = body
        .get("enum")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();

    let default_value = body.get("default").and_then(Value::as_str);
    let mut default_variant = 0;
    let variants: Vec<IrVariant> = values
        .iter()
        .enumerate()
        .map(|(i, v)| {
            if Some(v.as_str()) == default_value {
                default_variant = i;
            }
            let (rust_name, rename) = naming::enum_variant(v);
            IrVariant {
                rust_name,
                json_value: v.clone(),
                doc: None,
                rename,
            }
        })
        .collect();

    IrEnum {
        name: name.to_string(),
        doc: clean_doc(body),
        variants,
        default_variant,
    }
}

fn build_struct(name: &str, body: &Value, decimal_ranges: &mut Vec<DecimalRange>) -> IrStruct {
    let required: Vec<String> = body
        .get("required")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();

    let mut fields = Vec::new();
    if let Some(props) = body.get("properties").and_then(Value::as_object) {
        for (key, pbody) in props {
            let optional = !required.contains(key);
            fields.push(build_field(name, key, pbody, optional, decimal_ranges));
        }
    }

    IrStruct {
        name: name.to_string(),
        doc: clean_doc(body),
        fields,
    }
}

fn build_field(
    owner: &str,
    key: &str,
    pbody: &Value,
    optional: bool,
    decimal_ranges: &mut Vec<DecimalRange>,
) -> IrField {
    let rust_name = naming::rust_field_name(key);
    let rename = naming::rename_if_needed(&rust_name, key);
    let mut constraints = Vec::new();
    let ty = resolve_type(pbody, &mut constraints, decimal_ranges);

    if primitives::is_identifier_string(owner, key) {
        constraints.push(Constraint::IdentifierString);
    }

    IrField {
        rust_name,
        json_key: key.to_string(),
        ty,
        optional,
        doc: clean_doc(pbody),
        constraints,
        rename,
    }
}

fn ref_name(pbody: &Value) -> Option<String> {
    pbody
        .get("$ref")
        .and_then(Value::as_str)
        .and_then(|r| r.rsplit('/').next())
        .map(str::to_string)
}

fn as_i64(v: &Value, key: &str) -> Option<i64> {
    v.get(key)
        .and_then(|n| n.as_i64().or_else(|| n.as_f64().map(|f| f as i64)))
}

fn resolve_type(
    pbody: &Value,
    constraints: &mut Vec<Constraint>,
    decimal_ranges: &mut Vec<DecimalRange>,
) -> IrTypeRef {
    if let Some(name) = ref_name(pbody) {
        return IrTypeRef::Named(name);
    }
    match pbody.get("type").and_then(Value::as_str) {
        Some("string") => {
            if pbody.get("format").and_then(Value::as_str) == Some("date-time") {
                IrTypeRef::DateTime
            } else {
                let min = as_i64(pbody, "minLength");
                let max = as_i64(pbody, "maxLength");
                if min.is_some() || max.is_some() {
                    constraints.push(Constraint::Length { min, max });
                }
                IrTypeRef::Str
            }
        }
        Some("integer") => IrTypeRef::Int {
            min: as_i64(pbody, "minimum"),
            max: as_i64(pbody, "maximum"),
        },
        Some("number") => {
            let min = pbody.get("minimum").and_then(Value::as_f64);
            let max = pbody.get("maximum").and_then(Value::as_f64);
            if min.is_some() || max.is_some() {
                let fn_name = decimal_range_fn_name(min, max);
                register_decimal_range(decimal_ranges, &fn_name, min, max);
                constraints.push(Constraint::DecimalRange { fn_name });
            }
            IrTypeRef::Number
        }
        Some("boolean") => IrTypeRef::Bool,
        Some("array") => {
            let min = as_i64(pbody, "minItems");
            let max = as_i64(pbody, "maxItems");
            if min.is_some() || max.is_some() {
                constraints.push(Constraint::Length { min, max });
            }
            let item = pbody
                .get("items")
                .map(|it| resolve_type(it, &mut Vec::new(), decimal_ranges))
                .unwrap_or(IrTypeRef::Str);
            IrTypeRef::Array(Box::new(item))
        }
        // Fallback: treat unknown as a string (should not occur in v2.x bodies).
        _ => IrTypeRef::Str,
    }
}

fn decimal_range_fn_name(min: Option<f64>, max: Option<f64>) -> String {
    fn part(label: &str, v: Option<f64>) -> String {
        match v {
            None => String::new(),
            Some(f) => {
                let s = format!("{f}").replace('-', "neg").replace('.', "_");
                format!("_{label}_{s}")
            }
        }
    }
    format!("validate_decimal{}{}", part("min", min), part("max", max))
}

fn register_decimal_range(
    ranges: &mut Vec<DecimalRange>,
    fn_name: &str,
    min: Option<f64>,
    max: Option<f64>,
) {
    if !ranges.iter().any(|r| r.fn_name == fn_name) {
        ranges.push(DecimalRange {
            fn_name: fn_name.to_string(),
            min,
            max,
        });
    }
}
