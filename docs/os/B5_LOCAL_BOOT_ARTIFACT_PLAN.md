# B5 local boot artifact plan

Status: planning scaffold
Scope: single explicit local boot artifact, recovery precheck, operator instructions, evidence path, and non-claim boundaries

## Purpose

B5 exists to select one explicit local boot artifact for the fastest safe hardware test path.

B5 follows B1 read-only detection, B2 dry-run assembly, B3 reviewed VM evidence, and B4 reviewed recovery evidence.

The goal is to prepare a named local artifact for hardware boot testing without modifying host boot settings, formatting disks, writing bootloaders, or claiming hardware readiness.

## Selected artifact rule

The first local hardware candidate should be a single UEFI image artifact, selected by path and validated by local evidence only.

The default candidate is `build/phase1-uefi.img`.

The artifact must remain ignored by Git unless it is moved to a release system or external artifact store.

## Required prechecks

- B1 read-only detection evidence exists;
- B2 dry-run assembly evidence exists;
- B3 reviewed VM evidence exists;
- B4 reviewed recovery evidence exists;
- selected artifact path is explicit;
- selected artifact is local-only;
- selected artifact is not staged in normal Git history;
- recovery path remains planned before hardware testing.

## Non-claims

This B5 local boot artifact plan does not make Base1 bootable on hardware, installer-ready, recovery-complete, hardened, hypervisor-ready, hardware-validated, release-candidate ready, or daily-driver ready.
