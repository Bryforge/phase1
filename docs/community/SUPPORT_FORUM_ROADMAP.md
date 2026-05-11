# Bryforge support forum roadmap

Status: planning roadmap
Scope: public support forum for Phase1, Base1, and Fyr

## Purpose

Bryforge should provide a public support forum where users, operators, contributors, and reviewers can ask questions, report problems, share workflows, and get guided support for the Bryforge project family:

- Phase1
- Base1
- Fyr

The forum should improve support without weakening the safety, security, documentation, and validation standards already used in the repositories.

## Forum goal

The support forum should be useful, searchable, safe, and welcoming.

It should help users find answers quickly while keeping maintainers organized and preserving clear boundaries between:

- user help;
- bug reports;
- feature requests;
- security reports;
- roadmap discussion;
- hardware/recovery support;
- language/toolchain discussion.

## Security and privacy goal

The forum should be as secure as possible while maintaining practical usability.

Support workflows must warn users not to post secrets, credentials, private keys, recovery codes, tokens, email passwords, Apple ID credentials, private logs, unrevised screenshots, or sensitive local files.

Security vulnerabilities should not be handled as normal public support threads. The forum should route security-sensitive issues to the project security policy instead.

## Project areas

| Area | Forum purpose |
| --- | --- |
| Phase1 | Terminal console, VFS, commands, safe mode, host tools, learning, storage, updates, UI, docs. |
| Base1 | OS-track planning, recovery, image provenance, validation reports, hardware targets, dry-run workflows. |
| Fyr | Native language usage, syntax, package plans, examples, toolchain roadmap, contributor questions. |
| Bryforge | Project coordination, announcements, releases, roadmap planning, community support. |

## Proposed forum categories

| Category | Purpose |
| --- | --- |
| Announcements | Release notes, milestone updates, public status posts. |
| Getting Started | Installation, quick start, first commands, expected boundaries. |
| Phase1 Support | Phase1 usage, commands, UI, safe mode, host tool questions. |
| Base1 Support | Base1 docs, recovery planning, dry-run validation, hardware planning. |
| Fyr Language | Fyr syntax, examples, package/toolchain roadmap. |
| Troubleshooting | Build errors, launch problems, common environment issues. |
| Feature Requests | Proposed enhancements, operator workflows, usability ideas. |
| Bugs | Reproducible defects with version, platform, logs, and expected behavior. |
| Security and Trust | Public security guidance, not private vulnerability disclosure. |
| Show and Tell | User workflows, screenshots after review, demos, and learning notes. |
| Documentation | Gaps, corrections, tutorial requests, and manual improvements. |

## Required pinned posts

The forum should launch with pinned posts for:

1. Community rules and code of conduct.
2. How to ask for help.
3. What not to post: secrets and private data.
4. Phase1 quick start.
5. Base1 status and non-claims.
6. Fyr status and non-claims.
7. Security vulnerability reporting path.
8. How to write a good bug report.
9. Roadmap and feature-request process.
10. Forum category guide.

## Bug report template

A bug report should ask for:

- project area: Phase1, Base1, Fyr, or docs;
- version or commit;
- operating system;
- command run;
- expected result;
- actual result;
- minimal reproduction steps;
- redacted logs or screenshots;
- whether safe mode or host trust was enabled;
- whether the issue affects security, data loss, or recovery.

## Support request template

A support request should ask for:

- what the user is trying to do;
- which project area is involved;
- what command or page they are using;
- what happened;
- what they expected;
- what they already tried;
- whether they are on stable, edge, or a checkpoint branch;
- any redacted screenshots or logs.

## Security routing

The forum should not be the primary place to disclose private vulnerabilities.

Security-sensitive posts should be redirected when they include or imply:

- credentials or tokens;
- private keys;
- account compromise;
- exploitable vulnerability details;
- bypass of safety gates;
- private logs or unrevised screenshots;
- real user data exposure;
- destructive Base1 or host mutation behavior.

The forum should point users to `SECURITY.md` for responsible reporting.

## Moderation plan

Moderation should prioritize:

- removing secrets or private data quickly;
- moving posts to the right category;
- merging duplicate issues;
- labeling project area and status;
- keeping roadmap claims conservative;
- preventing harassment and spam;
- preserving a welcoming tone for beginners and advanced operators.

## Labels and tags

Suggested tags:

```text
phase1
base1
fyr
docs
bug
support
feature-request
security-guidance
safe-mode
host-tools
build
install
roadmap
hardware
recovery
crypto-policy
```

## Integration with GitHub

The forum should complement GitHub issues, not replace them.

Suggested workflow:

- forum discussion for questions and triage;
- GitHub issue for confirmed bugs and actionable feature requests;
- GitHub pull request for implementation;
- documentation updates when a question repeats;
- roadmap updates when a topic becomes a planned workstream.

## Search and knowledge base plan

Repeated forum answers should become:

- FAQ entries;
- docs pages;
- troubleshooting guides;
- quick-start improvements;
- examples;
- tests when behavior is claimed.

## Launch phases

### Phase 1: planning

- Choose forum platform.
- Define categories.
- Draft pinned posts.
- Draft support, bug, and feature templates.
- Define moderation policy.
- Link forum from README and docs when ready.

### Phase 2: private or limited preview

- Invite early users and contributors.
- Test category structure.
- Test moderation workflow.
- Collect recurring questions.
- Convert repeated answers into docs.

### Phase 3: public launch

- Publish forum link.
- Pin safety and support posts.
- Announce Phase1, Base1, and Fyr categories.
- Route bugs and confirmed work into GitHub issues.
- Keep roadmap claims aligned with repository docs.

### Phase 4: knowledge base integration

- Convert common threads into documentation.
- Add forum FAQ links to project docs.
- Track unresolved support themes.
- Add tests or quality checks for repeated confusion.

## Success criteria

The forum is successful when:

- new users can find quick-start help;
- repeated questions turn into documentation improvements;
- bugs are triaged into actionable GitHub issues;
- security-sensitive reports are routed privately;
- Phase1, Base1, and Fyr support stay organized;
- support remains welcoming, safe, and searchable.

## Non-claims

This roadmap does not create or launch a forum by itself.

It does not replace `SECURITY.md`, GitHub issues, release notes, validation reports, or official repository documentation.

It defines a support forum plan for the Bryforge project family.
