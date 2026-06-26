//! Schema-agnostic internal representation produced by a frontend and consumed
//! by the backend. A new schema style only needs a new frontend targeting these
//! types; the backend is reused unchanged.

/// Which generated sub-module a named type lives in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Module {
    Datatypes,
    Enumerations,
}

impl Module {
    pub fn dir(self) -> &'static str {
        match self {
            Module::Datatypes => "datatypes",
            Module::Enumerations => "enumerations",
        }
    }
}

#[derive(Debug, Clone)]
pub struct IrModel {
    pub version: String, // e.g. "v2_0_1"
    pub types: Vec<IrType>,
    pub messages: Vec<IrMessage>,
    /// Distinct (min,max) decimal-range validators to emit in helpers.
    pub decimal_ranges: Vec<DecimalRange>,
}

#[derive(Debug, Clone)]
pub enum IrType {
    Struct(IrStruct),
    Enum(IrEnum),
}

impl IrType {
    pub fn name(&self) -> &str {
        match self {
            IrType::Struct(s) => &s.name,
            IrType::Enum(e) => &e.name,
        }
    }
    pub fn module(&self) -> Module {
        match self {
            IrType::Struct(_) => Module::Datatypes,
            IrType::Enum(_) => Module::Enumerations,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IrStruct {
    pub name: String,
    pub doc: Option<String>,
    pub fields: Vec<IrField>,
}

impl IrStruct {
    pub fn derives_validate(&self) -> bool {
        self.fields.iter().any(|f| f.has_validation())
    }
}

#[derive(Debug, Clone)]
pub struct IrField {
    pub rust_name: String,
    /// Original JSON key (retained for debugging/round-trip clarity).
    #[allow(dead_code)]
    pub json_key: String,
    pub ty: IrTypeRef,
    pub optional: bool,
    pub doc: Option<String>,
    pub constraints: Vec<Constraint>,
    pub rename: Option<String>,
}

impl IrField {
    /// Whether this field contributes a `#[validate(...)]` attribute (and thus
    /// requires the enclosing struct to derive `Validate`). Integer range
    /// validators are derived from the `Int` bounds, not from `constraints`.
    pub fn has_validation(&self) -> bool {
        if !self.constraints.is_empty() {
            return true;
        }
        matches!(
            &self.ty,
            IrTypeRef::Int { min, max } if min.is_some() || max.is_some()
        )
    }
}

#[derive(Debug, Clone)]
pub enum IrTypeRef {
    Named(String),
    Str,
    Int { min: Option<i64>, max: Option<i64> },
    Number, // rust_decimal::Decimal
    Bool,
    DateTime,
    Array(Box<IrTypeRef>),
}

#[derive(Debug, Clone)]
pub enum Constraint {
    Length { min: Option<i64>, max: Option<i64> },
    DecimalRange { fn_name: String },
    IdentifierString,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DecimalRange {
    pub fn_name: String,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct IrEnum {
    pub name: String,
    pub doc: Option<String>,
    pub variants: Vec<IrVariant>,
    pub default_variant: usize,
}

#[derive(Debug, Clone)]
pub struct IrVariant {
    pub rust_name: String,
    /// Original JSON enum value (retained for debugging clarity).
    #[allow(dead_code)]
    pub json_value: String,
    pub doc: Option<String>,
    pub rename: Option<String>,
}

#[derive(Debug, Clone)]
pub struct IrMessage {
    /// File stem, snake_case, e.g. "authorize".
    pub file_stem: String,
    pub request: Option<IrStruct>,
    pub response: Option<IrStruct>,
}
