# Base1 supervisor artifact-flow contract

Status: planning scaffold
Scope: artifact identity, staging, validation, launch-preview handoff, evidence capture, recovery metadata, and non-claim boundaries

## Purpose

Base1 needs a supervisor artifact flow so every staged kernel, initrd, OpenBSD image, GNU/Linux image, storage scratch bundle, and evidence log can be tracked explicitly.

The artifact flow keeps direct-first and supervisor-lite compatible by using the same profile, policy, storage-tier, and evidence vocabulary.

The flow must support real workflow without silently promoting staged artifacts into bootable, installer-ready, hardware-validated, hardened, or daily-driver claims.

## Artifact classes

The supervisor artifact flow recognizes these initial classes:

- kernel;
- initrd;
- uefi-proof-log;
- gnulinux-stage-log;
- openbsd-stage-image;
- openbsd-stage-log;
- storage-scratch-bundle;
- profile-report;
- policy-report;
- validation-report;
- recovery-metadata.

Every artifact must be explicitly named, profile-bound, evidence-bound, and claim-bound.

## Required artifact fields

Each artifact record must include:

- artifact id;
- artifact class;
- source path;
- staged path;
- selected Base1 profile;
- storage-tier policy;
- policy decision;
- expected marker when applicable;
- evidence path;
- validation state;
- claim state;
- non-claim boundaries.

The default claim state is not_claimed.

## Flow stages

The artifact flow moves through these stages:

1. declare artifact;
2. check profile policy;
3. assign storage tier;
4. stage artifact under build/;
5. validate artifact shape;
6. capture evidence path;
7. record launch-preview eligibility;
8. record recovery metadata;
9. preserve non-claims.

No stage may mutate host boot settings, format disks, install Base1, or claim hardware validation.

## X200 rule

For x200-supervisor-lite, artifact flow must default to one active staged kernel, direct-first or supervisor-lite only, zram plus SSD scratch as a workflow backstop, and no supervisor-concurrent launch path.

## VM validation rule

For x86_64-vm-validation, artifact flow may record supervisor-concurrent planning up to the selected profile limit, but launch-preview remains evidence-required and VM-only until reviewed.

## Non-claims

This artifact-flow contract does not make Base1 bootable, installer-ready, recovery-complete, hardened, hypervisor-ready, hardware-validated, release-candidate ready, or daily-driver ready.
