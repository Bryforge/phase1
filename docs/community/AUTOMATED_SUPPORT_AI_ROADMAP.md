# Bryforge automated support AI roadmap

Status: planning roadmap
Scope: automated AI-assisted support for Phase1, Base1, Fyr, and Bryforge community workflows

## Purpose

Bryforge should plan an automated support AI that helps users get faster technical support for Phase1, Base1, and Fyr while keeping maintainers organized and preserving safety, privacy, and accuracy.

The support AI should help with common questions, triage, documentation routing, troubleshooting, and issue preparation. It should not replace maintainers, security reporting, release validation, or official documentation.

## Goal

The support AI should make technical support easier, safer, and more searchable.

It should help users:

- find the right docs quickly;
- understand the current status boundary of Phase1, Base1, and Fyr;
- run safe diagnostic commands;
- prepare useful bug reports and support requests;
- avoid posting secrets or private information;
- route security-sensitive reports to the correct private path;
- convert repeated support questions into documentation improvements.

## Security and privacy goal

The support AI should be as secure as possible while maintaining practical usability.

It must warn users not to share:

- passwords;
- tokens;
- private keys;
- recovery codes;
- Apple ID or email credentials;
- private logs;
- unrevised screenshots;
- account secrets;
- sensitive local files;
- production data.

The support AI should default to redaction-first guidance and should never ask a user to paste secrets.

## Supported project areas

| Area | Support AI role |
| --- | --- |
| Phase1 | Quick start, safe mode, commands, VFS, UI, runtime boundaries, troubleshooting. |
| Base1 | Read-only recovery planning, hardware docs, image provenance docs, dry-run validation. |
| Fyr | Syntax help, examples, package planning, language/toolchain docs. |
| Security | Public safety guidance, trust boundaries, redaction, non-claims, private vulnerability routing. |
| Crypto policy | Documentation routing only until implementation, tests, review, and validation exist. |
| Community | Forum triage, duplicate detection, support templates, docs suggestions. |

## Non-goals

The support AI must not:

- replace `SECURITY.md` for vulnerability reporting;
- claim Phase1 is a secure OS replacement;
- claim Base1 is a released bootable daily-driver image;
- claim Fyr is production-ready;
- claim cryptographic completeness, audit completion, certification, quantum safety, or hardware validation without evidence;
- run destructive host commands automatically;
- request secrets or private credentials;
- provide unsupported legal, medical, or financial advice;
- silently create issues, PRs, or public posts without operator review.

## Core capabilities

### Phase 1: documentation assistant

- Search and cite repository documentation.
- Explain Phase1, Base1, and Fyr status boundaries.
- Point users to quick-start guides.
- Explain safe-mode and trust-gate behavior.
- Recommend read-only diagnostics first.
- Suggest the correct issue template when a problem needs maintainer review.

### Phase 2: support triage assistant

- Ask structured troubleshooting questions.
- Classify support requests by project area.
- Identify missing version, environment, command, and reproduction details.
- Detect likely duplicates.
- Draft issue text for user review.
- Suggest docs that should be updated if the same question repeats.

### Phase 3: redaction assistant

- Remind users not to paste secrets.
- Help users redact logs before posting.
- Flag likely tokens, private keys, credentials, recovery codes, cookies, and private paths.
- Recommend private security reporting when sensitive data appears.

### Phase 4: maintainer assistant

- Summarize support threads.
- Identify recurring issues.
- Propose documentation updates.
- Suggest labels and project areas.
- Draft maintainer responses for review.
- Convert confirmed bugs into GitHub issue drafts.

### Phase 5: forum integration

- Integrate with the planned support forum from [`SUPPORT_FORUM_ROADMAP.md`](SUPPORT_FORUM_ROADMAP.md).
- Provide category suggestions.
- Suggest tags.
- Route security-sensitive topics away from public threads.
- Create FAQ candidates from repeated answers.

## Support workflow

A safe support AI workflow should be:

1. Identify project area: Phase1, Base1, Fyr, docs, security, crypto, website, or community.
2. Ask for non-sensitive context: version, branch, OS, command, expected behavior, actual behavior.
3. Recommend read-only checks first.
4. Search relevant docs.
5. Provide a concise answer with links to docs.
6. If unresolved, draft a bug/support issue for user review.
7. Never post publicly without user approval.
8. Route security-sensitive reports to `SECURITY.md`.

## Read-only diagnostic preference

The support AI should prefer commands such as:

```bash
sh phase1 version
sh phase1 doctor
sh phase1 selftest
sh scripts/quality-check.sh quick
sh scripts/quality-check.sh base1-docs
sh scripts/quality-check.sh security-crypto-docs
```

Base1 and recovery guidance should prefer dry-run or read-only commands first.

## Suggested knowledge sources

The support AI should ground answers in repository docs, including:

- [`../../README.md`](../../README.md)
- [`../../CONTRIBUTING.md`](../../CONTRIBUTING.md)
- [`../../SECURITY.md`](../../SECURITY.md)
- [`../../docs/quality/QUALITY.md`](../../docs/quality/QUALITY.md)
- [`../README.md`](../README.md)
- [`../security/README.md`](../security/README.md)
- [`../community/README.md`](README.md)
- [`SUPPORT_FORUM_ROADMAP.md`](SUPPORT_FORUM_ROADMAP.md)
- [`../fyr/README.md`](../fyr/README.md)
- [`../base1/README.md`](../base1/README.md)

## Escalation rules

Escalate to maintainer review when:

- a bug is reproducible;
- a command may mutate host state;
- Base1 recovery, image, boot, rollback, or hardware guidance is involved;
- crypto policy or provider behavior is involved;
- logs mention credentials, keys, tokens, or private data;
- a user reports possible security bypass;
- the support AI is uncertain.

## Public issue routing

The support AI should recommend the correct GitHub template:

- bug report for reproducible defects;
- feature request for proposed enhancements;
- support request for troubleshooting;
- documentation issue for docs gaps;
- crypto/security-policy proposal for cryptographic policy planning.

## Privacy and logging policy

The support AI should minimize stored data.

Planned behavior:

- do not store secrets;
- redact sensitive strings before summaries;
- keep user-facing logs minimal;
- clearly mark generated issue drafts as drafts;
- allow users to review before anything is posted publicly;
- maintain a deletion/removal path for accidental sensitive content when supported by the platform.

## Evaluation plan

The support AI should be evaluated against:

- answer accuracy against repository docs;
- correct refusal to handle private vulnerability details publicly;
- secret-redaction behavior;
- correct issue-template routing;
- quality of reproduction-step collection;
- reduction in repeated questions;
- maintainer review usefulness;
- user clarity and usability.

## Launch phases

### Phase 1: roadmap and safety model

- Create this roadmap.
- Link it from community docs.
- Define privacy and escalation rules.
- Define supported docs and issue templates.

### Phase 2: prompt and knowledge-base prototype

- Create a controlled support prompt.
- Ground answers in repository docs.
- Add redaction instructions.
- Test against common support questions.

### Phase 3: issue-draft assistant

- Draft bug/support/feature reports for user review.
- Require explicit user approval before public posting.
- Add labels and project areas as suggestions only.

### Phase 4: forum assistant

- Integrate with the planned public support forum.
- Suggest categories and tags.
- Detect repeated topics.
- Propose FAQ/docs updates.

### Phase 5: maintainer review and metrics

- Review AI-assisted support quality.
- Track repeated issues and docs gaps.
- Add tests or quality checks for recurring confusion.
- Publish limitations and safe-use guidance.

## Success criteria

The support AI is successful when:

- users reach the correct docs faster;
- support posts contain enough detail to act on;
- fewer users accidentally post sensitive data;
- maintainers spend less time asking for basic reproduction details;
- repeated questions become documentation improvements;
- security-sensitive topics are routed privately;
- the AI preserves project non-claims.

## Non-claims

This roadmap does not create or deploy an automated support AI by itself.

It does not replace maintainers, GitHub issues, `SECURITY.md`, release notes, validation reports, or official documentation.

It defines a safe implementation plan for future AI-assisted technical support across Phase1, Base1, Fyr, and Bryforge community workflows.
