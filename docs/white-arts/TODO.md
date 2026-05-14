# White Arts TODO

Status: living maintenance checklist  
Scope: defensive maintenance posture, integrity validation, security audit readiness, and future open security server suite work  

## Current priority

Keep White Arts easy to maintain, conservative in claims, and useful for future implementation.

## Always-on maintenance posture

- [ ] Keep White Arts docs defensive-only.
- [ ] Keep all healing language staged and review-required.
- [ ] Keep all security claims evidence-bound.
- [ ] Keep all first-stage commands read-only/report-only.
- [ ] Keep no-execute boundaries visible for analysis metadata.
- [ ] Keep Base1 recovery and dry-run claims conservative.
- [ ] Keep Fyr automation scoped to safe checks until reviewed.
- [ ] Keep public status metadata synchronized only through reviewed update paths.
- [ ] Keep secret redaction requirements visible in every report plan.
- [ ] Keep unknown states reported as unknown.

## Documentation TODO

- [ ] Link `PROTOCOLS_AND_GUARDRAILS.md` from the White Arts index.
- [ ] Link `OPEN_SECURITY_SERVER_SUITE.md` from the White Arts index when present.
- [ ] Add a White Arts command-surface design document.
- [ ] Add a White Arts finding taxonomy.
- [ ] Add a White Arts evidence directory policy.
- [ ] Add a White Arts report archive policy.
- [ ] Add a White Arts server inventory schema.
- [ ] Add a White Arts server nominal-state schema.
- [ ] Add a White Arts incident-response readiness checklist.
- [ ] Add a White Arts recovery readiness checklist.

## Test TODO

- [ ] Add docs tests for protocols and guardrails.
- [ ] Add docs tests for the TODO maintenance checklist.
- [ ] Add docs tests for open security server suite boundaries.
- [ ] Add tests proving White Arts command stubs are read-only.
- [ ] Add tests proving White Arts reports preserve non-claims.
- [ ] Add tests proving White Arts healing cannot silently mutate files.
- [ ] Add tests proving White Arts does not expose secrets.
- [ ] Add tests proving nominal reports include pass/fail/unknown states.
- [ ] Add tests proving server inventory remains read-only at first.
- [ ] Add tests proving promotion blockers are listed.

## Command TODO

- [ ] Add `white-arts status` as a read-only command.
- [ ] Add `white-arts help` as a read-only command.
- [ ] Add `white-arts inventory` as a read-only reporter.
- [ ] Add `white-arts check` as a read-only integrity summary.
- [ ] Add `white-arts report` as a local report preview.
- [ ] Add `white-arts audit security` as documentation-backed output.
- [ ] Add `white-arts audit integrity` as documentation-backed output.
- [ ] Add `white-arts audit base1` as documentation-backed output.
- [ ] Add `white-arts audit fyr` as documentation-backed output.
- [ ] Add `white-arts server status` as a planned/read-only stub.

## Security TODO

- [ ] Define security finding severity labels.
- [ ] Define blocked-claim labels.
- [ ] Define operator approval labels.
- [ ] Define safe repair candidate states.
- [ ] Define audit report required fields.
- [ ] Define redaction tests for White Arts reports.
- [ ] Define dependency/license review rules for open security components.
- [ ] Define data-retention rules for server evidence.
- [ ] Define network-scope authorization rules before any server scan feature.
- [ ] Define incident-response non-claims and escalation boundaries.

## Open security server suite TODO

- [ ] Define server inventory model.
- [ ] Define service inventory model.
- [ ] Define package inventory model.
- [ ] Define network policy state model.
- [ ] Define identity and access review model.
- [ ] Define backup and recovery proof model.
- [ ] Define log ingestion and redaction model.
- [ ] Define report-only dashboard plan.
- [ ] Define open-source component review checklist.
- [ ] Define server-suite promotion gates.

## Base1 alignment TODO

- [ ] Map Base1 recovery docs into White Arts report fields.
- [ ] Map Base1 dry-run scripts into White Arts integrity checks.
- [ ] Map Base1 hardware evidence into White Arts nominal-state fields.
- [ ] Map Base1 rollback metadata into White Arts recovery posture.
- [ ] Preserve no destructive write support until explicitly reviewed.

## Fyr alignment TODO

- [ ] Define safe Fyr scripts for White Arts checks.
- [ ] Define Fyr report-generation constraints.
- [ ] Define Fyr package validation hooks.
- [ ] Block privileged repair from Fyr until reviewed.
- [ ] Keep Fyr automation evidence-bound.

## Analysis metadata TODO

- [ ] Keep `analyze load` metadata-only.
- [ ] Keep `analyze inspect` metadata-only.
- [ ] Keep `analyze list` metadata-only.
- [ ] Add White Arts report ingestion for analysis records.
- [ ] Preserve `execution-state : not-executed` in every linked output.
- [ ] Preserve `sandbox-claim : not-claimed` in every linked output.

## Posture review cadence

Suggested review loop:

```text
weekly: docs links, TODO status, failed tests, stale status metadata
per PR: claim language, test coverage, failure behavior, redaction, rollback notes
per release candidate: full White Arts report, Base1 evidence review, security audit summary, promotion decision
```

## Done definition

A White Arts TODO item is not done until it has:

- documentation
- tests or an explicit reason tests are not applicable yet
- clear safe-default behavior
- non-claim language
- reviewable evidence
- owner-visible next step
