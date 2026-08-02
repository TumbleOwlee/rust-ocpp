# Architecture — rust-ocpp

How `rust-ocpp` is put together: the module map, how data moves through it, and
the concurrency model. *What* it must do is in [`docs/specs/`](./docs/specs/);
*why* it exists is in [`PRD.md`](./PRD.md).

This file describes structure, which changes with refactors. It carries no
normative "shall" statements — those belong to the specs, and only the specs.

## Layout

Rust library crate, stack: Rust (stable toolchain, pinned via
`rust-toolchain.toml`). Cargo workspace: the root crate plus `codegen`.

| Module | Responsibility |
|---|---|
| `src/lib.rs` | Crate root. Feature-gates each protocol version module; no logic. |
| `src/v1_6/` | OCPP 1.6: `messages/`, `types/`. Hand-written. |
| `src/v2_0_1/` | OCPP 2.0.1: `messages/`, `datatypes/`, `enumerations/`, `helpers/`. Generated from the OCA schemas. |
| `src/v2_1/` | OCPP 2.1 (WIP), same shape as `v2_0_1`. Generated. |
| `src/tests/` | Test-only module (`#[cfg(test)]`), including `schema_validation/` and its bundled `schemas/`. |
| `codegen/` | Separate workspace member: reads OCA JSON schemas and emits the `v2_0_1` / `v2_1` type modules. Not published as part of the library. |

Each protocol version module maps to the spec area of the same name;
`codegen/` maps to the `codegen` area.

*(TBD — expand per sub-module as behavior is specified through gate 1.)*

## Data flow

There is no runtime pipeline: the crate is a type layer. Data moves in two
directions, both through serde.

- **Inbound** — a consumer deserializes a JSON payload into a message type; serde
  and `validator` reject fields that do not match the schema-derived shape.
- **Outbound** — a consumer constructs a message type and serializes it to JSON.

A third flow is build-time rather than runtime: `codegen/` reads an OCA JSON
schema (`ir.rs` → intermediate representation, `naming.rs` → Rust identifiers,
`backend.rs` → emitted source) and writes the generated version modules into
`src/`.

*(TBD — expand once the codegen area's requirements land.)*

## Concurrency model

Single-threaded and synchronous. The crate defines data types and derives; it
owns no state, spawns nothing, and performs no I/O. All concurrency belongs to
the consumer.

## Error handling

Failures surface as `serde` deserialization errors and `validator` validation
errors on the consumer's side; the crate itself returns no `Result` from a public
entry point today.

*(TBD — if the crate grows a public fallible surface, its error type is typed and
`#[non_exhaustive]`, and it is specified in the owning area's `api-contract.md`.)*

## Testing seams

There is no external world to abstract. The one seam that matters is the
**bundled OCA JSON schemas** under `src/tests/schema_validation/schemas/`: tests
validate serialized types against those files rather than against expectations
typed out by hand, which is what keeps the type set honest to the published
specification. The schemas are checked in deliberately so the test suite is
offline and reproducible — do not replace them with network fetches.

`codegen/` reads the same family of schemas, so a generator test and a
schema-validation test can disagree; that disagreement is a real defect, not
duplication to be cleaned up.
