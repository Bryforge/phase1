# White Arts security audit movement

The White Arts security audit movement is a large, repeatable defensive audit plan for Phase1/Base1/Fyr.

It organizes audit work into visible scopes, evidence requirements, blocked claims, and review decisions.

## Audit principles

- security and usability are a shared goal
- safe defaults must remain understandable
- findings must distinguish facts from inferences
- reports must avoid secrets
- audit status must not overclaim hardening
- remediation must be staged and reviewed

## Audit report fields

Every audit report should include:

```text
audit id
scope
risk level
required evidence
commands run
findings
blocked claims
required fixes
reviewer notes
status
```

## Audit scopes

| Audit id | Scope | Risk level | Required evidence | Blocked claims |
| --- | --- | --- | --- | --- |
| WA-SEC-001 | Threat model refresh | high | trust model, security docs, current command surfaces | hardened-by-design |
| WA-SEC-002 | Command capability metadata | high | registry tests, policy tests, host-tool gates | secure command execution |
| WA-SEC-003 | Safe-mode and host-trust gates | critical | safe-mode tests, storage tests, denial output | safe host mutation |
| WA-SEC-004 | Secret redaction | critical | redaction tests, logs, history sanitizer | private-by-default beyond tested surfaces |
| WA-SEC-005 | Update path | high | updater tests, edge/stable target evidence | safe automatic update |
| WA-SEC-006 | Storage/Git/Rust workflows | high | storage smoke tests and input validation | hardened runtime execution |
| WA-SEC-007 | WASI-lite plugin boundary | high | plugin manifest tests, no host shell passthrough | certified sandbox |
| WA-SEC-008 | Program Loading + Analysis | critical | load/list/inspect metadata-only tests | malware-safe analysis |
| WA-SEC-009 | Fyr package/runtime safety | high | Fyr VFS and redaction tests | production runtime safety |
| WA-SEC-010 | Base1 recovery and dry-runs | critical | Base1 dry-run and recovery validation tests | installer-ready or recovery-complete |
| WA-SEC-011 | Crypto policy readiness | critical | crypto registry/provider/profile docs | cryptographically complete |
| WA-SEC-012 | CI and release metadata | high | workflow and release metadata tests | release-qualified without CI |

## Movement phases

```text
inventory -> threat review -> evidence collection -> findings -> remediation plan -> validation -> review -> promotion decision
```

## Finding levels

```text
informational
low
medium
high
critical
blocked-claim
```

## Required non-claims

Until evidence says otherwise, the audit movement must preserve:

- not production hardened
- not malware-safe
- not forensic-admissible
- not a certified sandbox
- not installer-ready
- not daily-driver ready
- not cryptographically complete

## Promotion rule

A security-sensitive promotion requires:

- focused tests
- full test suite or documented exception
- non-claim review
- failure behavior review
- rollback/recovery path
- human review notes
