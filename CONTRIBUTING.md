# Contributing to Phase1

Status: active contribution guidelines
Scope: Phase1, Base1, Fyr, documentation, security, quality, crypto policy, website, and community support work

Thank you for wanting to contribute to Phase1 and the Bryforge project family.

Phase1 values useful engineering over hype. Good contributions improve clarity, safety, usability, documentation, validation, compatibility, or the long-term Phase1/Base1/Fyr roadmap.

## Project areas

| Area | Good contribution examples |
| --- | --- |
| Phase1 | Shell commands, VFS behavior, safe-mode clarity, UI, docs, tests, learning, storage, runtime support. |
| Base1 | Read-only validation, recovery planning, hardware docs, image provenance, dry-run scripts, compatibility maps. |
| Fyr | Language docs, examples, parser/runtime tests, package planning, operator-facing scripts. |
| Security | Trust boundaries, redaction, safe defaults, crypto policy docs, threat-model clarity, quality checks. |
| Website/community | Public docs, forum planning, support workflows, accessibility, mobile fit, branding consistency. |

## Ground rules

- Keep safety and usability together.
- Do not overclaim security, OS readiness, hardware validation, cryptographic completeness, or audit status.
- Hardened security is allowed and encouraged as a roadmap goal when it is clearly labeled as planned, design, preview, or validation-dependent.
- Only describe something as currently hardened when implementation, tests, validation reports, recovery evidence, and review evidence support that claim.
- Prefer tests over claims.
- Preserve compatibility paths unless a future change explicitly approves removal.
- Keep destructive behavior out of default paths.
- Use read-only and dry-run workflows before mutation.
- Never commit secrets, tokens, private keys, credentials, recovery codes, private logs, or unrevised screenshots.
- Keep contributions reviewable: small, focused changes are better than large mixed changes.

## Before you start

Read the relevant docs first:

- [`README.md`](README.md)
- [`QUALITY.md`](QUALITY.md)
- [`SECURITY.md`](SECURITY.md)
- [`SECURITY_REVIEW.md`](SECURITY_REVIEW.md)
- [`docs/README.md`](docs/README.md)
- [`docs/security/README.md`](docs/security/README.md)
- [`docs/base1/README.md`](docs/base1/README.md)
- [`docs/fyr/README.md`](docs/fyr/README.md)

For crypto policy work, also read:

- [`docs/security/CRYPTO_POLICY_ROADMAP.md`](docs/security/CRYPTO_POLICY_ROADMAP.md)
- [`docs/security/CRYPTO_IMPLEMENTATION_PLAN.md`](docs/security/CRYPTO_IMPLEMENTATION_PLAN.md)
- [`docs/security/CRYPTO_REGISTRY.md`](docs/security/CRYPTO_REGISTRY.md)
- [`docs/security/CRYPTO_PROVIDER_REGISTRY.md`](docs/security/CRYPTO_PROVIDER_REGISTRY.md)
- [`docs/security/CRYPTO_OPERATOR_COMMANDS.md`](docs/security/CRYPTO_OPERATOR_COMMANDS.md)
- [`docs/security/CRYPTO_CONFIG_SCHEMA.md`](docs/security/CRYPTO_CONFIG_SCHEMA.md)

For Base1 x86_64 and boot-support work, also read:

- [`docs/os/X86_64_BOOT_SUPPORT_ROADMAP.md`](docs/os/X86_64_BOOT_SUPPORT_ROADMAP.md)
- [`docs/os/ROADMAP.md`](docs/os/ROADMAP.md)
- [`base1/HARDWARE_TARGETS.md`](base1/HARDWARE_TARGETS.md)
- [`base1/LIBREBOOT_PROFILE.md`](base1/LIBREBOOT_PROFILE.md)

## Branch and PR model

Use this model unless maintainers say otherwise:

- branch from `edge/stable`;
- use `feature/<short-name>` for normal work;
- use `docs/<short-name>` for documentation-only work;
- use `fix/<short-name>` for bug fixes;
- open PRs back into `edge/stable`;
- keep stable base and checkpoint branches boring and preservation-focused.

## Contribution workflow

1. Sync your checkout.
2. Create a focused branch.
3. Make the smallest useful change.
4. Add or update tests/docs.
5. Run the relevant quality gates.
6. Open a PR with clear validation notes.

Example:

```bash
git fetch origin
git checkout edge/stable
git pull --ff-only origin edge/stable
git checkout -b feature/my-change
```

## Required validation

Before opening release-facing work, run:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
sh scripts/quality-check.sh quick
```

For documentation-only work, run the most relevant focused gate:

```bash
sh scripts/quality-check.sh base1-docs
sh scripts/quality-check.sh security-crypto-docs
```

For broader Base1 organization readiness work, run:

```bash
sh scripts/quality-check.sh base1-reorg
```

Before release work, run:

```bash
sh scripts/quality-check.sh full
```

## PR checklist

Every PR should explain:

- what changed;
- why it changed;
- which project area is affected;
- which files were touched;
- what validation was run;
- what risks remain;
- whether docs were updated;
- whether tests were updated;
- whether safe defaults are preserved;
- whether any compatibility path is affected.

## Documentation rules

New docs should include a status boundary when the page makes claims about implementation, safety, hardware, security, crypto, or OS readiness.

Use this pattern:

```md
> **Status:** Implemented | Experimental | Design | Dry-run | Preview | Roadmap | Not claimed
> **Validation:** tests, scripts, release notes, or manual verification path
> **Non-claims:** what this page does not guarantee
```

Use narrow, testable wording. Prefer:

- `blocked by default`;
- `requires explicit confirmation`;
- `read-only validation`;
- `dry-run`;
- `planned`;
- `design`;
- `not claimed`.

Avoid broad current-state claims such as:

- secure;
- hardened;
- production-ready;
- bootable daily-driver;
- installer-ready;
- cryptographically complete;
- audited;
- certified;
- quantum-safe;
- hardware-validated.

These words may be used for goals, plans, roadmaps, designs, or future target states when the status is explicit. Only use them as current claims when the repository contains implementation, tests, release notes, validation reports, and review evidence.

## Security contribution rules

Security-sensitive work must preserve:

- safe mode as the normal posture;
- explicit host trust gates;
- no silent host mutation;
- secret redaction in logs and user-facing output;
- no credential requests in docs or examples;
- conservative claims until evidence exists.

Hardening work is welcome when it is designed, staged, testable, and honest about current status.

Do not report private vulnerabilities in a public PR or issue. Follow [`SECURITY.md`](SECURITY.md).

## Base1 x86_64 and boot support contribution rules

x86_64 and boot-support work must be automatic only after read-only detection, boot-parameter reporting, recovery planning, and validation exist.

Rules:

- detect architecture, firmware mode, boot loader, storage layout, and recovery availability before mutation;
- document required boot parameters before using them;
- keep unknown boot states fail-closed;
- preserve emergency shell and recovery paths;
- prefer VM validation before physical hardware claims;
- require hardware validation before support claims;
- do not silently modify boot loaders, partitions, or kernel command lines;
- do not claim hardened boot, secure boot, measured boot, TPM support, or lockdown support without evidence.

## Crypto policy contribution rules

Crypto work is documentation-first until implementation, tests, review, and validation exist.

Rules:

- do not invent custom security-critical primitives;
- use reviewed open-source implementations where possible;
- document algorithms with [`docs/security/CRYPTO_ALGORITHM_TEMPLATE.md`](docs/security/CRYPTO_ALGORITHM_TEMPLATE.md);
- document providers with [`docs/security/CRYPTO_PROVIDER_TEMPLATE.md`](docs/security/CRYPTO_PROVIDER_TEMPLATE.md);
- keep unknown profiles, scopes, algorithms, and providers fail-closed;
- keep `lab-only` isolated from real protection decisions;
- do not connect providers or algorithms to runtime behavior until the implementation plan allows it;
- preserve non-claims.

Focused gate:

```bash
sh scripts/quality-check.sh security-crypto-docs
```

## Base1 contribution rules

Base1 is preservation-first and non-destructive by default.

Rules:

- keep root compatibility paths recoverable;
- keep organized mirrors linked;
- run `base1-docs` before and after organization work;
- do not remove checkpoint/release files without explicit future approval;
- do not claim installer readiness, boot readiness, hardware validation, hardened status, or daily-driver readiness without evidence;
- keep scripts read-only or dry-run unless a future validated path explicitly allows mutation.

Focused gates:

```bash
sh scripts/quality-check.sh base1-docs
sh scripts/quality-check.sh base1-reorg
```

## Fyr contribution rules

Fyr contributions should improve the Phase1-native language carefully and testably.

Good Fyr contributions include:

- syntax docs;
- examples;
- parser tests;
- runtime behavior tests;
- package/lockfile planning;
- operator workflow docs.

Do not describe Fyr as production-ready until the repository has the implementation and validation evidence to support that claim.

## Website and branding contributions

Website and public-branding work should preserve:

- Phase1/Base1/Fyr visual consistency;
- mobile fit;
- readable public copy;
- clear calls to docs, support, GitHub, and roadmap pages;
- accurate non-claims.

Do not make marketing copy stronger than the evidence in the repo.

## Community and support contributions

Community support work should align with:

- [`docs/community/README.md`](docs/community/README.md)
- [`docs/community/SUPPORT_FORUM_ROADMAP.md`](docs/community/SUPPORT_FORUM_ROADMAP.md)

Support docs must not ask users to post secrets, private keys, tokens, recovery codes, private logs, or unrevised screenshots.

## Testing expectations

Add or update tests when you:

- add a feature;
- change command behavior;
- change docs that are protected by guard tests;
- add a new roadmap or quality gate;
- change security, crypto, Base1, or release claims;
- fix a bug that could recur.

When in doubt, add a small test that protects the claim you are making.

## Commit and PR style

Use clear commit messages:

```text
Add crypto provider template
Update Base1 readiness docs
Fix safe-mode help output
```

PRs should be focused. Avoid mixing unrelated work such as UI changes, crypto docs, Base1 reorganization, and website branding in one PR unless the PR is explicitly a coordinated release update.

## What maintainers may reject

Maintainers may reject or ask for changes when a contribution:

- weakens safe defaults;
- removes compatibility paths without approval;
- makes unsupported security, crypto, OS, Base1, Fyr, or hardware claims;
- introduces secret leakage risk;
- lacks tests for claimed behavior;
- changes too many unrelated areas at once;
- bypasses quality gates;
- adds custom cryptography for real protection;
- makes public docs less clear for new users.

## Non-claims

These guidelines do not guarantee acceptance of any contribution, provide legal advice, launch a support program, or make Phase1, Base1, or Fyr production-ready.

They define the expected contribution process for this repository.
