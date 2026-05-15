# White Arts Command Stub Contract

## Purpose

This contract defines the first safe runtime-facing White Arts command surface before implementation. The initial command must be read-only, report-only, deterministic, and explicit about non-claims.

The first runtime slice should provide operator visibility into White Arts status without granting repair, host mutation, credential access, destructive writes, or autonomous healing.

## Initial command surface

```text
white-arts
white-arts status
white-arts help
white-arts inventory
white-arts check
white-arts report
white-arts audit security
white-arts audit integrity
white-arts audit base1
white-arts audit fyr
```

All commands in the first runtime slice are status or planned-action surfaces only.

## Required status output

The first `white-arts status` output should preserve these fields:

```text
phase1 white arts
status           : planned
mode             : defensive-read-only
mutation         : disabled
repair-policy    : staged-candidate-only
execution-state  : not-executed
host-execution   : disabled
network-access   : disabled
credential-access: disabled
sandbox-claim    : not-claimed
audit-scope      : report-only
integrity-scope  : docs-scripts-release-metadata
base1-scope      : read-only-recovery-and-evidence
fyr-scope        : metadata-and-reporting-only
claim-boundary   : evidence-bound-maintenance
```

## Planned command behavior

| Command | First behavior | Mutation allowed | Required boundary |
| --- | --- | --- | --- |
| `white-arts` | same as `white-arts status` | no | defensive-read-only |
| `white-arts status` | print status fields | no | evidence-bound-maintenance |
| `white-arts help` | list command surface and non-claims | no | no repair execution |
| `white-arts inventory` | planned nominal-state inventory report | no | report-only |
| `white-arts check` | planned read-only check summary | no | no host mutation |
| `white-arts report` | planned report template path | no | deterministic output |
| `white-arts audit security` | planned security audit movement summary | no | no exploitation |
| `white-arts audit integrity` | planned integrity audit summary | no | read-only verification |
| `white-arts audit base1` | planned Base1 evidence review summary | no | no device writes |
| `white-arts audit fyr` | planned Fyr metadata/reporting summary | no | no host execution |

## Forbidden behavior

The first runtime stub must not:

- execute host tools;
- access the network;
- collect credentials;
- silently modify files;
- write boot artifacts;
- delete recovery material;
- promote candidate repairs;
- imply malware safety;
- imply forensic admissibility;
- imply production hardening.

## Audit and Optics expectations

When runtime wiring is added, command execution should record a sanitized audit event such as:

```text
white-arts.status mode=read-only mutation=disabled
```

Optics rails may show White Arts state only as a visible status label, for example:

```text
C STATUS HUD   white-arts=planned integrity=report-only repair=staged-candidate-only
D BOTTOM HUD   mutation=disabled host-tools=gated claim=evidence-bound-maintenance
```

## Promotion rule

Runtime implementation should not begin with repair behavior. It should promote in this order:

```text
contract-tested -> runtime-status-stub -> inventory-reporter -> check-reporter -> candidate-repair-planner -> reviewed-repair-flow
```

Every promotion requires focused tests, docs, non-claim language, and a failure behavior note.
