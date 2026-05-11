# Base1 B2 dry-run assembly output review

Status: initial output review
Scope: secret-redaction and safe-output review for `scripts/base1-b2-assembly-dry-run.sh --dry-run --profile <profile>`

## Purpose

This review records the expected safe-output boundary for the B2 dry-run assembly preview.

The B2 command should print only planning hints, selected profile data, dry-run status, preview sections, known limitations, and next validation steps. It should not print secrets, credentials, private keys, recovery codes, personal tokens, environment variables, private logs, or mutable host configuration contents.

## Command reviewed

```bash
sh scripts/base1-b2-assembly-dry-run.sh --dry-run --profile x86_64-vm-validation
```

## Safe output fields

The initial B2 output is expected to include only these high-level fields:

- command start/completion markers;
- B2 readiness level;
- `writes: no`;
- `mutation: no`;
- `network: no`;
- selected profile;
- machine architecture hint;
- firmware hint;
- profile assumptions;
- image-builder preview status;
- boot handoff preview status;
- installer preview status;
- recovery preview status;
- rollback preview status;
- validation bundle planned path;
- explicit known limitations;
- next validation step.

## Output that must not appear

The B2 output must not print:

- tokens;
- private keys;
- passwords;
- credentials;
- recovery codes;
- GitHub, SSH, Apple ID, email, or cloud credentials;
- environment variables;
- full private logs;
- private user files;
- secret-bearing kernel command-line values;
- mutable boot-loader configuration contents;
- EFI variable contents;
- partition table write commands;
- package-manager install commands;
- network request contents.

## Source-output review

The initial script output is intentionally static and preview-oriented.

Current source-output observations:

- It reads only the machine architecture through `uname -m` or `arch`.
- It checks only `/sys/firmware/efi` directory presence for a firmware hint.
- It does not print `/proc/cmdline`.
- It does not print environment variables.
- It does not print file contents from `/boot`, `/etc`, EFI variables, initramfs files, or partitions.
- It does not call network tools.
- It does not call package managers.
- It does not call mutating disk or boot-loader commands.
- It keeps bootability, installer readiness, recovery completion, rollback completion, hardening, VM validation, hardware validation, and release-candidate readiness explicitly unclaimed.

## Required review checklist

Before B2 is considered complete, confirm:

- [x] output avoids environment variables;
- [x] output avoids credentials and tokens;
- [x] output avoids private keys and recovery codes;
- [x] output avoids private file contents;
- [x] output avoids mutable boot configuration contents;
- [x] output avoids EFI variable contents;
- [x] output avoids secret-bearing kernel command-line values;
- [x] output prints `writes: no`;
- [x] output prints `mutation: no`;
- [x] output prints `network: no`;
- [x] output keeps B2 claims bounded to dry-run preview only;
- [ ] script/test suite passes in CI or local validation.

## Related docs

- [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md)
- [`B2_DRY_RUN_ASSEMBLY_PLAN.md`](B2_DRY_RUN_ASSEMBLY_PLAN.md)
- [`B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md`](B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md)
- [`B2_DRY_RUN_ASSEMBLY_VALIDATION.md`](B2_DRY_RUN_ASSEMBLY_VALIDATION.md)

## Non-claims

This output review does not make Base1 bootable, installer-ready, recovery-complete, hardened, VM-validated, hardware-validated, release-candidate ready, or daily-driver ready.

It records a safe-output review for the current B2 dry-run assembly preview.
