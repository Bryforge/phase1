# White Arts roadmap

White Arts is the defensive-care roadmap for turning Phase1/Base1/Fyr maintenance into a tested, integrity-validated, and security-auditable workflow.

The roadmap is intentionally evidence-bound. It plans a corporate-ready operating posture, not a corporate certification claim.

## Roadmap ladder

```text
planned -> documented -> read-only checked -> locally validated -> CI validated -> reviewed -> release eligible
```

A White Arts feature does not advance unless its tests, docs, failure behavior, recovery notes, and claim boundary are present.

## W1 — Doctrine and vocabulary

Goal: define White Arts as defensive care.

Required outputs:

- `docs/white-arts/README.md`
- non-claim language
- command-surface boundary
- prohibited meanings

Status: initial documentation.

## W2 — Nominal-state matrix

Goal: define what "functional and nominal" means for each major surface.

Required coverage:

- Phase1 boot selector and shell
- VFS, `/proc`, `/dev`, and logs
- command registry, help, and man pages
- safe-mode and host-trust policy gates
- storage, Git, and Rust guarded workflows
- WASI-lite plugin surface
- Program Loading + Analysis metadata commands
- Portal and Nest metadata surfaces
- Fyr check, build, test, package, and docs tracks
- Base1 docs, scripts, dry-runs, recovery, and evidence surfaces
- website, status JSON, and release metadata
- CI, quality, and security workflows

## W3 — Integrity validation layer

Goal: make integrity checks repeatable and read-only by default.

Required checks:

- required docs exist
- required scripts exist and have valid shell syntax where applicable
- release metadata agrees across public surfaces
- generated docs blocks are present
- Base1 links and inventories are intact
- security and crypto docs preserve guardrails
- Fyr roadmap and fixtures are linked
- analysis commands preserve no-execute boundaries
- CI workflows preserve formatting, tests, security posture, and metadata checks

## W4 — Healing and maintenance model

Goal: define repair as a staged, reviewable process.

Allowed first-stage behavior:

- detect missing docs, links, scripts, tests, or metadata
- explain likely cause
- recommend a repair plan
- generate candidate patches in a branch
- require human review before promotion

Forbidden behavior:

- silent writes
- unsafe host mutation
- credential handling
- deleting recovery material
- auto-promoting repairs to stable branches
- bypassing safe-mode or trust gates

## W5 — Security audit movement

Goal: build a massive, repeatable security-audit movement that tracks findings and blocked claims.

Audit surfaces:

- threat model refresh
- command capability metadata
- safe-mode and host-trust gates
- secret redaction
- update path
- storage, Git, and Rust workflows
- WASI-lite plugin boundary
- Program Loading + Analysis metadata commands
- Fyr package/runtime safety
- Base1 recovery and dry-run surfaces
- crypto policy readiness
- CI and release metadata

## W6 — Read-only command surface

Goal: add command stubs only after the docs and matrix are tested.

Planned commands:

```text
white-arts status
white-arts inventory
white-arts check
white-arts explain <finding>
white-arts plan-repair <finding>
white-arts report
white-arts audit security
white-arts audit integrity
white-arts audit base1
white-arts audit fyr
```

First implementation rule: report-only, no mutation.

## W7 — Evidence reports and promotion gates

Goal: every White Arts result should be captured in a reviewable report.

A report must include:

```text
report id
scope
systems checked
commands run
nominal findings
integrity findings
security findings
repair candidates
blocked claims
review notes
promotion decision
```

## W8 — Base1 and recovery alignment

Goal: connect White Arts to Base1 as a safe-maintenance layer.

Initial Base1 scope:

- read-only recovery checklist review
- dry-run command checks
- evidence bundle validation
- hardware report completeness checks
- no destructive write support

## W9 — Fyr integration

Goal: use Fyr eventually as a native scripting surface for White Arts.

Initial Fyr scope:

- run safe checks
- validate package metadata
- generate reports
- avoid host mutation
- avoid privileged repair

## W10 — First implementation milestones

1. Add White Arts documentation index.
2. Add White Arts roadmap.
3. Add nominal-state matrix.
4. Add integrity validation plan.
5. Add healing and maintenance model.
6. Add security audit movement plan.
7. Add report template.
8. Add documentation guard tests.
9. Add `white-arts status` read-only command stub.
10. Add `white-arts inventory` read-only reporter.

## Non-claims

White Arts does not make Phase1, Base1, or Fyr:

- production hardened
- malware-safe
- forensic-admissible
- a certified sandbox
- a finished operating system
- installer-ready
- daily-driver ready
- cryptographically complete
