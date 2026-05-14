# White Arts protocols and security guardrails

Status: documentation-first  
Scope: Phase1 / Base1 / Fyr defensive maintenance, security posture, and open security server planning  
Runtime claim: not implemented unless a later command surface proves otherwise  
Security claim: not hardened, not certified, not production-ready  

## Purpose

This document records the standing White Arts protocols and security guardrails for future implementation work.

White Arts exists to keep the system functional, nominal, integrity validated, maintainable, and safety-reviewed without overstating what has been proven.

These rules apply to White Arts docs, future commands, security-server planning, healing workflows, audit reports, and any future Fyr or Base1 automation that touches maintenance posture.

## Protocol 0 — Evidence before claim

No White Arts surface may claim protection, containment, hardening, malware safety, corporate compliance, or production readiness unless the claim has matching evidence.

Required evidence for any promoted claim:

```text
implemented behavior
test coverage
negative tests
safe failure behavior
operator documentation
validation report
review notes
rollback or recovery notes
promotion decision
```

Unknown remains unknown. Planned remains planned. Experimental remains experimental.

## Protocol 1 — Read-only first

Every new White Arts capability starts as read-only/report-only.

Allowed first-stage behavior:

- inspect repository state
- inspect documented metadata
- inspect local evidence files
- summarize status
- report pass/fail/unknown
- propose a repair plan
- generate reviewable candidate notes

Forbidden first-stage behavior:

- silent writes
- host mutation
- service restart
- firewall mutation
- privilege escalation
- destructive cleanup
- credential access
- automatic promotion

## Protocol 2 — No-execute analysis boundary

White Arts may consume metadata-only analysis records, but it must not execute untrusted files or imply sandbox containment.

Required output language for analysis-linked surfaces:

```text
execution-state  : not-executed
host-execution   : disabled
sandbox-claim    : not-claimed
claim-boundary   : metadata-only
```

Dynamic analysis, malware detonation, exploit behavior, or hostile-code execution is out of scope unless a future isolated lab design is separately documented, reviewed, and gated.

## Protocol 3 — Explicit operator consent for mutation

Any future mutating White Arts command must require all of the following:

- explicit command name
- explicit target
- explicit mode
- visible diff or action preview
- rollback note
- safety boundary text
- evidence capture
- human approval before promotion

No mutation may be hidden behind status, inspect, diagnose, audit, list, or report commands.

## Protocol 4 — Secret hygiene and privacy

White Arts must never request, store, print, or commit:

- passwords
- personal access tokens
- SSH private keys
- API keys
- Apple ID credentials
- browser cookies
- recovery codes
- private keys
- private personal data

Reports must use redaction. Findings should identify secret-risk classes without echoing secret material.

## Protocol 5 — Integrity validation posture

Integrity checks must be repeatable and conservative.

Minimum integrity checks for White Arts promotion:

- required docs exist
- required tests exist
- generated docs preserve managed blocks
- release metadata agrees across public surfaces
- Base1 links and inventories are intact
- security and crypto docs preserve guardrails
- analysis commands preserve no-execute boundaries
- White Arts docs preserve defensive-only scope
- no secrets, tokens, private keys, or credentials are committed
- all new claims are backed by evidence

## Protocol 6 — Healing is staged repair, not autonomous repair

White Arts healing means safe diagnosis and reviewable repair planning first.

Healing may:

- detect stale metadata
- detect broken links
- detect missing documentation
- detect failed tests
- detect mismatched release notes
- recommend repair steps
- create candidate patch plans

Healing must not:

- silently repair production systems
- bypass trust gates
- mutate host state without approval
- delete recovery evidence
- hide failed checks
- mark unsafe systems as nominal

## Protocol 7 — Nominal means checked, not invulnerable

Nominal state means the documented checks passed for the documented scope.

Nominal does not mean:

- hardened
- secure against all attackers
- ransomware-proof
- malware-safe
- compliant
- certified
- daily-driver ready
- production-ready

Every nominal report must include scope, timestamp, evidence source, unknowns, and exceptions.

## Protocol 8 — Security audit movement

White Arts security audits must track findings without creating offensive capability.

Allowed audit scopes:

- command capability metadata
- host-trust gates
- safe-mode behavior
- redaction paths
- dependency and license posture
- release metadata consistency
- Base1 recovery evidence
- Fyr package/runtime safety
- open security server planning
- incident response readiness
- maintenance posture

Disallowed audit scopes:

- exploit deployment
- unauthorized scanning
- credential harvesting
- persistence, evasion, or stealth
- bypassing access controls
- real-target attack simulation without authorization

## Protocol 9 — Open security server suite guardrail

The future White Arts open security server suite must remain defensive, reviewable, and replaceable.

Required properties:

- open-source-first component preference
- source and license review before integration
- local/private evidence handling
- read-only inventory before any agent behavior
- no secret collection
- no unauthorized network scanning
- report-only default mode
- clear operator controls
- per-component non-claim language

Any server-facing integration must document its trust boundary, data flow, retention policy, and failure behavior.

## Protocol 10 — Promotion and release gate

A White Arts feature may only move toward release when it satisfies the promotion ladder:

```text
planned -> documented -> read-only checked -> locally validated -> CI validated -> reviewed -> release eligible
```

Promotion blockers:

- missing tests
- missing docs
- missing failure behavior
- unsafe mutation
- unreviewed dependency
- overclaiming security language
- missing recovery notes
- missing operator consent path
- unclear data handling

## Standing guardrails

White Arts must preserve these boundary statements unless implementation evidence proves a narrower, reviewed claim:

```text
mode             : defensive
mutation         : review-required
execution-state  : not-executed unless explicitly proven safe for that command
host-execution   : disabled by default
sandbox-claim    : not-claimed
repair-policy    : staged-candidate-only until reviewed
claim-boundary   : evidence-bound-maintenance
```

## Future reference rule

Future White Arts PRs should link back to this document when changing:

- maintenance posture
- healing behavior
- security audit behavior
- server-suite planning
- Base1 recovery alignment
- Fyr automation
- analysis metadata handling
- release promotion criteria
- claim language

If a future change cannot satisfy these protocols, it must explicitly mark the gap and remain planned, experimental, or blocked.
