# Base1 supervisor policy bus

Status: planning scaffold
Scope: profile-gated command authorization, resource limits, storage-tier policy, evidence requirements, recovery policy, and non-claim boundaries

## Purpose

The Base1 supervisor policy bus decides whether a requested supervisor control-plane action is allowed under the selected Base1 profile.

It keeps the X200 path lightweight while allowing larger VM and workstation profiles to use broader supervisor-concurrent validation.

The policy bus must be used before stage-artifact, validate-artifact, launch-preview, capture-evidence, request-recovery, or stop work is promoted beyond planning.

## Required policy inputs

- selected Base1 profile;
- requested supervisor command;
- allowed delivery modes;
- maximum staged-kernel concurrency;
- target RAM budget;
- storage-tier policy;
- evidence requirements;
- recovery behavior;
- non-claim boundaries.

## Initial policy decisions

- allow;
- deny;
- plan-only;
- evidence-required;
- profile-upgrade-required.

## X200 rule

For x200-supervisor-lite, the policy bus must default to one active staged kernel, supervisor-lite only, zram plus SSD scratch as a backstop, and no supervisor-concurrent launch path.

## VM validation rule

For x86_64-vm-validation, the policy bus may allow supervisor-concurrent planning up to the profile concurrency limit, but this remains VM evidence only until reviewed.

## Non-claims

This policy bus does not make Base1 bootable, installer-ready, recovery-complete, hardened, hypervisor-ready, hardware-validated, release-candidate ready, or daily-driver ready.
