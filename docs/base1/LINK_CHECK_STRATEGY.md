# Base1 link-check strategy

Status: active link-check planning document
Scope: Base1 markdown organization and compatibility paths

## Purpose

This strategy defines how Base1 markdown links should be checked before and after organization work.

Base1 has public docs, root compatibility files, release/checkpoint mirrors, dry-run scripts, and validation reports. Link safety must be handled before any broad reorganization.

## Current rule

Do not broadly reorganize Base1 markdown until link checking is available through documentation, scripts, tests, or a CI-safe command.

For now, `scripts/base1-doc-integrity.sh` is the minimum link/reference guard. It checks required files, key references, root compatibility paths, release mirrors, script syntax, dry-run wording, and non-claims.

## Required link surfaces

Any link-check strategy must cover:

- `base1/*.md`
- `docs/base1/*.md`
- `docs/base1/releases/*.md`
- `docs/base1/real-device/*.md`
- `docs/base1/real-device/reports/*.md`
- `docs/os/BASE1_*.md`
- root-level Base1 release/checkpoint notes
- README links pointing into Base1 docs

## Compatibility requirements

The checker must preserve both types of paths:

| Path type | Example | Requirement |
| --- | --- | --- |
| Root compatibility path | `RELEASE_BASE1_LIBREBOOT_READONLY_V1.md` | Must remain present and discoverable. |
| Organized mirror path | `docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1.md` | Must remain present and discoverable. |
| Canonical design path | `base1/RECOVERY_USB_DESIGN.md` | Must remain present until a compatibility plan exists. |
| Manual index path | `docs/base1/README.md` | Must link to organization, inventory, readiness, and validation docs. |

## Link-check behavior

A future link checker should:

1. Parse Markdown links with relative paths.
2. Ignore external URLs by default unless an explicit online check mode exists.
3. Ignore anchors initially unless an anchor-check mode is added.
4. Resolve relative links from the source file directory.
5. Fail on missing local targets.
6. Report the source file and missing target.
7. Stay read-only.
8. Run in CI without network access.

## Proposed command

Future command:

```bash
sh scripts/base1-link-check.sh
```

Future quality integration:

```bash
sh scripts/quality-check.sh base1-docs
```

The Base1 docs quality gate should eventually call the link checker after the integrity gate.

## Incremental path

1. Keep `scripts/base1-doc-integrity.sh` as the current minimum guard.
2. Add a read-only Markdown link checker.
3. Add tests proving known Base1 links resolve.
4. Add the link checker to `quality-check.sh base1-docs`.
5. Only then move one documentation group at a time.

## Non-claims

This strategy does not make Base1 installer-ready, hardware-validated, or daily-driver ready. It only defines link-safety expectations for repository organization.
