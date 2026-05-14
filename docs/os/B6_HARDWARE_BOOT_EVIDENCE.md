# B6 named hardware boot evidence

Status: X200 phase1 marker observed
Scope: named local hardware boot observation, artifact identity, machine identity, recovery readiness, result capture, and non-claim boundaries

## Purpose

B6 records a named local hardware boot attempt after B1, B2, B3, B4, and B5 evidence exists.

B6 is not the boot action itself. It is the evidence record for what happened during a hardware boot attempt.

The goal is to capture hardware evidence without hiding risk, changing host boot settings, or overstating readiness.

## Required inputs

- B1 read-only detection evidence;
- B2 dry-run assembly evidence;
- B3 reviewed VM evidence;
- B4 reviewed recovery evidence;
- B5 local boot artifact plan;
- explicit artifact path;
- explicit machine name;
- operator-observed boot result;
- recovery path status;
- non-claim boundaries.

## Result states

Allowed B6 result states:

- not_attempted;
- boot_menu_seen;
- boot_started;
- phase1_marker_seen;
- blocked;
- failed.

The first successful hardware evidence target is `phase1_marker_seen` with the expected marker `phase1 6.0.0 ready`.

## Non-claims

B6 named hardware boot evidence does not make Base1 installer-ready, recovery-complete, hardened, hypervisor-ready, release-candidate ready, or daily-driver ready.

## X200 phase1 marker evidence

Observed local hardware result:

- machine: `X200`
- artifact: `build/base1-b3-uefi-proof.img`
- artifact present: `yes`
- artifact SHA256: `688518c1437003c7b8325b1d5d479bc97f77c3404c8fd27dace6d823d406b79b`
- USB target observed before attempt: `/dev/sdb`
- USB partition: `/dev/sdb1`
- USB label: `PHASE1B42`
- expected marker: `phase1 6.0.0 ready`
- observed result: `phase1_marker_seen`
- claim state: `not_claimed`

This records named X200 hardware boot evidence only. It does not make Base1 installer-ready, recovery-complete, hardened, hypervisor-ready, release-candidate ready, or daily-driver ready.
