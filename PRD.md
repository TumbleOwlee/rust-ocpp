# PRD — rust-ocpp

Product framing for `rust-ocpp`. Normative behavior lives in
[`docs/specs/`](./docs/specs/); structure lives in
[`ARCHITECTURE.md`](./ARCHITECTURE.md). This document states *why* the project
exists and what it is and is not for — it does not restate requirements.

## Overview

OCPP 1.6, 2.0.1 and 2.1 message and datatype definitions for Rust, validated
against the official Open Charge Alliance JSON schemas.

The Open Charge Point Protocol is the wire protocol between a charging station
and a Charging Station Management System. `rust-ocpp` supplies the Rust type
layer for that wire format — serde-serializable request and response types,
datatypes and enumerations — so an implementor writes protocol logic instead of
transcribing a specification into structs.

*(Expand: who runs this, in what setting, against what. Keep it framing, not
requirements.)*

## Goals

*(TBD — the properties that decide design trade-offs. Each goal should be able to
settle an argument. The shape, grounded in this project:)*

- **Correctness against the authoritative source first.** Types follow the
  published OCPP specification and the official OCA JSON schemas, tested against
  vectors derived from those schemas rather than from our own output.
- **Versions are independent.** Each protocol version is a separate, feature-gated
  type set. Sharing a type across versions is a decision, never a convenience.
- **Generated where generation is honest.** v2.0.1 and v2.1 types come from the
  schemas via `codegen/`, so a schema correction propagates instead of being
  re-typed by hand.

## Non-goals

*(TBD — what this deliberately will not do. A non-goal here is what lets an agent
close a scope question without asking. Candidates to confirm:)*

- Transport. The crate defines types, not a WebSocket client or server.
- Session and state machine logic — connector state, transaction lifecycle,
  message routing — belongs to the consumer.

## Users

*(TBD — who consumes this and how: charging-station firmware, CSMS backends,
test harnesses and simulators.)*

## Success criteria

*(TBD — observable conditions under which this is working. Not metrics for their
own sake.)*

## Capability areas

The specification is split by area; each owns its behavior end to end.

| Area | Covers | ID prefix |
|---|---|---|
| [`codegen`](./docs/specs/codegen/) | The schema-to-Rust generator in `codegen/`, and the code it emits | `CG-R-*` |

Per-version areas for OCPP 1.6, 2.0.1 and 2.1 are not created yet; `OA-R-*`,
`OB-R-*` and `OC-R-*` are reserved for them. See
[`docs/specs/README.md`](./docs/specs/README.md) for how an area is created.

Cross-cutting concerns live in
[`docs/specs/non-functional-requirements.md`](./docs/specs/non-functional-requirements.md).
