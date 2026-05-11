# Base1 supervisor control-plane contract

Status: planning scaffold
Scope: command surface, policy gates, artifact flow, evidence flow, recovery hooks, and non-claim boundaries

## Purpose

Base1 needs a supervisor control plane that can coordinate staged kernels, evidence capture, storage tiers, and recovery hooks without slowing the direct-first path.

The supervisor route must stay useful on low-resource targets such as the X200 while still leaving room for concurrent staged-kernel validation on larger VM or workstation profiles.

## Non-claims

This control-plane contract does not make Base1 bootable, installer-ready, recovery-complete, hardened, hypervisor-ready, hardware-validated, release-candidate ready, or daily-driver ready.

It only defines the supervisor command and policy contract needed before implementation.

## Command surface

The supervisor control plane should expose a small command surface:

```text
status
plan
stage-artifact
validate-artifact
launch-preview
capture-evidence
request-recovery
stop
```

## Policy gates

The control plane must load the selected Base1 profile before doing any work.

The loaded profile defines:

- allowed delivery modes;
- maximum staged-kernel concurrency;
- target RAM budget;
- storage-tier policy;
- evidence requirements;
- recovery behavior;
- non-claim boundaries.

## Command surface

The supervisor control plane should expose a small command surface:

- status
- plan
- stage-artifact
- validate-artifact
- launch-preview
- capture-evidence
- request-recovery
- stop

## Policy gates

The control plane must load the selected Base1 profile before doing any work.

The selected profile must define:

- maximum staged-kernel concurrency
- target RAM budget
- storage-tier policy
- evidence requirements
- recovery behavior
- non-claim boundaries

The control plane must reject work that exceeds the selected profile, especially on X200-class low-resource targets.
