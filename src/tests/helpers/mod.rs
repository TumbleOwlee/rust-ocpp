//! Tests for the generated `helpers` modules.
//!
//! The helpers under `src/v2_0_1/helpers/` and `src/v2_1/helpers/` are emitted by
//! `codegen/` and carry a "do not edit by hand" banner, so their tests cannot live
//! beside them as `#[cfg(test)] mod tests` — a regeneration would delete them.
//! They live here instead, and are specified under `docs/specs/codegen/`.

#[cfg(all(test, any(feature = "v2_0_1", feature = "v2_1")))]
mod datetime_rfc3339;
