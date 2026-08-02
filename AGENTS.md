# AGENTS.md

Router for AI coding agents. Read first.

## Repo

`rust-ocpp` — OCPP 1.6, 2.0.1 and 2.1 message and datatype definitions for Rust,
validated against the official Open Charge Alliance JSON schemas. Rust library
crate. Product framing: [`PRD.md`](./PRD.md). Structure:
[`ARCHITECTURE.md`](./ARCHITECTURE.md).

## Spec-driven

- `docs/specs/` is authoritative. Code conforms to the spec, not the reverse.
- Read the area's `requirements.md` **and** `edge-cases.md` before editing that area.
  `edge-cases.md` records deliberate ugliness — check it before "fixing" something.
- A behavior change with no spec change is incomplete.
- `main` never holds an unfinished spec: a requirement on `main` describes code that
  exists and is tested. A branch may hold a spec commit ahead of its code; the squash
  merge keeps that off `main`.
- Pre-existing spec/code disagreement that is not the task you were given: stop, raise it
  separately. Folding it in widens already-approved work and skips its own review.
- Specs carry no `file:line`. Locate code with search tools.
- Requirement IDs are stable and append-only. Cite them in commits and PRs.

## TDD — fixed order within every stage

1. Write the test. Its doc comment cites the requirement ID (`/// OB-R-012 — …`).
2. Run it, watch it fail for the right reason, report the failure. A wrong assertion, a
   test-side compile error, or a pass before the code exists proves nothing.
3. Minimum implementation that passes.
4. Refactor green.

- Implementation without a preceding failing test: not done. Test written afterwards to
  fit the code: not done.
- Derive expected values from the authoritative source — the OCPP specification and the
  official OCA JSON schemas — never from a debug print of your own implementation.
- Coverage floor 80% of lines, CI-gated on every push and PR. A floor, not a target —
  never inflate it with tests that execute code without asserting.

## Workflow

Triggers on **behavior change, any size**: a new public function, a changed default, a
new error variant, any observable semantics. Not a behavior change: refactor, rename,
perf work with identical semantics, tests, docs — no gates, just do it. Size sets the
number of stages, never whether the gates exist.

- Replaces any generic workflow skill (`/workflow`); do not run one. `docs/specs/` is
  already the PRD and the design record.
- Branch off `main`, never commit to `main`. `<type>/<slug>`, type ∈ {`feat`, `fix`, `docs`}.
- Delegate gates to agents. **All agents run Sonnet or better**, planning and
  implementation alike. Weaker models stop mid-plan and call the rest future work, commit
  a stub as a "green" checkpoint, and report a hanging test as verified.
- **One git worktree per issue, per agent.** Two agents in one checkout interleave commits
  and stage each other's work — a branch is not a working tree. Create before the agent
  starts, remove after its branch merges.
- The plan is a contract. An implementer who finds it wrong stops and reports; it does not
  improvise a different design.
- **An agent's report of its own verification is not verification.** Re-run the tools.

Verify before every approval request:

- **A plan** — quoted requirement text matches the file, appended IDs are genuinely
  unused, nothing contradicts an existing requirement, and it proposes what was asked.
- **An implementation** — re-run the full build/test/lint/coverage gauntlet
  yourself in the agent's worktree, read the code the report describes, check ID citations
  sit directly beside their test declarations, mutation-check any test written after its
  implementation, diff the spec against what was approved.

### Gate 1 — spec diff. Stop for approval.

- Propose the normative text itself: the "shall" statements with appended IDs, plus
  `edge-cases.md` entries. Ready to land, not prose about intent.
- Observable design is spec: public type and function signatures, the error enum, feature
  gating, configuration keys.
- Bug fix where the spec is right and the code is wrong: no diff — state the violated
  requirement and continue.

### Gate 1b — tracking issue. Stop for approval.

- Search `gh issue list` and closed issues for the same goal. Reuse what exists and
  reference its number; never open a second. Otherwise draft, get approval, `gh issue create`.
- Title: plain language a maintainer can scan. Not a slug, an ID, or a commit subject.
- Self-contained — the spec is not pushed yet, so quote the full normative text beside
  each new ID, each changed requirement as old → new, plus `api-contract.md` and
  `edge-cases.md` entries. An ID with no text is useless.
- Goal and normative changes only. No implementation detail (structure, files, functions,
  approach) — that belongs to gate 2 and the PR.
- `##` sections, not prose: `## Background`/`## Why`, `## Scope`, `## Goal`, more as
  warranted. Compact enumerations, grouped ID ranges. Same shape for PR bodies.

### Write the spec into the working tree

Never marked unfinished — the file holds only normative text. The plan tracks what lacks
a passing test.

### Gate 2 — implementation plan. Stop for approval.

Stages; file-level steps; a table mapping each new requirement ID to the test that pins
it; a **Verification** section naming the method (unit tests alone / schema validation
against the official OCA JSON schemas / a run against a real CSMS); expected commits;
expected coverage impact.

### Implement, stage by stage

- TDD order above. A stage is a green checkpoint: it builds, tests pass, lint is
  clean, coverage ≥ 80%. Commit every green stage — that is what makes the plan
  resumable.
- Stage messages stay cheap; they are squashed. The squash message carries the requirement
  IDs and the why. The spec is the first stage and the first commit.
- Every new or changed requirement ships ≥ 1 test citing its ID.
- Every existing test that pins observable behavior cites its requirement. Tests of pure
  internal or helper detail may stay untagged. Behavior no requirement states means the
  requirement is missing — add it (gate 1), never attach a loose ID.
- The citation goes directly beside the test declaration, immediately above the function
  body. Each ID appears at most once per test.
- Not done until the Verification method has been run and its outcome reported. Waiving it
  requires asking.

### Reconcile the spec

Behavior differing from what gate 1 approved is normative and **re-opens gate 1**: show
the diff, state what forced it, get approval before committing. A wrong cross-reference or
clumsy wording is editorial — no approval needed. Always report the final spec diff.

### Gate 3 — review. Stop for approval.

Before proposing a PR, run an independent review of the branch in a **separate agent**
that did not write the code — a reviewer sharing the implementer's context reproduces its
blind spots. Give it the diff, the approved spec text, and this file. It reports on:

- **Spec fidelity** — every approved requirement implemented, nothing implemented that was
  not approved (scope creep), no requirement pinned by a test that does not actually
  exercise it.
- **Standards** — the conventions below, test naming and ID citation, error handling.
- **TDD honesty** — tests that could pass against an empty implementation, assertions on
  the implementation's own output, coverage padded by tests that execute without asserting.

Re-run the verification yourself, then report the findings and the fixes. Findings the
user should decide on are raised, not silently fixed.

### Gate 4 — pull request. Stop for approval.

- Verification run and reported, then **ask whether to open a PR** — the user may want
  their own manual run first; do not pre-empt it.
- Then draft title and body, get approval of that text, then push and open the PR.
- Title in plain language, same style as the issue. Body is the implementation: the why,
  the requirement IDs, how the issue was resolved (the approach and structure the issue
  omitted), the verification actually performed, the coverage number, `Closes #<issue>`.

### Merge

Squash merge to `main`, so stage commits — including the spec commit that ran ahead of its
code — never reach `main`. Remove the worktree after the merge.

## Where to look for task X

| Task touches | Read | ID prefix |
|---|---|---|
| OCPP 1.6 messages and types | [`docs/specs/v1_6/`](./docs/specs/v1_6/) | `OA-R-*` |
| OCPP 2.0.1 messages, datatypes, enumerations | [`docs/specs/v2_0_1/`](./docs/specs/v2_0_1/) | `OB-R-*` |
| OCPP 2.1 messages, datatypes, enumerations (WIP) | [`docs/specs/v2_1/`](./docs/specs/v2_1/) | `OC-R-*` |
| The schema-to-Rust generator in `codegen/` | [`docs/specs/codegen/`](./docs/specs/codegen/) | `CG-R-*` |
| Platforms, toolchain, performance posture, security, versioning, testing conventions | [`docs/specs/non-functional-requirements.md`](./docs/specs/non-functional-requirements.md) | `NF-R-*` |
| Module graph, data flow, concurrency model | [`ARCHITECTURE.md`](./ARCHITECTURE.md) | — |
| Contribution workflow, conventions | [`CONTRIBUTING.md`](./CONTRIBUTING.md) | — |

## Build / test / lint

```sh
cargo fmt --check
cargo clippy --all-features --all-targets -- -D warnings
cargo check --all-features
cargo test --all-features
cargo llvm-cov --all-features --fail-under-lines 80
```

Narrow the loop while iterating:

```sh
cargo test ut_name                     # one unit test
cargo test --features v2_0_1           # one protocol version only
cargo check -p codegen                 # typecheck the generator alone
cargo llvm-cov --all-features --html   # browsable per-line coverage
```

Run the full set before considering work done. `lefthook` carries the spec and
requirement-ID reminders; `.pre-commit-config.yaml` carries `fmt`, `cargo check` and
`clippy`; CI runs the full set on every push and pull request.

## Conventions

- Unit tests: `#[cfg(test)] mod tests` at the bottom of the file under test, functions
  named `ut_*`.
- Integration tests: `src/tests/`, functions named `it_*`. Schema-validation tests live
  in `schema_validation.rs` per version and validate against the official OCA JSON
  schemas.
- **Types under `src/v2_0_1/` and `src/v2_1/` are generated** by `codegen/` from the OCA
  schemas. Hand edits are overwritten on the next generation run — change the generator,
  then regenerate. This is a scope boundary; see below.
- Do not split a file for size alone. A split separates distinct responsibilities,
  improves navigability, or cuts coupling. Cohesive files and flat generated data stay
  whole.
- Start each implementation stage by listing the functionality it needs and searching
  crates.io. Report downloads, last release, maintenance state, and recommend — do not
  default to hand-rolling. Adding the dependency is a scope boundary, so the finding goes
  to the user, not to `Cargo.toml`.
- Errors are typed, never stringly. A new failure mode is a new error type, which is
  public API, which is spec (gate 1).
- Hostile, malformed or truncated JSON produces a typed deserialization error, never a
  crash or an unbounded allocation. Test every deserialization path with truncated and
  out-of-range input.
- Edition 2021, stable toolchain (`rust-toolchain.toml`). MSRV is a non-functional
  requirement — raising it is normative.
- No bare `unwrap` outside tests; `expect("why this cannot fail")`.
- `#[non_exhaustive]` on public error enums so adding a variant is not breaking.
- Prefer typed handling over `serde_json::Value` — the compiler must catch a wrong field
  name, not the wire. This holds even when it forces duplication across protocol
  versions: `v1_6`, `v2_0_1` and `v2_1` are separate type sets by design, and sharing a
  type between them is a normative decision, not a cleanup.
- Each protocol version sits behind its own feature flag (`v1_6`, `v2_0_1`, `v2_1`) and
  there is no default feature. A type must compile with only its own version enabled.

## Scope boundaries — ask before

- **Adding a dependency.** The crate is deliberately close to dependency-free (`serde`,
  `chrono`, `rust_decimal`, `validator`). Report the finding; do not edit `Cargo.toml`.
- **Breaking the public type surface.** Renaming or removing an exported type, changing a
  field's type or optionality — semver-breaking for downstream CSMS and charging-station
  implementations.
- **Hand-editing generated types.** `src/v2_0_1/` and `src/v2_1/` come from `codegen/`
  against the OCA schemas. Fix the generator instead; a hand edit is lost on regeneration.
- **Adding a feature flag or a protocol version.** A new OCPP version or optional feature
  changes the build matrix, CI, and the crate's published contract.
