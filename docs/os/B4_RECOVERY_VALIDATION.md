# B4 recovery validation

Status: planning scaffold
Scope: recovery path, rollback path, evidence capture, failure handling, boot artifact safety, and non-claim boundaries

## Purpose

B4 exists to prove that Base1 can describe and validate a safe recovery path before any stronger local hardware boot claim.

The goal is to make hardware boot work faster without risking the host system, disks, firmware settings, or existing boot chain.

B4 is required before Base1 promotes from reviewed VM evidence to named local hardware boot evidence.

## Required recovery model

The B4 recovery model must define:

- selected Base1 profile;
- boot artifact identity;
- recovery artifact identity;
- rollback path;
- emergency stop path;
- evidence output path;
- operator-visible failure reason;
- non-mutating dry-run mode;
- non-claim boundaries.

The first implementation must be dry-run only.

## Fast hardware path rule

The fastest safe path is:

1. keep B1 read-only detection complete;
2. keep B2 dry-run assembly complete;
3. keep B3 reviewed VM evidence complete;
4. add B4 recovery validation;
5. create one explicit local boot artifact;
6. boot that artifact only after recovery evidence exists.

B4 must not modify host boot settings, write bootloaders, format disks, mount filesystems, enable swap, install packages, fetch network resources, or claim hardware readiness.

## Completion criteria

B4 recovery validation is complete when:

- the recovery validation script exists;
- the script has valid shell syntax;
- dry-run mode writes a local report under build/;
- the selected profile is loaded;
- the report records recovery path planned;
- the report records rollback path planned;
- the report records emergency stop path planned;
- the report records claim state as not_claimed;
- tests preserve all non-claim boundaries.

## Non-claims

This B4 recovery validation scaffold does not make Base1 bootable, installer-ready, recovery-complete, hardened, hypervisor-ready, hardware-validated, release-candidate ready, or daily-driver ready.
