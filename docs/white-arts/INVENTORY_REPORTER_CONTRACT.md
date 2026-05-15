# White Arts Inventory Reporter Contract

## Purpose

The White Arts inventory reporter is the first planned read-only reporter after the `white-arts status` stub. Its job is to show what Phase1/Base1/Fyr surfaces are covered by nominal-state and integrity planning without performing repairs or host actions.

This contract is documentation and test evidence only. It does not wire a live `white-arts inventory` command yet.

## Planned command

```text
white-arts inventory
```

The first runtime implementation should be read-only and deterministic.

## Required report header

```text
phase1 white arts inventory
status           : planned
mode             : defensive-read-only
mutation         : disabled
host-execution   : disabled
network-access   : disabled
repair-policy    : staged-candidate-only
claim-boundary   : evidence-bound-maintenance
```

## Required inventory columns

The reporter should preserve this table shape:

```text
system
owner-surface
nominal-signal
test-command
integrity-gate
failure-behavior
recovery-path
claim-boundary
```

## Required inventory rows

The first inventory report should include rows for these surfaces:

| System | Owner surface | Nominal signal | Claim boundary |
| --- | --- | --- | --- |
| Phase1 boot and shell | boot selector, Optics shell, command loop | boots to explicit shell state | active edge console, not finished OS |
| VFS and kernel model | VFS, proc, dev, audit log | deterministic simulated surfaces | simulator model, not kernel replacement |
| Policy and host gates | safe-mode, host trust, capability metadata | host tools gated by policy | no implicit host access |
| Command registry | help, man, completions, capabilities | discoverable command metadata | docs and registry evidence only |
| Storage and host workspace | storage helper, Git/Rust guarded workflows | validated workspace and redaction rules | no arbitrary host directory access |
| WASI-lite plugins | wasm list, inspect, validate, run | no host shell, no host network | lightweight runtime, not hardened sandbox |
| Program analysis | analyze load/list/inspect metadata | not-executed sample metadata | no malware safety or forensic claim |
| Nest and portal surfaces | nest, portal, phase navigation contracts | visible local context metadata | internal navigation only |
| Fyr language | check, build, test, run, staged docs | deterministic VFS-oriented behavior | not self-hosting or privileged repair |
| Base1 evidence track | docs, dry-runs, recovery validation | read-only evidence chain | not installer-ready or daily-driver ready |
| Security and crypto docs | security policy, crypto roadmaps, provider registry | guardrails documented and tested | not cryptographically complete |
| Website and release metadata | status JSON, README, release docs | public status matches evidence | no overclaiming public status |
| CI and quality gates | Rust workflow, xtask, quality scripts | fmt/check/test/validation pass | CI evidence, not certification |
```

## Output rules

The inventory reporter must:

- prefer deterministic text output;
- use text labels, not color or icons alone;
- avoid host execution;
- avoid network access;
- avoid collecting credentials;
- avoid writing files;
- avoid deleting or repairing material;
- avoid claiming malware safety, forensic admissibility, production hardening, or OS completion.

## Audit expectation

When runtime wiring is added, a status-only audit event may be recorded:

```text
white-arts.inventory mode=read-only mutation=disabled rows=<count>
```

The audit event must not include raw secrets, credential-bearing paths, unredacted command bodies, or host output.

## Promotion rule

The inventory reporter may promote only after:

```text
contract-tested -> runtime-inventory-stub -> deterministic-output-tested -> quality-gate-linked -> reviewed
```

Repair planning remains a later staged-candidate feature and must not be included in the inventory reporter.
