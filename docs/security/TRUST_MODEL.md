# Phase1 Trust Model

> **Status:** Documentation guardrail and safety model.
>
> **Validation:** This page defines documentation requirements. Implementation claims must link to tests, scripts, release notes, or validation reports.
>
> **Non-claims:** Phase1 is not currently a finished secure OS replacement, Base1 is not currently a released bootable daily-driver image, and guarded host tools do not make an untrusted host trustworthy.

This page defines the trust and safety language for **The Phase1 Codex**.

## Core principle

Phase1 should make capability, trust, and risk visible to the operator. It should not hide host mutation, imply guarantees that are not implemented, or market roadmap items as current security properties.

## Current trust boundary

Phase1 currently runs on a host operating system. The host controls process execution, the real filesystem, devices, network, credentials, and kernel-level enforcement. Phase1 can model, organize, gate, log, and explain actions inside its own environment, but it does not currently replace the host kernel or turn the host into a trusted computing base.

## Safe defaults

Safe defaults mean Phase1 starts with higher-risk host-backed behavior disabled or gated.

Safe-default documentation must state:

- what is disabled by default;
- how the operator can inspect the current state;
- what action changes the state;
- what command or log records the change;
- what the default does not protect against.

Avoid saying `safe` as a broad guarantee. Prefer `disabled by default`, `requires confirmation`, `read-only by default`, or `logged for operator review`.

## Guarded host tools

A guarded host tool is any Phase1 command or helper that reaches outside the Phase1 model into host-backed behavior. Examples may include host shell execution, Git, Cargo, filesystem mutation outside the VFS model, networking, image writing, or device inspection.

Guarded host tools must document:

- required host capability;
- whether safe shield blocks it by default;
- whether a trust gate is required;
- whether explicit operator confirmation is required;
- what is logged;
- what tests or dry-run commands validate behavior.

## No host trust escalation

A trust gate is a Phase1 decision point, not a guarantee about the host. Documentation must not imply that enabling a trust gate makes arbitrary host commands safe.

Allowed wording:

> This command requires the operator to enable host trust before Phase1 will invoke the host-backed workflow.

Disallowed wording:

> Turning on host trust makes host execution secure.

## Explicit operator confirmation

Any workflow that mutates host state, writes images, touches block devices, changes boot material, handles credentials, or removes files must require a visible confirmation path before the real operation.

Docs should show dry-run examples before real mutation examples.

## Read-only validation bundles

Read-only validation is the preferred first implementation mode for Base1 and recovery features. A validation bundle should be able to inspect docs, scripts, metadata, checksums, and target identity without modifying the target.

If a validation script writes files, the docs must state:

- exact files or directories written;
- whether the write is inside the repo, a temp directory, or a target device;
- cleanup instructions;
- how the operator can verify no target mutation occurred.

## Recovery shell preservation

Base1 recovery planning must preserve a shell path even when Phase1 fails. This is a requirement for future bootable environments, not proof that current hardware recovery is complete.

Recovery docs must distinguish:

- design requirement;
- dry-run command;
- emulator validation;
- real hardware validation;
- release-ready recovery procedure.

## Image provenance and checksum rules

A Base1 or recovery image must not be presented as usable unless docs identify:

- source repository and commit;
- build recipe;
- artifact name;
- checksum;
- signature or attestation status, if available;
- release identifier;
- validation date;
- known limitations.

Without those details, call the artifact a design, dry-run, or preview artifact.

## Base1 claim levels

| Claim level | Allowed wording | Required evidence |
| --- | --- | --- |
| Design | planned, proposed, expected, intended | design doc |
| Dry-run | dry-run checker, read-only validation | script and test output |
| Emulator | boots or validates in emulator | reproducible emulator steps |
| Hardware lab | validated on named hardware | dated hardware report |
| Preview | early image for testers | checksums, release notes, rollback notes |
| Stable | supported release | repeatable builds, tests, recovery evidence, known limitations |

## Security review checklist

A security reviewer should verify:

- The page names the host boundary.
- Claims are tied to commands, tests, scripts, or release evidence.
- Destructive workflows are not presented as default paths.
- Dry-run commands appear before mutation commands.
- Host trust is described as operator consent, not host hardening.
- Recovery and rollback claims include validation status.
- Secrets are never requested in docs, examples, issue templates, or screenshots.
