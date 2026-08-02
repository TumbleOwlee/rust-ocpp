# Non-Functional Requirements

Cross-cutting requirements that belong to no single capability area: platforms,
toolchain, performance posture, security, versioning, and testing conventions.

IDs are stable and append-only (`NF-R-nnn`). See [`README.md`](./README.md).

Requirements are added through the workflow in [`AGENTS.md`](../../AGENTS.md) —
gate 1 approves the "shall" text before any code is written. Nothing here is
fabricated ahead of that.

---

## 1. Platforms and toolchain

*(Empty. The first requirement lands through gate 1 — e.g. the supported
platforms, the MSRV and where it is declared, the pinned toolchain, `no_std`
posture.)*

## 2. Performance

*(Empty. State a posture, not a number you cannot measure: what is allowed to
allocate, what must not block, what is explicitly not optimised.)*

## 3. Security

*(Empty. Input trust boundaries — every OCPP payload arrives from an untrusted
peer — what must never panic or allocate unbounded, dependency policy,
unsafe-code policy.)*

## 4. Versioning and release

*(Empty. Crate versioning scheme, what constitutes a breaking change to the
public type surface, how OCPP protocol versions map to feature flags, changelog
policy.)*

## 5. Testing conventions

*(Empty. Test naming, the 80% coverage floor and where it is enforced, schema
validation against the bundled OCA schemas, what may not run in CI.)*
