# Phase1 Real-System Development Guide

Status: active development guide
Scope: real-system direction, evidence ladder, concept promotion rules, Phase1/Base1/Fyr/Optics roles, and next development order.

## Purpose

Phase1 is not being built as a simulator game.

Phase1 starts with simulated and preview surfaces because those surfaces let us test the shape of a system before it is allowed to touch real hardware, real filesystems, real networks, or real recovery paths. Simulation is a proving layer, not the destination.

The long-term direction is a real-life computing system with a Phase1-first operator environment, a Base1 evidence-chain boot foundation, Fyr as the native Phase1 language, and Optics PRO as the operator lens for state, movement, safety, recovery, and system health.

## Reality rule

A concept is not real because it has a name.

A concept becomes real only when it moves through evidence:

```text
idea
fixture
documentation contract
tests
read-only command surface
runtime preview
guarded experiment
VM or emulator evidence
real-device read-only evidence
real-device execution evidence
promotion after review
```

This rule keeps Phase1 ambitious without becoming dishonest.

## Core roles

| Area | Role | Current boundary |
| --- | --- | --- |
| Phase1 | Operator environment, command model, VFS, runtime surface, capability gates, status surfaces, and future Phase universe controller. | Terminal-first virtual OS console with simulated and guarded host-backed components. |
| Base1 | Real-system boot, recovery, validation, hardware evidence, delivery profiles, and promotion ladder. | Evidence-chain target; no broad daily-driver or installer-ready claim until hardware proof exists. |
| Fyr | Native Phase1 language and future system scripting/control language. | Implemented language/toolchain track, still evidence-bound and not production-claimed. |
| Optics PRO | Operator lens for rails, status, Phase movement, state, safety, and recovery visibility. | Read-only/status-only until explicit-gate and invariant evidence exists. |

## Design principles

1. Build toward reality without skipping proof.
2. Keep simulation as the lab, not the product claim.
3. Promote only the narrowest claim supported by evidence.
4. Prefer read-only state visibility before mutation.
5. Prefer pure transition functions before live commands.
6. Prefer VM and QEMU evidence before real-device execution.
7. Preserve explicit approval gates for host, network, boot, recovery, and external effects.
8. Treat every powerful concept as safety-sensitive until tests prove its boundary.

## Real-system ladder

### Level R0: Concept

The idea exists in notes, design docs, or conversation.

Promotion requirement:

- write the concept down;
- name its non-claims;
- define what it must not do yet.

### Level R1: Fixture

The concept exists as static state, sample output, or documentation fixture.

Promotion requirement:

- deterministic fixture;
- no mutation;
- no host effect;
- no external effect.

### Level R2: Tested model

The concept has tests that check invariants and forbidden behavior.

Promotion requirement:

- unit or docs tests;
- forbidden marker tests;
- no accidental claim expansion.

### Level R3: Read-only command surface

The concept is visible inside Phase1 as a command or preview.

Promotion requirement:

- status-only command;
- deterministic output;
- explicit non-claims;
- capability metadata.

### Level R4: Runtime preview

The concept is wired to a runtime state model but still cannot mutate host, origin, boot, recovery, network, or external systems.

Promotion requirement:

- pure transition model;
- invariant tests;
- rollback state visible;
- host-effect and external-effect remain `none`.

### Level R5: Guarded experiment

The concept can perform a limited mutation inside a controlled Phase1 scope.

Promotion requirement:

- explicit operator command;
- safe-mode and trust gate review;
- rollback or reset path;
- audit log;
- failure behavior.

### Level R6: VM evidence

The concept runs in an emulator or VM profile.

Promotion requirement:

- reproducible command path;
- captured logs;
- validation report;
- no real-device claim.

### Level R7: Real-device read-only evidence

The concept observes or reports from real hardware without mutating it.

Promotion requirement:

- hardware profile;
- operator checklist;
- log bundle;
- redaction review;
- explicit read-only result.

### Level R8: Real-device execution evidence

The concept executes on real hardware within a narrow, reversible, operator-approved path.

Promotion requirement:

- device identity;
- backup and recovery plan;
- explicit confirmation phrase;
- rollback or recovery evidence;
- post-run validation.

### Level R9: Promoted real-system capability

The concept is part of the stable system contract.

Promotion requirement:

- repeated evidence;
- docs and tests;
- safety review;
- release note;
- clear support boundary.

## Concept notes to preserve

### Trilateral Phase Movement

Trilateral Phase Movement is the future Phase universe navigation model.

It should not start as live movement. It should start as a pure state model with fixtures and invariant tests.

The three movement planes are:

```text
Plane A: spatial movement
origin / route / axis / path

Plane B: state movement
breadcrumb / trace / current context / selected domain

Plane C: safety movement
safe-portal / rollback / health / risk / lock / dark_phase / host-effect / external-effect
```

A movement is valid only when all three planes agree that the transition is safe, observable, reversible where required, and free of unintended host or external effects.

### Optics PRO

Optics PRO should visualize the Phase universe before it controls it.

Current labels such as `origin`, `route`, `axis`, `path`, `breadcrumb`, `trace`, `safe-portal`, `rollback`, `health`, `risk`, `lock`, `dark_phase`, `host-effect`, and `external-effect` are the right foundation for a real operator lens.

The next step is not live HUD activation. The next step is tested state fixtures that Optics can render consistently.

### Safe portal and rollback

Safe portal and rollback must be visible before they are executable.

Recommended states:

```text
safe-portal=planned
safe-portal=available
safe-portal=blocked
safe-portal=armed
safe-portal=executed
safe-portal=failed

rollback=unavailable
rollback=available
rollback=armed
rollback=executed
rollback=failed
```

Only `executed` may imply that a real recovery action happened.

### Dark phase

`dark_phase` should remain a visible state and risk label, not a stealth claim.

Recommended states:

```text
dark_phase=off
dark_phase=observed
dark_phase=isolated
dark_phase=blocked
```

Do not use dark phase to imply unauthorized persistence, stealth, offensive action, or hidden control.

### Fyr staged runtime and Black Arts

Fyr is the native language of Phase1.

The staged runtime and Black Arts concepts should remain candidate-only until they pass validation and approval.

Allowed early states:

```text
candidate-only
non-live
fixture-backed
validation-required
approval-gated
promotion-blocked
```

Forbidden early states:

```text
live-system-mutation
automatic-promotion
unreviewed-host-effect
unreviewed-network-effect
unreviewed-recovery-action
```

### Base1 real-system path

Base1 is the path from Phase1-as-lab toward Phase1-as-real-system.

The preferred Base1 ladder is:

```text
dry-run
read-only validation
preview bundle
QEMU evidence
VM evidence
real-device read-only evidence
hardware boot evidence
promotion after proof
```

Do not let Phase1 UI, Optics, or Fyr language progress imply Base1 boot readiness. Base1 claims need Base1 evidence.

## Development direction

### Immediate next steps

1. Merge the #391 Optics read-only command surface after its docs-link fix passes CI.
2. Add a dedicated Phase universe state fixture with fields for origin, route, axis, path, breadcrumb, trace, safe portal, rollback, health, risk, lock, dark phase, host effect, and external effect.
3. Add invariant tests proving the fixture is read-only and effect-free.
4. Connect Optics rendering to that fixture without live movement.
5. Create a pure Trilateral Phase Movement transition model.
6. Add tests for valid movement, denied movement, locked movement, rollback-visible movement, and no host/external effects.
7. Only after that, add a preview command such as `phase move preview <direction>`.

### Near-term implementation order

```text
1. phase-state fixture
2. phase-state invariant tests
3. trilateral transition model
4. movement preview command
5. Optics renders movement preview state
6. Fyr can inspect or script preview-only state
7. Base1 records evidence for real-system promotion paths
```

### Real-system development order

```text
Phase1 model first
Optics visibility second
Fyr scripted inspection third
Base1 VM evidence fourth
Base1 real-device read-only evidence fifth
narrow real execution last
```

## What should not happen next

Do not jump directly to live movement.

Do not make Optics mutate origin.

Do not make safe portal execute recovery yet.

Do not claim Base1 boot readiness from UI work.

Do not turn Fyr staged candidates into live system mutation without approval gates.

Do not hide host or external effects behind friendly labels.

## Builder guidance

When building Phase1, ask these questions before every PR:

1. What real-world capability does this move us toward?
2. What is the exact current evidence level?
3. What can this not claim yet?
4. What state is visible to the operator?
5. What mutation is impossible in this PR?
6. What test proves the boundary?
7. What is the rollback or failure path?
8. Does this make the system more real without becoming unsafe or dishonest?

## North star

Phase1 should become a real operator-centered computing environment.

Base1 should make the system touch real hardware only when evidence supports it.

Fyr should become the native language that lets operators and developers express Phase1-native workflows safely.

Optics should make system state clear enough that a human can understand movement, risk, recovery, trust, and effect boundaries before anything dangerous happens.

That is the path from concept to real system.
