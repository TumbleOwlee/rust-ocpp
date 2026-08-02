# Codegen — Edge Cases and Known Limitations

Boundary behavior, error semantics, and behavior that is **ugly on purpose**.

Read this before "fixing" something in this area that looks wrong. An entry here
is a decision, not an oversight — reversing one is a normative change and goes
through gate 1.

---

## Empty identifier string

**What happens:** The identifier-string validator accepts `""`.
**Why:** The regex quantifier is `*`, not `+`, and the OCA schemas express
required-ness through field presence rather than through a minimum length on
`identifierString`. Emptiness is the schema's business, not the charset check's.
**Cites:** CG-R-006

## Sub-second precision is dropped on serialize

**What happens:** A date-time carrying milliseconds serializes without them;
round-tripping through the adapter is lossy.
**Why:** OCPP timestamps are specified to second precision. Emitting more
precision than the protocol defines invites peers to depend on it.
**Cites:** CG-R-001

## v2.0.1 emits no decimal range validators

**What happens:** `validator.rs` under v2.0.1 contains only the
identifier-string validator; the decimal range validators exist under v2.1 only.
**Why:** They are emitted per distinct bound pair found in the schemas, and the
2.0.1 schemas declare none. Their absence is generator output, not an omission.
**Cites:** CG-R-008
