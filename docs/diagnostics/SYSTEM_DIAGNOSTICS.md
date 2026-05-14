# Phase1 system diagnostics

Status: design and operator contract  
Scope: local, sanitized diagnostics report for Phase1, Base1, Fyr, repository state, toolchain state, and selected validation commands.  
Non-claim: this is not telemetry, not automatic remote upload, not a support backdoor, not a secret collector, and not a production health monitor.

## Goal

Create one repeatable diagnostic command that collects the information needed to debug and maintain Phase1 without asking the operator to paste scattered terminal output.

The diagnostic report is meant to answer:

```text
Which branch and commit are active?
What files are modified or untracked?
Which toolchain commands are available?
Which core docs and scripts exist?
Which focused validation commands pass or fail?
Where is the generated report?
What boundaries and non-claims are still in force?
```

## Command

```sh
sh scripts/phase1-system-diagnostics.sh --quick
```

Default output path:

```text
build/diagnostics/latest.md
```

Timestamped reports are also written under:

```text
build/diagnostics/
```

## Modes

```text
--quick       collect repository/toolchain state and run focused fast checks
--full        collect state and run cargo test --workspace --all-targets
--no-tests    collect state only
--repo-copy   after generating the report, copy it to docs/diagnostics/LATEST_LOCAL_DIAGNOSTICS.md for operator review and manual commit
--help        print usage
```

`--repo-copy` is explicit because local diagnostic reports can still reveal local paths, branch names, or private filenames. The script does not run `git add`, `git commit`, `git push`, network upload, package installation, or host mutation.

## Safety contract

The diagnostic script must:

- stay local;
- avoid network upload;
- avoid credential collection;
- avoid reading arbitrary home directories;
- avoid printing environment variables wholesale;
- avoid secrets, tokens, cookies, private keys, recovery codes, and credentials;
- avoid package installation;
- avoid destructive disk tools;
- avoid firmware, bootloader, partition, mount, or device writes;
- write reports only under `build/diagnostics/` unless `--repo-copy` is explicitly provided;
- make repo-copy output reviewable before any manual commit;
- preserve all Phase1, Base1, and Fyr non-claims.

## Collected sections

The report should include:

```text
Report metadata
Repository state
Git status summary
Modified/untracked path list
Toolchain availability
Core Phase1/Base1/Fyr file presence
Focused validation command results
Local-only safety boundaries
Next operator commands
```

## Non-claims

This report does not prove Phase1 is production-ready, installer-ready, daily-driver ready, hardened, cryptographically complete, hardware-validated, recovery-complete, release-candidate ready, live-self-updating, autonomous, or safe for hostile code.

## Recommended flow

```sh
sh scripts/phase1-system-diagnostics.sh --quick
cat build/diagnostics/latest.md
```

For a full gate report:

```sh
sh scripts/phase1-system-diagnostics.sh --full
```

To create a repo-reviewable local snapshot after reviewing the content:

```sh
sh scripts/phase1-system-diagnostics.sh --quick --repo-copy
git diff -- docs/diagnostics/LATEST_LOCAL_DIAGNOSTICS.md
```

Only commit the repo-copy file when the operator has reviewed it and confirmed that it contains no private or sensitive local information.
