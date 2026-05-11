# Base1 dual-path delivery design

Status: design scaffold
Scope: preserving a fast first-kernel delivery path while Base1 grows into a supervisor orchestration system

## Purpose

Base1 should not choose between a simple first-kernel path and a supervisor orchestration system too early.

The intended model is dual-path delivery:

- **Direct path:** a first-kernel or single-kernel path that gives the shortest delivery route to a bootable Base1 target.
- **Supervisor path:** a supervisor/control-plane path that orchestrates staged kernels, policy, logs, recovery boundaries, and workflow isolation.
- **Shared contract:** both paths use the same profile names, policy vocabulary, log model, evidence reports, non-claims, and storage-tier assumptions.

This keeps Phase1/Base1 useful quickly without blocking the deeper invention.

## Design principle

Base1 is becoming a supervisor orchestration system, but it must retain a direct-kernel path so development does not stall behind full hypervisor complexity.

The direct path is the minimal delivery path.

The supervisor path is the long-term architecture path.

The shared contract is the compatibility layer.

## Path A: direct first-kernel delivery

The direct path is for minimal delivery time.

Expected shape:

```text
firmware / QEMU / hardware profile
  -> bootloader
      -> Base1 kernel or first supported kernel payload
          -> Base1 init/runtime boundary
          -> logs and recovery hooks
```

Primary goals:

- reduce boot path complexity;
- keep memory use low;
- keep X200-class targets viable;
- support fast B3/B4 iteration;
- make one boot artifact explicit;
- preserve boot logs and non-claims;
- avoid waiting for complete supervisor/hypervisor features.

Direct path does not mean insecure path. It still uses Base1 profile policy, log capture, storage-tier planning, recovery assumptions, and evidence review.

## Path B: supervisor orchestration delivery

The supervisor path is for maximum security workflow and long-term Base1 identity.

Expected shape:

```text
firmware / QEMU / hardware profile
  -> Base1 supervisor/control plane
      -> staged GNU/Linux kernel environment
      -> staged OpenBSD boot environment
      -> Base1 runtime/service environment
      -> shared policy/log/evidence bus
```

Primary goals:

- isolate staged kernels and workflows;
- capture concurrent or sequential evidence;
- keep host-tool execution guarded;
- enable policy-controlled execution;
- support recovery and rollback planning;
- let Linux/OpenBSD serve as useful stages without claiming Base1 is Linux/OpenBSD;
- grow toward supervisor/hypervisor behavior only when evidence supports it.

Supervisor path must not be described as a hypervisor, hardened release, physical-hardware validation, or daily-driver system until implementation and reviewed evidence support those claims.

## Shared contract

Both paths must share these concepts:

| Contract area | Requirement |
| --- | --- |
| Profile names | Use stable names such as `x86_64-vm-validation`, `x200-supervisor-lite`, and future direct-kernel profiles. |
| Evidence state | Use `not_claimed`, `evidence-incomplete`, `evidence-present`, and `reviewed` carefully. |
| Boot artifact | Record the exact kernel/image/initrd/ISO/UEFI artifact used. |
| VM/hardware profile | Record runtime, firmware mode, memory, CPU count, and display/serial policy. |
| Logs | Capture serial logs, summary env files, and review results under `build/`. |
| Storage tiers | Use RAM tmpfs, zram, SSD scratch, and swap backstop as separate tiers. Do not call storage true RAM. |
| Non-claims | Preserve no installer, no hardware validation, no hardening proof, no daily-driver claim unless separately proven. |
| Compatibility | Direct path and supervisor path must be able to consume the same evidence and profile files. |

## Minimal delivery route

The fastest useful route is:

1. Keep B2 local focused-suite evidence passing.
2. Keep B3 UEFI proof and kernel/initrd handoff evidence passing.
3. Add direct-kernel profile planning without removing supervisor scaffolds.
4. Keep GNU/Linux and OpenBSD stages as supervisor-path inputs.
5. Review the B3 log bundle.
6. Promote only the exact claim supported by reviewed evidence.

## Performance model

The direct path should be preferred when:

- RAM is limited;
- only one kernel/runtime is needed;
- startup time matters;
- debug complexity must stay low;
- the target is X200-class hardware.

The supervisor path should be preferred when:

- isolation matters more than raw minimalism;
- multiple staged kernels provide useful evidence;
- workflow separation matters;
- recovery and policy routing are being tested;
- a workstation-class profile is available.

## X200 policy

For a 4 GB ThinkPad X200-class target, the default should be:

```text
profile: x200-supervisor-lite
mode: direct-first when boot delivery matters
mode: supervisor-lite when staged evidence matters
concurrency: 1 active staged kernel by default
storage: zram pressure relief + SSD scratch + swap backstop
```

The X200 should be treated as a low-resource operator target, not a high-concurrency VM workstation.

## Delivery mode matrix

| Mode | Purpose | Expected resource use | Claim level |
| --- | --- | --- | --- |
| `direct-first` | Fastest path to a bootable first-kernel target | lowest | no boot claim until validated |
| `supervisor-lite` | One active staged kernel plus Base1 control plane | low/moderate | orchestration scaffold only |
| `supervisor-concurrent` | Multiple staged kernels under orchestration | high | VM evidence only until reviewed |
| `workstation-supervisor` | broader workflow and parallel validation | highest | still evidence-bound |

## Required near-term implementation pieces

- `scripts/base1-delivery-mode-plan.sh`
- `docs/os/BASE1_DUAL_PATH_DELIVERY.md`
- tests for direct path and supervisor path compatibility
- future direct-kernel handoff scaffold
- future supervisor orchestration scaffold
- common profile/evidence vocabulary

## Non-claims

This design does not make Base1 bootable, installer-ready, recovery-complete, hardened, hardware-validated, release-candidate ready, hypervisor-ready, or daily-driver ready.

It defines how Base1 can pursue both first-kernel delivery and supervisor orchestration without fragmenting the project or slowing Phase1 growth.
