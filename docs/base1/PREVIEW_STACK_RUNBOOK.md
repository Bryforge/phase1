# Base1 Preview Stack Runbook

This runbook describes the current safe Base1 emulator-preview workflow.

It is intentionally preview-only. It does not describe a released Base1 image, a finished installer, a completed recovery process, a hardware validation result, or a daily-driver-ready operating system.

## Purpose

The preview stack gives operators one guarded path for checking Base1 preview inputs, generating an emulator-preview bundle, inspecting that bundle, recording provenance, and verifying checksums.

The stack is useful for preparing evidence before any future bootable-image promotion work. It is not itself a boot validation.

## Current safe stack

The one-command stack is:

```sh
sh scripts/base1-preview-stack.sh \
  --bundle build/base1-preview-stack-demo \
  --kernel build/base1-test-vmlinuz \
  --initrd build/base1-test-initrd.img \
  --image-mb 1 \
  --no-qemu-check
```

The stack runs these steps in order:

1. `scripts/base1-preview-inputs.sh`
2. `scripts/base1-emulator-preview.sh`
3. `scripts/base1-emulator-doctor.sh`
4. `scripts/base1-preview-gate.sh --dry-run`
5. `scripts/base1-preview-provenance.sh`
6. `scripts/base1-preview-verify.sh`

## Inputs

Required inputs:

| Input | Meaning | Boundary |
| --- | --- | --- |
| `--kernel <path>` | Candidate kernel file for emulator preview packaging | Must be an existing file; not validated as a release kernel |
| `--initrd <path>` | Candidate initrd file for emulator preview packaging | Must be an existing file; not validated as a release initrd |
| `--bundle <dir>` | Output bundle directory under `build/` | Must remain under `build/` |
| `--image-mb <n>` | Placeholder raw sandbox size | Preview artifact only |
| `--no-qemu-check` | Skip QEMU presence check | Does not launch QEMU |

## Outputs

A successful stack run produces a preview bundle under `build/`.

Expected outputs include:

| Path | Purpose |
| --- | --- |
| `manifest.env` | Bundle-level preview metadata |
| `README.txt` | Bundle-local summary |
| `base1-sandbox.raw` | Sandbox placeholder artifact |
| `base1-rootfs-preview.tar` | Root filesystem preview archive when available |
| `run-qemu-bundle.sh` | Emulator scaffold script |
| `staging/manifest.env` | Staging metadata |
| `staging/boot/grub/grub.cfg` | Preview GRUB config |
| `staging/boot/vmlinuz` | Copied candidate kernel |
| `staging/boot/initrd.img` | Copied candidate initrd |
| `reports/provenance.env` | Preview provenance record |
| `reports/SHA256SUMS` | Recorded SHA-256 checksums |

## Verification behavior

The final verification step reads `reports/SHA256SUMS` and checks that each listed file still exists and still matches the recorded SHA-256.

A mismatch is treated as bundle drift. The verifier fails instead of silently accepting changed artifacts.

## Safe local smoke flow

```sh
mkdir -p build
printf 'kernel placeholder\n' > build/base1-test-vmlinuz
printf 'initrd placeholder\n' > build/base1-test-initrd.img

sh scripts/base1-preview-stack.sh \
  --bundle build/base1-preview-stack-demo \
  --kernel build/base1-test-vmlinuz \
  --initrd build/base1-test-initrd.img \
  --image-mb 1 \
  --no-qemu-check

sh scripts/base1-preview-verify.sh \
  --bundle build/base1-preview-stack-demo
```

This smoke flow uses placeholder inputs. Passing it proves the preview stack mechanics work; it does not prove Base1 boots.

## Promotion rule

Do not promote any preview bundle to a stronger status unless all of the following are recorded:

1. Exact source commit.
2. Exact kernel and initrd source.
3. Generated `reports/provenance.env`.
4. Generated `reports/SHA256SUMS`.
5. Passing `scripts/base1-preview-verify.sh` output.
6. A human-readable validation report under `docs/base1/validation/`.
7. A clear statement of what was not validated.

## Non-claims

This runbook does not claim that Base1 is bootable.

It also does not claim:

- Base1 is a released OS image.
- Base1 is a secure OS replacement.
- Base1 is daily-driver ready.
- The preview bundle is safe to install on hardware.
- Recovery USB behavior is complete.
- Real hardware has been validated.
- The emulator was launched by the safe preview stack.

## Related scripts

| Script | Role |
| --- | --- |
| `scripts/base1-preview-inputs.sh` | Read-only input checker |
| `scripts/base1-emulator-preview.sh` | Preview bundle generator under `build/` |
| `scripts/base1-emulator-doctor.sh` | Read-only bundle doctor |
| `scripts/base1-preview-gate.sh` | Dry-run guarded handoff gate |
| `scripts/base1-preview-provenance.sh` | Preview provenance and checksum writer |
| `scripts/base1-preview-verify.sh` | Read-only checksum verifier |
| `scripts/base1-preview-stack.sh` | End-to-end safe stack wrapper |

## Test targets

Relevant tests:

```sh
cargo test -p phase1 --test base1_preview_inputs_script
cargo test -p phase1 --test base1_emulator_preview_script
cargo test -p phase1 --test base1_emulator_doctor_script
cargo test -p phase1 --test base1_preview_gate_script
cargo test -p phase1 --test base1_preview_provenance_script
cargo test -p phase1 --test base1_preview_verify_script
cargo test -p phase1 --test base1_preview_stack_script
```
