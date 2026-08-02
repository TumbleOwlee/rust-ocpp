# Codegen — Requirements

The schema-to-Rust generator: schema intake, the intermediate representation, identifier naming, and the emitted source for the v2.0.1 and v2.1 type modules.

Normative. IDs are stable and append-only (CG-R-nnn) — never
renumber, never reuse a retired ID. Boundary behavior and stated limitations live
in [`edge-cases.md`](./edge-cases.md).

Requirements are added through the workflow in [`AGENTS.md`](../../../AGENTS.md):
gate 1 approves the "shall" text before any code is written.

---

## 1. Emitted date-time helper

**CG-R-001** — Serializing a UTC date-time through the emitted RFC3339 adapter
shall produce an RFC3339 string with whole-second precision and a literal `Z`
UTC designator, discarding any sub-second component.

**CG-R-002** — Deserializing through the emitted RFC3339 adapter shall accept
any valid RFC3339 timestamp, including one carrying a non-UTC offset, and shall
yield the equivalent instant expressed in UTC.

**CG-R-003** — Deserializing a string that is not a valid RFC3339 timestamp
shall fail with a deserialization error carrying the underlying parse message,
never a panic.

**CG-R-004** — The optional variant of the adapter shall serialize an absent
value as JSON `null`, and shall deserialize JSON `null` to an absent value.

**CG-R-005** — Deserializing JSON `null` through the non-optional adapter shall
fail with a deserialization error.

**CG-R-010** — The optional variant of the adapter shall serialize and
deserialize a present value with the same observable semantics as the
non-optional variant.

## 2. Emitted identifier-string validator

**CG-R-006** — The emitted identifier-string validator shall accept a string
composed solely of characters from `a-z`, `A-Z`, `0-9`, `*`, `+`, `=`, `:`,
`|`, `@`, `.`, `_`, `-`.

**CG-R-007** — The emitted identifier-string validator shall reject a string
containing any character outside that set — including a space, `/`, `#`, or any
non-ASCII character — by returning a validation error, never a panic.

## 3. Emitted decimal range validators

**CG-R-008** — A range validator emitted for a schema-declared `minimum` and/or
`maximum` shall accept a value inside the range and reject a value outside it,
returning a validation error that distinguishes below-minimum from
above-maximum.

**CG-R-009** — A range validator shall treat both declared bounds as inclusive:
a value exactly equal to the minimum or the maximum shall be accepted.
