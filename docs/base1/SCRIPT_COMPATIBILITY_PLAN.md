# Base1 script compatibility plan

Status: active compatibility plan
Scope: future Base1 script organization

## Purpose

This plan protects existing Base1 command paths before any script reorganization happens.

Base1 scripts are public operator surfaces. Even if a future layout such as `scripts/base1/` is introduced, current `scripts/base1-*.sh` paths should remain available as wrappers or compatibility entry points.

## Current rule

Do not move Base1 scripts until compatibility wrappers are planned and tested.

Current script paths remain the stable operator interface:

```text
scripts/base1-preflight.sh
scripts/base1-doc-integrity.sh
scripts/base1-install-dry-run.sh
scripts/base1-recovery-dry-run.sh
scripts/base1-storage-layout-dry-run.sh
scripts/base1-rollback-metadata-dry-run.sh
scripts/base1-network-lockdown-dry-run.sh
scripts/base1-libreboot-*.sh
scripts/base1-recovery-usb-*.sh
scripts/base1-real-device-readonly-*.sh
```

## Future candidate layout

A future organized tools layout may use:

```text
scripts/base1/core/
scripts/base1/dry-run/
scripts/base1/libreboot/
scripts/base1/recovery-usb/
scripts/base1/real-device/
scripts/base1/quality/
```

This is a planning target only. It is not an instruction to move files now.

## Compatibility wrapper rule

If a script is moved in the future:

1. Keep the original `scripts/base1-*.sh` path.
2. Turn the original path into a small wrapper if needed.
3. Preserve command-line arguments exactly.
4. Preserve read-only and dry-run wording.
5. Preserve output fields relied on by tests and docs.
6. Update `docs/base1/INVENTORY.md` and `docs/base1/MIGRATION_TABLE.md`.
7. Add tests proving the old path still works.
8. Run `sh scripts/quality-check.sh base1-docs`.

## Wrapper template

Future wrappers should be simple and auditable:

```sh
#!/usr/bin/env sh
set -eu
SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
exec "$SCRIPT_DIR/base1/<group>/<new-script>.sh" "$@"
```

Use the template only after the target script exists and tests cover the old path.

## Script groups

| Current pattern | Future candidate group | Compatibility decision |
| --- | --- | --- |
| `scripts/base1-preflight.sh` | `scripts/base1/core/` | keep old path as stable entry point |
| `scripts/base1-doc-integrity.sh` | `scripts/base1/quality/` | keep old path as quality entry point |
| `scripts/base1-*-dry-run.sh` | `scripts/base1/dry-run/` | keep old paths as stable entry points |
| `scripts/base1-libreboot-*.sh` | `scripts/base1/libreboot/` | keep old paths as stable entry points |
| `scripts/base1-recovery-usb-*.sh` | `scripts/base1/recovery-usb/` | keep old paths as stable entry points |
| `scripts/base1-real-device-readonly-*.sh` | `scripts/base1/real-device/` | keep old paths as stable entry points |

## Non-claims

This plan does not move scripts. It does not make Base1 installer-ready, hardware-validated, or daily-driver ready. It only defines how future script organization can preserve recoverability and operator compatibility.
