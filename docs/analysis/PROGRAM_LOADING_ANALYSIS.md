# Program Loading + Analysis

Program Loading + Analysis is the Phase1 / Base1 / Fyr workflow for loading a program-like file into Phase1 for controlled diagnostics, dissection, examination, analysis, and forensic review.

This document is a development contract, not a completion claim.

## Current status

| Area | Status | Notes |
| --- | --- | --- |
| Public concept | Ready | Safe wording is `now in development`. |
| Command surface | Planned | Proposed command namespace is `analyze`. |
| Safe sample loading | Planned | First implementation target. No execution. |
| Static diagnostics | Planned | Deterministic fixture-first path. |
| Reports and evidence records | Planned | VFS/log-backed reports. |
| Fyr integration | Planned | Native programmable checks, VFS-oriented by default. |
| Base1 evidence path | Planned | Documentation-first, evidence-bound. |
| Dynamic execution | Future / restricted | Requires explicit isolation design; no default execution. |

## Ecosystem roles

- **Phase1** is the console and analysis environment.
- **Base1** is the system foundation and evidence-bound host/recovery track.
- **Fyr** is the Phase1-native programming language for programmable analysis workflows.

## Non-claims

This work does not initially claim:

- hardened malware sandboxing
- VM or container isolation
- safe execution of hostile binaries
- production forensic admissibility
- completed Base1 installer, recovery, or daily-driver readiness
- finished OS replacement behavior

## Proposed command vocabulary

```text
analyze status
analyze load <path>
analyze inspect <id>
analyze report <id>
analyze forget <id>
```

The first implementation slice should make `analyze load <path>` metadata-only. It must not execute the target.

## Workflow path

### W0 — Boundary and vocabulary

Goal: define the feature without overclaiming.

Deliverables:

- this document
- feature-status entry with conservative status
- `analyze` command vocabulary
- public wording: `Program Loading + Analysis — now in development`

Acceptance gate:

- Documentation distinguishes static inspection, simulated analysis, host-backed execution, and future isolation.

### W1 — Safe sample ingestion

Goal: allow a program-like file to be ingested into a Phase1 analysis registry without executing it.

Deliverables:

- sample record model
- VFS-backed sample metadata registry
- read-only load path
- audit-log entry for load events
- tests for missing file, empty file, large file, duplicate sample, and redaction

Sample record fields:

```text
id
path
size_bytes
sha256
source
loaded_at
trust_state
execution_state = not-executed
```

Acceptance gate:

- `analyze load` records metadata only and never executes the file.

### W2 — Static diagnostics and dissection

Goal: create deterministic diagnostic output from sample bytes and metadata.

Deliverables:

- file type guesser / magic-byte detection
- size, hash, and extension mismatch checks
- entropy-like score or simple byte-distribution score
- optional hex preview
- structured diagnostics report
- unknown/unsupported format behavior

Acceptance gate:

- Phase1 can produce a deterministic static report from fixture samples.

### W3 — Examination and report workflow

Goal: provide repeatable operator review surfaces.

Deliverables:

- `analyze inspect <id>` detail view
- `analyze report <id>` summary view
- saved report path such as `/var/log/analysis/<id>.report`
- evidence checklist: sample hash, source, timestamps, commands run, non-execution notice
- tests for report determinism

Acceptance gate:

- Same sample plus same commands produce stable report output.

### W4 — Fyr-native programmable analysis

Goal: let Fyr participate as the native analysis automation language without unsafe host execution.

Deliverables:

- Fyr analysis helper design note
- read-only sample metadata access from Fyr fixtures
- simple rules/checks written in `.fyr`
- test fixture proving Fyr analysis stays VFS-oriented by default

Acceptance gate:

- Fyr can express safe metadata checks without invoking host tools.

### W5 — Base1-backed evidence path

Goal: connect analysis records to Base1's evidence-bound track without claiming complete forensic platform status.

Deliverables:

- Base1 evidence boundary note
- recovery/host-boundary relationship documented
- optional dry-run evidence export design
- evidence package manifest draft

Acceptance gate:

- Docs explain what Base1 contributes now versus what remains future work.

### W6 — Future dynamic analysis design

Goal: define future dynamic behavior only behind explicit gates.

Deliverables:

- design note for VM/container/external sandbox integration
- safe-mode and trust-gate requirements
- hostile-code warning text
- no default dynamic execution

Acceptance gate:

- No hostile binary is executed by default; host-backed execution remains explicit and restricted.

## First implementation checklist

- [x] Add analysis docs directory and entry point.
- [x] Add Program Loading + Analysis development contract.
- [ ] Add feature-status row with conservative status.
- [ ] Add `analyze` command registration and help/man-page draft.
- [ ] Implement no-execute sample metadata registry.
- [ ] Add deterministic fixture samples.
- [ ] Add static diagnostics report.
- [ ] Add saved report path.
- [ ] Add Fyr fixture for metadata-only analysis.
- [ ] Add Base1 evidence-boundary note.
- [ ] Add tests and quality-gate coverage.

## Validation targets

General repository validation should continue to pass:

```bash
cargo fmt --all -- --check
cargo test --all-targets
sh scripts/quality-check.sh quick
```

Future focused tests should include:

```bash
cargo test -p phase1 --test analysis_load_contract
cargo test -p phase1 --test analysis_static_diagnostics
cargo test -p phase1 --test analysis_report_determinism
cargo test -p phase1 --test fyr_analysis_metadata_only
```

## Public wording boundary

Safe wording:

> Phase1 / Base1 / Fyr Program Loading + Analysis is now in development: a controlled inspection path for diagnostics, dissection, examination, analysis, and forensic review.

Avoid claiming:

> Phase1 safely runs malware.
> Phase1 is a hardened sandbox.
> Phase1 is a production forensic platform.
> Base1 is installer-ready or daily-driver ready because of this feature.
