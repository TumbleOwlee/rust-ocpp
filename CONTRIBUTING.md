# Contributing to rust-ocpp

## Setup

Install the toolchain via [rustup.rs](https://rustup.rs/), then:

```sh
cargo build --all-features
```

There is no default feature — every protocol version is explicit (`v1_6`,
`v2_0_1`, `v2_1`).

For the coverage gate you also need `cargo-llvm-cov`:

```sh
cargo install cargo-llvm-cov --locked
```

Optionally install [lefthook](https://github.com/evilmartians/lefthook) and run
`lefthook install` to get the spec and requirement-ID reminders locally.
`.pre-commit-config.yaml` carries the `fmt` / `cargo check` / `clippy` hooks.

## Project layout

See [`ARCHITECTURE.md`](./ARCHITECTURE.md) for the module map and data flow, and
[`PRD.md`](./PRD.md) for the product framing.

`rust-ocpp` is **spec-driven**: [`docs/specs/`](./docs/specs/) is the
authoritative specification of what it must do, split by capability area. The
code is expected to conform to it. Before changing behavior, read the relevant
area's `requirements.md` and `edge-cases.md`.

Note that the `v2_0_1` and `v2_1` type modules are **generated** by `codegen/`
from the official OCA JSON schemas. Do not hand-edit them — change the generator
and regenerate, or the edit is lost on the next run.

## Test-driven development

Write the test first, watch it fail, then implement. A test written after the
code it covers asserts what you built rather than what the specification
requires — derive expected values from the authoritative source (the OCPP
specification and the official OCA JSON schemas), not from a debug print of your
own implementation.

Every new or changed requirement ships with at least one test whose doc comment
cites the requirement ID, directly beside the test declaration:

```rust
#[test]
/// OB-R-012 — The checksum is computed over the full frame excluding the checksum field.
fn ut_checksum_excludes_trailer() { /* … */ }
```

Line coverage must stay at or above **80%**, enforced in CI. Coverage is a floor,
not a goal — never pad it with tests that execute code without asserting on it.

## Before submitting

Please make sure the following pass locally:

```sh
cargo fmt --check
cargo clippy --all-features --all-targets -- -D warnings
cargo check --all-features
cargo test --all-features
cargo llvm-cov --all-features --fail-under-lines 80
```

CI runs these on every push **and every pull request**, so anything the
pre-commit hook would reject is rejected by CI too.

## Pull requests

- Branch off `main` and open your PR against `main`. Branch naming:
  `<type>/<slug>` with a conventional-commit type (`feat/`, `fix/`, `docs/`).
- Keep PRs focused — one feature or fix per PR.
- Add or update tests for behavior changes. Unit tests: `#[cfg(test)] mod tests`
  at the bottom of the file under test, functions named `ut_*`. Integration
  tests: `src/tests/`, functions named `it_*`.
- **Update the spec in the same PR.** When you change behavior, update the
  relevant `docs/specs/<area>/` file(s) — they are the authoritative source, not a
  one-time snapshot. New requirements get a fresh, appended ID (never renumber or
  reuse). A behavior change with no spec change is incomplete.
- Reference requirement IDs in the PR body.
- Update the README when you change the public surface.
- PRs are merged to `main` by **squash merge**.

Agents working in this repo follow the fuller gated workflow in
[`AGENTS.md`](./AGENTS.md); human contributors are welcome to, but the checks
above are the hard requirements.

## Reporting issues

Open an issue with steps to reproduce, the version (or commit), your platform,
and which OCPP version and feature flags you build with.
