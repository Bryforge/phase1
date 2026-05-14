# Base1 B6 X200 marker checkpoint v1

Status: checkpoint release note
Scope: named X200 B6 marker evidence, public checkpoint trail, and non-claim boundary
Repository: Bryforge/phase1
Branch: edge/stable
Release-note source commit: `f23fcb822f9c0d11bcd0b07adf71c811fcfd99c1`

## Summary

This checkpoint note records the B6 X200 marker evidence trail for Phase1/Base1.

It packages the named X200 marker checkpoint without strengthening the claim beyond the recorded evidence.

## Evidence chain

| Item | Value |
| --- | --- |
| Checkpoint file | [`../../checkpoints/B6_X200_MARKER_CHECKPOINT.md`](../../checkpoints/B6_X200_MARKER_CHECKPOINT.md) |
| Checkpoint commit | `d4cd1e13d429662f6713466f57a41233d8238416` |
| Checkpoint source commit recorded inside file | `8eeca92294e8fc67437b46f4cb38917a4428e219` |
| Final evidence anchor | `095786e808d3908d27c045f04f3de0b5cd538ab9` |
| Marker result | `phase1_marker_seen` |
| Claim state | `not_claimed` |
| Artifact path | `build/base1-b3-uefi-proof.img` |
| Artifact SHA256 | `688518c1437003c7b8325b1d5d479bc97f77c3404c8fd27dace6d823d406b79b` |

## Validation completed before checkpoint packaging

The checkpoint trail was validated with:

    cargo test --test b6_hardware_boot_evidence_docs
    cargo test --test b3_reviewed_vm_evidence_docs
    cargo test --test boot_readiness_status_docs
    cargo test --test b3_vm_validation_report_docs
    sh scripts/phase1-big-verify.sh --fix --full --base1 --b2 --b3 --status --wiki

## Claim boundary

This checkpoint does not claim:

- installer readiness
- recovery completion
- hardening
- release-candidate readiness
- daily-driver readiness
- broad hardware validation across targets

The only B6 marker result recorded here is `phase1_marker_seen` for the named X200 evidence path.

## Reproduction commands

    cd ~/phase1
    git checkout edge/stable
    git pull origin edge/stable
    cat docs/checkpoints/B6_X200_MARKER_CHECKPOINT.md
    sha256sum build/base1-b3-uefi-proof.img

## Public trail

The public trail should continue to point readers toward:

- `docs/checkpoints/B6_X200_MARKER_CHECKPOINT.md`
- `docs/os/BOOT_READINESS_STATUS.md`
- `docs/status/PROJECT_STATUS.md`
- `site/status.json`

All public wording must preserve `not_claimed` and the non-claims above.
