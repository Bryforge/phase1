# Base1 supervisor orchestration

Status: planning scaffold
Scope: Base1 supervisor/control-plane model, staged kernel slots, policy flow, storage tiers, evidence flow, and X200-safe limits

## Purpose

Base1 keeps two first-class delivery paths:

- direct-first: shortest first-kernel route;
- supervisor orchestration: Base1 control plane coordinating staged kernels, policy, storage tiers, logs, and evidence.

The supervisor route is now the main Base1 design goal, but it must not slow Phase1 growth. Direct-first remains available so Base1 can keep shipping small, testable, low-risk slices.

## Supervisor model

The supervisor is not a hypervisor claim.

It is a control-plane design for:

- selecting one active staged kernel by default;
- keeping staged GNU/Linux and OpenBSD paths evidence-bound;
- applying shared profile policy;
- enforcing memory and storage-tier limits;
- routing logs into local evidence bundles;
- preserving recovery and fallback boundaries.

## Initial profiles

| Profile | Intent | Default |
| --- | --- | --- |
| `x200-supervisor-lite` | 4GB-class low-resource target | direct-first / supervisor-lite |
| `x86_64-vm-validation` | deterministic VM evidence profile | supervisor-lite |
| `workstation-supervisor` | higher-memory development target | supervisor-concurrent |

## X200 supervisor-lite rule

For the ThinkPad X200 / 4GB-class target:

- one active staged kernel by default;
- no default concurrent multi-kernel execution;
- serial/headless evidence preferred;
- zram plus SSD scratch/swap backstop;
- no hypervisor claim;
- no hardware-ready claim;
- no daily-driver claim.

## Storage-tier rule

Base1 can use storage tiers for workflow speed, but storage-backed memory must be explicit and evidence-bound:

- RAM first for hot control-plane state;
- zram for compressed memory relief;
- SSD scratch for build/runtime spillover;
- swap backstop only when required;
- no claim that storage is equivalent to RAM.

## Orchestration contract

The supervisor route shares the same contract as direct-first:

- profile name;
- delivery mode;
- staged artifact IDs;
- memory policy;
- storage-tier policy;
- log paths;
- evidence state;
- non-claims.

## Non-claims

This document does not make Base1 bootable, installer-ready, recovery-complete, hardened, hypervisor-ready, hardware-validated, release-candidate ready, or daily-driver ready.
