# White Arts report template

Use this template for White Arts integrity, maintenance, healing, and security audit reports.

```text
white-arts report
report id        : <id>
status           : nominal | warning | blocked | unknown
scope            : <scope>
systems checked  : <count>
commands run     : <commands>
nominal findings : <summary>
integrity findings: <summary>
security findings: <summary>
repair candidates: <summary>
blocked claims   : <claims>
mutation         : none | staged-candidate | reviewed
host-execution   : disabled by default
repair-policy    : staged-candidate-only
rollback path    : <path or not-applicable>
review notes     : <notes>
promotion decision: blocked | hold | eligible
claim-boundary   : evidence-bound-maintenance
```

## Required sections

### Scope

Name the checked system, command surface, documentation surface, or release surface.

### Evidence

List commands, files, reports, and tests used as evidence.

### Findings

Separate findings into:

- nominal findings
- integrity findings
- security findings
- maintenance findings
- blocked claims

### Repair candidates

Repair candidates must be staged and reviewable. Do not present a candidate repair as already promoted.

### Rollback path

Every promoted repair needs a rollback or recovery note.

### Decision

Use one of:

```text
blocked
hold
eligible
```

## Non-claims

A White Arts report is not a certification, external audit, production hardening proof, malware-safety proof, forensic admissibility proof, or installer-readiness proof.
