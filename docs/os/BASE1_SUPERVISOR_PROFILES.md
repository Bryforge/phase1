# Base1 supervisor profiles

Status: profile contract scaffold
Scope: shared delivery and orchestration profiles for direct-first and supervisor paths

## Purpose

Base1 now keeps both the direct-first path and supervisor orchestration path first-class.

These profiles define the shared contract used by both paths so Phase1 can keep delivery fast while growing toward supervisor orchestration.

Profiles are plain `.env` files under:

```text
profiles/base1/
```

They are deliberately small, inspectable, and compatible with shell tooling.

## Profile set

| Profile | Class | Default delivery | Intended target |
| --- | --- | --- | --- |
| `x200-supervisor-lite` | low-resource | `direct-first` | 4 GB ThinkPad X200-class systems and low-memory validation |
| `x86_64-vm-validation` | vm-validation | `supervisor-lite` | deterministic QEMU evidence and B3 validation scaffolds |
| `workstation-supervisor` | workstation | `workstation-supervisor` | larger-memory development and parallel validation |

## Shared fields

Each profile must define:

```text
BASE1_PROFILE_NAME
BASE1_PROFILE_CLASS
BASE1_PROFILE_TARGET_RAM_MB
BASE1_PROFILE_DEFAULT_DELIVERY_MODE
BASE1_PROFILE_ALLOWED_DELIVERY_MODES
BASE1_PROFILE_DEFAULT_CONCURRENCY
BASE1_PROFILE_MAX_CONCURRENCY
BASE1_PROFILE_DISPLAY_POLICY
BASE1_PROFILE_VM_MEMORY_MB
BASE1_PROFILE_OPENBSD_MEMORY_MB
BASE1_PROFILE_UEFI_MEMORY_MB
BASE1_PROFILE_STORAGE_TIER_POLICY
BASE1_PROFILE_TMPFS_MB
BASE1_PROFILE_ZRAM_MB
BASE1_PROFILE_SWAP_MB
BASE1_PROFILE_SSD_SCRATCH_MB
BASE1_PROFILE_SECURITY_POSTURE
BASE1_PROFILE_CLAIM
BASE1_PROFILE_NON_CLAIM_BOOTABLE
BASE1_PROFILE_NON_CLAIM_INSTALLER
BASE1_PROFILE_NON_CLAIM_HARDENED
BASE1_PROFILE_NON_CLAIM_HYPERVISOR
BASE1_PROFILE_NON_CLAIM_HARDWARE
BASE1_PROFILE_NON_CLAIM_DAILY_DRIVER
```

## X200 supervisor-lite

The X200 profile is the low-resource anchor.

It prioritizes:

- one active staged kernel by default;
- serial/headless execution;
- direct-first delivery when boot delivery matters;
- supervisor-lite when staged evidence matters;
- zram plus SSD scratch plus swap backstop;
- no heavy concurrent VM workstation behavior.

Expected use:

```bash
sh scripts/base1-delivery-mode-plan.sh --dry-run --mode direct-first --profile x200-supervisor-lite
sh scripts/base1-delivery-mode-plan.sh --prepare --mode supervisor-lite --profile x200-supervisor-lite --write-report
```

## x86_64 VM validation

The VM validation profile is the deterministic evidence profile.

It prioritizes:

- serial capture;
- explicit artifacts;
- B3 evidence review;
- GNU/Linux stage checks;
- OpenBSD launch-check evidence;
- no generalized hardware claim.

Expected use:

```bash
sh scripts/base1-b3-vm-validate.sh --dry-run --profile x86_64-vm-validation --write-report
sh scripts/base1-b3-log-bundle-review.sh --review --write-report
```

## Workstation supervisor

The workstation profile is for larger-memory development and parallel validation.

It may plan more concurrency, but still requires explicit evidence and reviewed logs before any claim strengthens.

Expected use:

```bash
sh scripts/base1-delivery-mode-plan.sh --dry-run --mode workstation-supervisor --profile workstation-supervisor
```

## Compatibility rules

- Direct-first and supervisor paths must consume the same profile vocabulary.
- No profile may imply boot readiness by itself.
- No profile may imply hardening proof by itself.
- No profile may imply hypervisor readiness by itself.
- No profile may imply physical hardware validation by itself.
- Storage-backed swap is an OOM backstop, not true RAM.
- X200-class profiles must keep concurrency conservative.

## Future wiring

These profiles should feed:

- `scripts/base1-delivery-mode-plan.sh`;
- `scripts/base1-storage-tier-plan.sh`;
- `scripts/base1-b3-vm-validate.sh`;
- `scripts/base1-b3-log-bundle-review.sh`;
- future direct-kernel handoff scaffolds;
- future supervisor orchestration scaffolds.

## Non-claims

These profiles do not make Base1 bootable, installer-ready, recovery-complete, hardened, hypervisor-ready, hardware-validated, release-candidate ready, or daily-driver ready.

They are compatibility contracts for low-overhead delivery and supervisor orchestration growth.
