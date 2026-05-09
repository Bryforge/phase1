# Base1 rollback metadata design

The Base1 rollback metadata design defines the safe record format for restoring a Phase1-first Base1 system after an update, install preview, or recovery event.

This checkpoint is documentation-only. It does not add mutating installer behavior.

## Goal

Provide a conservative metadata contract that lets Base1 describe what changed, what can be restored, and what recovery path exists without hiding risks or weakening the host boundary.

## Metadata scope

Rollback metadata should describe:

- Base1 version.
- Phase1 version.
- Boot target before the change.
- Boot target after the change.
- State directory path.
- Recovery directory path.
- Timestamp.
- Hardware target summary.
- Validation status.
- Operator confirmation status.
- Whether destructive writes occurred.

## Example metadata

```text
base1 rollback metadata
base1_version      : foundation
phase1_version     : v5.0.0
stable_version     : v4.4.0
previous_stable    : v4.3.0
boot_before        : host default
boot_after         : phase1
state_path         : /state/phase1
recovery_path      : /recovery
writes             : no
operator_confirmed : no
status             : preview only
```

## Guardrails

- Do not store secrets.
- Do not store credentials.
- Do not store private keys.
- Do not imply rollback was tested unless it was actually tested.
- Do not overwrite rollback metadata silently.
- Do not allow rollback metadata to replace a recovery shell.
- Do not mark destructive writes as safe by default.

## Promotion gate

Rollback metadata can only become part of an installer or updater after:

1. Preview metadata exists.
2. Secret redaction is tested.
3. Metadata write location is explicit.
4. Recovery shell path is tested.
5. State export path is tested.
6. Restore instructions are documented.
7. Final confirmation exists for destructive actions.
