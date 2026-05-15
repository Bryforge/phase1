# White Arts

White Arts is the Phase1/Base1/Fyr defensive-care track for system integrity, healing, maintenance, nominal-state verification, and security audit readiness.

It is a planning and validation surface first. It does not grant autonomous repair power, host mutation power, execution of untrusted samples, sandbox certification, malware-safety claims, production forensic admissibility claims, or finished hardening claims.

## Purpose

White Arts exists to make the project easier to keep healthy without overstating what has been proven.

Core purposes:

- verify that major systems are functional and nominal
- keep integrity checks repeatable and reviewable
- surface maintenance findings in plain language
- prepare safe repair plans before any mutation
- connect diagnostics, Base1 recovery, Fyr automation, and security review into one evidence-bound workflow
- provide audit reports that separate implemented facts from future plans

## Meaning of White Arts

In this repository, White Arts means defensive maintenance only:

- integrity verification
- diagnostics
- nominal-state reporting
- healing proposals
- recovery readiness
- security audit support
- safe operator guidance
- evidence-bounded promotion gates

White Arts does not mean:

- offensive exploitation
- stealth or persistence tooling
- credential collection
- destructive repair
- silent mutation
- bypassing safe-mode or host-trust gates
- malware-safe execution
- production forensic certification

## Required boundary

Every White Arts surface must preserve these claims:

```text
mode             : defensive
mutation         : review-required
execution-state  : not-executed unless explicitly proven safe for that command
host-execution   : disabled by default
sandbox-claim    : not-claimed
repair-policy    : staged-candidate-only until reviewed
claim-boundary   : evidence-bound-maintenance
```

## Current stage

White Arts is documentation-first.

The first stage is to define the roadmap, nominal-state matrix, integrity plan, healing model, security audit movement, protocols and guardrails, open security server suite planning, maintenance TODOs, and report templates before wiring any command surface.

## Planned command surface

Initial command planning is read-only/report-only:

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

No command should silently modify repository files, host settings, credentials, boot artifacts, recovery material, or release metadata.

## Documents

- [Roadmap](ROADMAP.md)
- [Nominal-state matrix](NOMINAL_STATE_MATRIX.md)
- [Integrity validation plan](INTEGRITY_VALIDATION_PLAN.md)
- [Healing and maintenance model](HEALING_MAINTENANCE_MODEL.md)
- [Security audit movement](SECURITY_AUDIT_MOVEMENT.md)
- [Command stub contract](COMMAND_STUB_CONTRACT.md)
- [Inventory reporter contract](INVENTORY_REPORTER_CONTRACT.md)
- [Protocols and security guardrails](PROTOCOLS_AND_GUARDRAILS.md)
- [Open security server suite](OPEN_SECURITY_SERVER_SUITE.md)
- [Maintenance TODO](TODO.md)
- [Report template](REPORT_TEMPLATE.md)
