# White Arts nominal-state matrix

This matrix defines what White Arts means by functional, nominal, and integrity validated.

The matrix is a planning contract. A row is not considered complete until the listed test command, integrity gate, failure behavior, recovery path, and claim boundary exist.

| System | Owner surface | Nominal signal | Test command | Integrity gate | Failure behavior | Recovery path | Claim boundary |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Phase1 boot selector | preboot UI | boot selector renders safe defaults | `cargo test --test bleeding` | release metadata and theme docs agree | fail visibly, keep safe mode on | reset boot config | virtual console only |
| Phase1 shell | command loop | commands run with stable prompts | `cargo test --test smoke` | registry/help/man coverage | report clear errors | return to root shell | no OS replacement claim |
| VFS/proc/dev/logs | kernel model | files, proc, dev, and audit commands respond | `cargo test --test smoke` | VFS tests and audit tests | no host mutation | volatile state reset | simulated subsystem |
| Command registry | registry/help | aliases, help, man, completion work | `cargo test --test smoke` | registry unit tests | unknown commands fail clearly | help guidance | command metadata only |
| Policy gates | safe mode/host trust | risky host-backed actions are blocked by default | `cargo test --test storage_smoke` | command capability metadata | deny by default | re-enable safe mode | not hardened sandbox |
| Storage/Git/Rust | guarded workflows | trusted workflows validate inputs | `cargo test --test storage_smoke` | storage docs and script checks | reject unsafe paths/URLs | clean workspace | experimental host-backed |
| WASI-lite plugins | plugin runtime | plugin manifests run without host shell | `cargo test --test phase1_launch` | manifest and runtime tests | reject host passthrough | disable plugin | not a certified sandbox |
| Program Loading + Analysis | `analyze` commands | load/list/inspect report metadata only | `cargo test --test analysis_load_metadata` | no-execute rows in tests | missing records fail clearly | reload VFS sample | no dynamic analysis claim |
| Portal | local metadata portals | open/list/enter/close work locally | `cargo test --test portal_floor1_runtime` | portal contract tests | invalid actions are no-op | close portal | no network portal claim |
| Nest | recursive metadata contexts | spawn/enter/tree/destroy work locally | `cargo test --test nest_tree` | nest contract tests | invalid actions are no-op | return to root | no runtime child kernel claim |
| Fyr | native language | check/build/test/run work in VFS | `cargo test --test fyr_f4_vfs_workflows` | Fyr docs and fixtures | diagnostics stay scoped/redacted | regenerate VFS package | growing language surface |
| Base1 docs | OS foundation docs | docs, links, inventories pass | `sh scripts/base1-doc-integrity.sh` | Base1 link and inventory gates | fail read-only | restore docs from git | not installer-ready |
| Base1 recovery | dry-run/recovery docs | recovery plans are documented and dry-run checked | `cargo test --test base1_recovery_usb_validation_bundle` | recovery validation reports | no destructive writes | recovery checklist | no recovery-complete claim |
| Website/status | public surface | status JSON, homepage, and release metadata agree | `cargo test --test website_phase_b` | release metadata tests | fail CI | regenerate status | public status only |
| Security/crypto docs | policy docs | crypto policy docs preserve non-claims | `sh scripts/security-crypto-doc-integrity.sh` | security crypto docs gate | fail read-only | restore docs from git | not cryptographically complete |
| CI/quality | workflows and gates | formatting, clippy, tests, quality gates pass | `cargo test --all-targets` | GitHub workflows and quality scripts | block promotion | fix branch before merge | evidence-bound quality |

## Required fields for future rows

Every new White Arts row must include:

```text
system
owner surface
nominal signal
test command
integrity gate
failure behavior
recovery path
claim boundary
```

## Nominal status meanings

```text
nominal          : expected checks pass
warning          : non-blocking issue found
blocked          : promotion blocked
unknown          : evidence missing
not-applicable   : outside this check scope
```

## Non-claims

A nominal row is not a production-hardening, malware-safety, forensic-admissibility, installer-readiness, or daily-driver-readiness claim.
