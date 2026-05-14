# Fyr staged candidate track

Status: experimental design track  
Codename: `black_arts`  
Scope: staged Phase1/Base1 candidate workflows driven by Fyr plans  
Non-claim: this is not a production updater, not a release candidate process, and not a hardened recovery system.

This track defines how Fyr can eventually help Phase1 create candidate copies of Phase1/Base1, apply controlled configuration or feature plans to those candidates, validate them, and promote them only after evidence gates pass.

## Goal

The staged candidate track should eventually let Phase1 use Fyr to:

1. create a candidate from known-good inputs;
2. apply versioned configuration and feature changes to that candidate;
3. run deterministic checks against the candidate;
4. record evidence and non-claims;
5. promote the candidate only after validation succeeds.

## Proposed command surface

These command names are placeholders until implementation lands:

```text
fyr staged status
fyr staged plan
fyr staged create <candidate>
fyr staged apply <candidate> <plan.fyr>
fyr staged validate <candidate>
fyr staged promote <candidate>
fyr staged discard <candidate>
```

The public skin can later expose these through the `black_arts` codename if the guarded behavior is implemented and tested first.

## Candidate lifecycle

| Stage | Plain meaning | Required evidence |
| --- | --- | --- |
| plan | Load a versioned plan. | Plan schema, target, scope, non-claims. |
| create | Create a staged candidate tree. | Source reference, candidate ID, clean workspace. |
| apply | Apply scoped changes to the candidate. | Change log, file list, rejected operations. |
| validate | Validate the candidate. | Test output, doc checks, status checks. |
| promote | Promote only after validation. | Operator approval, rollback path, evidence link. |
| discard | Remove failed or stale candidate. | Deletion log, live-system untouched marker. |

## Fixtures

The current staged candidate fixtures are:

```text
docs/fyr/fixtures/staged-candidate-ok.txt
docs/fyr/fixtures/staged-plan-ok.txt
docs/fyr/fixtures/staged-validation-ok.txt
docs/fyr/fixtures/staged-approval-ok.txt
docs/fyr/fixtures/staged-discard-ok.txt
docs/fyr/fixtures/staged-claim-boundary-ok.txt
```

These fixtures define expected shapes for candidate metadata, plan metadata, validation results, explicit approval, discard records, and claim boundaries. They are not implementations.

## Safety rules

Required defaults:

- candidate-only writes;
- declared workspace first;
- live system remains untouched until explicit promotion;
- no hidden host command execution;
- no network by default;
- no promotion without validation;
- deterministic evidence output;
- explicit rollback or discard path;
- visible non-claims in every report.

## Promotion rule

A candidate can be promoted only if all are true:

1. the candidate was created from a known source;
2. all changes are recorded;
3. validation passes;
4. the public status and non-claim boundary remain accurate;
5. rollback or discard instructions exist;
6. an operator explicitly approves promotion.

## First evidence gate

Before implementation, the repository needs fixtures and tests for:

- plan shape;
- candidate metadata shape;
- validation result shape;
- promotion approval shape;
- discard result shape;
- non-claim preservation.

## Current wording

Use this wording now:

> `black_arts` is the codename for an experimental Fyr staged-candidate design track. It does not change the live system and does not promote candidates without validation evidence and explicit operator approval.
