# B22 polished Phase1 system plan

Status: planning scaffold

Scope: ThinkPad X200 / Libreboot / SeaBIOS payload / external USB GRUB / Phase1 polished system surface.

## Purpose

B22 turns the proven B20/B21 path into a polished Phase1-facing system surface.

The proven launch route is:

`Libreboot -> SeaBIOS payload -> USB GRUB -> Phase1 console`

B22 keeps that route and improves presentation, operator flow, safety, and next-step readiness.

## Lessons applied

### SeaBIOS display lesson

The SeaBIOS GRUB screen remains in a small text mode. B22 must not rely on display resizing.

B22 must:

- use console output first;
- avoid long wrapped lines;
- keep menu entries short;
- avoid assuming gfxterm works;
- treat splash mode as optional, never required;
- keep a guaranteed text-safe path.

### OpenBSD evidence lesson

The OpenBSD stage showed that a visible or available boot path can still have console and evidence limitations. B22 applies the same boundary discipline:

- separate observed behavior from claimed behavior;
- keep non-claim language visible;
- preserve fallback modes;
- record evidence states only when the operator can actually observe them;
- do not promote a release, hardening, installer, recovery, or daily-driver claim from a partial boot/control-surface result.

### Privacy lesson

Operational docs and helper output must not hard-code local private IP addresses. Use placeholders such as:

- `<X200_IP>`
- `<MAC_IP>`
- `<LAN_HOST>`

## B22 target

B22 should provide:

- a polished text-safe Phase1 main menu;
- optional splash/gfx mode if assets are present and the firmware path supports it;
- automatic fallback to text-safe mode;
- evidence/status pages;
- hardware profile pages;
- recovery and safety pages;
- a Linux/OpenBSD pivot page;
- a future Phase1 handoff lab page;
- no internal disk writes.

## Success result

`phase1_polished_system_seen`

This means the B22 polished Phase1 system surface appears through the proven SeaBIOS USB GRUB route.

Optional additional result:

`phase1_splash_mode_seen`

This means the optional splash/gfx mode appears. It is not required for success.

## Non-claims

B22 does not make Base1 installer-ready, recovery-complete, hardened, release-candidate ready, or daily-driver ready.

B22 does not write the internal disk and does not claim a completed operating system boot.
