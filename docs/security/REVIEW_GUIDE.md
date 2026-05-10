# Phase1 Codex Security Review Guide

> **Status:** Documentation review guide.
>
> **Validation:** Use during documentation and safety-sensitive PR review with `DOCS_CLAIMS.md`, `TRUST_MODEL.md`, and repository tests.
>
> **Non-claims:** This guide does not certify Phase1 as a secure OS replacement, Base1 as a released bootable system, or any recovery path as hardware-complete.

This guide defines how reviewers should evaluate safety-sensitive Codex changes.

## Review goals

A reviewer should confirm that documentation makes risk, capability, trust, and validation visible to the operator. The goal is not to make every page longer. The goal is to make every claim narrow, testable, and tied to evidence.

## First-pass review

Check every safety-sensitive page for:

- a status block;
- a clear current/roadmap split;
- a validation path;
- a non-claims section;
- no broad security wording without evidence;
- no destructive workflow presented as the default path;
- no claim that Phase1 upgrades host trust;
- no claim that Base1 is bootable, installer-ready, or recovery-complete unless evidence exists.

## Host boundary review

For host-backed workflows, verify that the page states:

- which command or workflow crosses into host-backed behavior;
- what capability is required;
- whether safe shield or trust gates apply;
- whether explicit confirmation is required;
- what is logged;
- what the workflow does not protect against.

## Base1 and recovery review

For Base1, recovery, rollback, image, hardware, or installer pages, verify that claims are labeled as one of:

- design;
- dry-run;
- emulator validation;
- hardware validation;
- preview release;
- stable release.

A page may only use stronger language when it links to artifacts, checksums, build notes, validation reports, or named hardware results.

## Fyr review

For Fyr pages, verify that examples and language features match current tests or are clearly labeled roadmap.

Do not allow production-language claims unless the repository includes explicit release, compatibility, and stability evidence.

## Secret and privacy review

Reject or request changes if examples include:

- real tokens;
- real passwords;
- real email inbox content;
- private host usernames when unnecessary;
- API keys;
- secret-looking placeholders that may encourage unsafe copying.

Use placeholders such as `<token>`, `<device>`, `<path>`, or `<target>` only when the surrounding text explains that the operator must provide their own value.

## Approval checklist

Before approving a safety-sensitive documentation PR, answer:

- Does the page describe what exists now?
- Does the page describe what is only planned?
- Does the page avoid overclaiming?
- Does the page show dry-run or read-only paths first?
- Does the page explain host-backed behavior clearly?
- Does the page preserve recovery-shell and rollback caution?
- Does the page name evidence for strong claims?
- Do relevant docs tests pass?

If any answer is no, request changes.