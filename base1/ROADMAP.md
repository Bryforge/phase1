# Base1 Roadmap

Base1 is the secure host layer for Phase1. This roadmap keeps the work staged so the project does not jump directly into risky installer behavior.

## Phase B0 - Foundation

Status: started

Goals:

- Define Base1 mission and trust boundary.
- Define Raspberry Pi and X200 target classes.
- Define Base1 and Phase1 compatibility contract.
- Add secure-default profile metadata.
- Add non-destructive preflight checker.
- Add hardened Phase1 launcher wrapper.
- Add systemd service sandbox template.
- Add CI-visible tests for the foundation assumptions.

Exit criteria:

- Documentation is present and linked.
- Preflight script is read-only.
- Launcher refuses root by default.
- Service template uses hardening directives.
- Tests validate the foundation files.

## Phase B1 - Reproducible host profile

Goals:

- Choose first reproducible image path.
- Generate package manifest.
- Generate users, directories, and permissions plan.
- Generate firewall profile plan.
- Add dry-run output for every planned host change.
- Add checksums for generated artifacts.

Exit criteria:

- Operator can generate a Base1 plan without changing the host.
- Generated plan is reviewable in plain text.
- No secrets are stored in generated files.

## Phase B2 - Raspberry Pi profile

Goals:

- Add Raspberry Pi profile defaults.
- Add serial-console friendly output.
- Add low-memory profile.
- Add removable-media recovery notes.
- Add offline mode.
- Add service-mode launch path.

Exit criteria:

- Raspberry Pi profile can run Phase1 under the Base1 contract.
- Recovery process is documented.
- Network profile starts with deny-inbound behavior.

## Phase B3 - X200 profile

Goals:

- Add X200 profile defaults.
- Add legacy terminal compatibility profile.
- Add offline-first operator notes.
- Add storage-encryption planning docs.
- Add minimal desktop or terminal-only deployment notes.

Exit criteria:

- X200 profile can run Phase1 under the Base1 contract.
- Offline docs are available.
- Compatibility notes cover legacy display and terminal constraints.

## Phase B4 - Phase1 release installation flow

Goals:

- Install Phase1 from a known-good release artifact.
- Verify artifact checksums.
- Keep Phase1 assets read-only from the Phase1 runtime account.
- Keep workspace separate and resettable.
- Add rollback to previous Phase1 release.

Exit criteria:

- Damaged Phase1 workspace can be reset without damaging Base1.
- Known-good Phase1 assets remain protected.
- Update and rollback actions are logged outside the Phase1 workspace.

## Phase B5 - Hardened runtime profiles

Goals:

- Add service mode.
- Add interactive operator mode.
- Add offline-only mode.
- Add maintenance mode.
- Add restricted-network mode.
- Add development mode with clear warnings.

Exit criteria:

- Secure-default remains the default.
- Maintenance and development modes are explicit and logged.
- Phase1 cannot silently persist elevated host permissions.

## Phase B6 - Recovery and resilience

Goals:

- Add recovery media plan.
- Add workspace reset workflow.
- Add host log preservation workflow.
- Add Base1 profile backup and restore.
- Add first rollback or snapshot profile.

Exit criteria:

- A destroyed Phase1 workspace can be recovered.
- Base1 remains bootable and auditable.
- Recovery steps are simple enough for field use.

## Phase B7 - Audit maturity

Goals:

- Add threat-model review checklist.
- Add shell script linting.
- Add service hardening review.
- Add secret scanning to Base1 files.
- Add hardware target test reports.
- Add release checklist.

Exit criteria:

- Every release has a documented Base1 security review.
- Known limitations are visible.
- Security claims remain accurate and conservative.

## Long-term direction

Base1 should build toward operating-system-level security discipline:

- Minimal trusted computing base.
- Privilege separation.
- Reproducible builds.
- Conservative defaults.
- Documented recovery.
- Operator-readable configuration.
- Clear compatibility with Phase1.
- Honest security claims backed by tests and audits.
