# Fyr 100% completion plan

Status: evidence-bound completion plan  
Scope: Fyr native language and `black_arts` staged-candidate track  
Non-claim: Fyr is not 100% complete yet.

This plan defines the remaining gates required before Fyr can honestly be described as complete for the current Phase1 roadmap. It keeps public status conservative and prevents documentation-only progress from being counted as runtime capability.

## Current checkpoint

- Public roadmap status: 67% overall.
- Fyr native language public status: 58%.
- `black_arts` staged-candidate evidence, fixtures, visual contract, source-wiring checklist, and patch handoff are landed.
- The first runtime source implementation is still tracked by issue #317.

## Completion principle

A Fyr completion percentage can only advance when the repository contains landed source, tests, fixtures, docs, or status data that prove the claim.

Documentation-only work may prepare a gate, but it must not claim the runtime behavior exists until implementation and tests land.

## Gate F100-1: first staged runtime stub

Primary tracker: issue #317.

Required result:

```text
fyr staged
fyr staged status
fyr staged help
fyr staged nonsense
```

Required behavior:

- `fyr staged` is recognized by the shell.
- `fyr staged` and `fyr staged status` emit the `black_arts` visual runtime markers.
- `fyr staged help` emits usage, boundaries, workspace, and claim boundary rows.
- Unknown staged actions are no-op and help-guided.
- Output remains fixture-backed, non-live, candidate-only, evidence-bound, and claim-boundary aware.
- Output does not include host command, network, shell, or compiler execution markers.

Explicitly blocked:

```text
candidate creation
candidate apply/change behavior
validation execution
promotion execution
discard execution
host command execution
network access
live-system writes
```

Completion evidence:

- `src/main.rs` contains `Some("staged") => fyr_staged(&args[1..])` inside `fyr_command`.
- `src/main.rs` contains `fyr_staged`, `fyr_staged_visual`, `fyr_staged_help`, and `fyr_staged_unknown` helpers.
- Integration tests run Phase1 commands for staged, staged status, staged help, and unknown staged actions.
- Tests assert the required visual markers and forbidden host/network markers.

## Gate F100-2: guarded staged candidate lifecycle

This gate remains blocked until F100-1 lands.

Required command family:

```text
fyr staged plan
fyr staged create
fyr staged apply
fyr staged validate
fyr staged promote
fyr staged discard
```

Required behavior:

- Plan is read-only and explains candidate scope.
- Create writes only inside the declared staged candidate workspace.
- Apply mutates only candidate workspace state, not the live system.
- Validate records deterministic validation evidence.
- Promote requires validation and explicit operator approval.
- Discard is explicit and records evidence.
- Every command reports `live-system   : untouched` until promotion is intentionally implemented and guarded.

Required evidence:

- Integration tests cover each command.
- Fixtures document expected outputs.
- Tests prove blocked live writes, blocked host commands, blocked network access, and explicit approval requirements.

## Gate F100-3: Fyr language capability closure

This gate tracks core language work beyond the staged-candidate shell.

Required capabilities:

- Parser-backed diagnostics remain deterministic.
- Package and module resolution remain VFS-only.
- Assertions and comparison expressions are covered.
- Syntax highlighting has a safe no-color fallback.
- `fyr check`, `fyr build`, `fyr test`, and `fyr run` remain host-independent.

Relevant trackers:

- issue #97: Fyr toolchain bootstrap.
- issue #101: parser diagnostics.
- issue #103: AST and module resolver.
- issue #108: test assertions.
- issue #110: boolean assertions.
- issue #112: comparison assertions.
- issue #113: syntax color coding.

## Gate F100-4: self-hosting readiness boundary

Fyr should not be called self-hosting until Phase1 has landed evidence for:

- Fyr-authored Phase1 workflows.
- Deterministic Fyr package manifests.
- A documented execution/backend strategy.
- Clear non-claim language when Rust remains the implementation language.
- Tests proving the Fyr path does not depend on host Cargo, shell, or network execution.

## Gate F100-5: public status promotion

Only after source and tests land should public status be updated.

Promotion rules:

- Do not raise Fyr percentage for plan-only changes.
- Do not close #317 until the runtime source and tests are merged.
- Do not claim live self-updating, autonomous mutation, self-hosting, or production OS replacement from Fyr staged evidence.
- Update `scripts/update-public-status.py`, `site/status.json`, `site/status-badge.json`, and `docs/status/PROJECT_STATUS.md` together.

## Immediate next implementation target

The next implementation PR should be source-only plus tests for F100-1:

1. Patch `src/main.rs` using `docs/fyr/STAGED_RUNTIME_PATCH_HANDOFF.md`.
2. Add `tests/fyr_black_arts_runtime_wiring.rs`.
3. Verify the four staged command cases.
4. Verify forbidden host/network markers are absent.
5. Close #317 only after the implementation PR is merged.

## Definition of done for Fyr 100%

Fyr reaches 100% for the current roadmap only when:

- F100-1 through F100-5 are complete.
- Every completion claim has landed evidence.
- Public status data matches the implementation state.
- Remaining non-claims are explicit and accurate.
- The repository can prove the behavior through deterministic tests.
