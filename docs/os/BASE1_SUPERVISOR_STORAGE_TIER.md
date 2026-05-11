# Base1 supervisor storage-tier contract

Status: planning scaffold
Scope: RAM, zram, tmpfs, SSD scratch, swap backstop, evidence paths, and safety boundaries for Base1 supervisor workflows

## Purpose

Base1 needs a storage-tier model that keeps low-resource systems usable without pretending disk is equivalent to RAM.

The storage tier exists to support real workflow: artifact staging, temporary execution workspace, evidence capture, recovery metadata, and bounded scratch expansion.

The X200-class path must stay lightweight: one active staged kernel, small tmpfs, zram first, SSD scratch second, swap as a backstop only.

## Tier order

The supervisor storage model uses this order:

1. real RAM for hot state;
2. small tmpfs for tiny high-churn files;
3. zram for compressed memory pressure relief;
4. SSD scratch for artifact staging and temporary workspace;
5. swap backstop for survival, not speed;
6. persistent evidence logs under build/ for review.

SSD scratch may improve workflow throughput for staged artifacts and temporary work, but it must not be described as RAM-equivalent.

## Required policy fields

Each profile must define:

- target RAM budget;
- tmpfs budget;
- zram budget;
- SSD scratch budget;
- swap backstop budget;
- maximum staged-kernel concurrency;
- storage-tier policy;
- evidence path policy;
- non-claim boundaries.

The supervisor policy bus must reject storage requests that exceed the selected profile.

## X200 rule

For x200-supervisor-lite, the storage tier must default to low-memory behavior: tiny tmpfs, zram-first pressure relief, SSD scratch as a workflow backstop, and one active staged kernel.

The X200 profile must not enable supervisor-concurrent as a launch path.

## VM validation rule

For x86_64-vm-validation, build-directory scratch may be used for deterministic evidence, but it remains VM evidence only until reviewed.

## Non-claims

This storage-tier contract does not make Base1 bootable, installer-ready, recovery-complete, hardened, hypervisor-ready, hardware-validated, release-candidate ready, or daily-driver ready.
