# White Arts integrity validation plan

The White Arts integrity layer is read-only first. It verifies that required project surfaces exist, agree with each other, and preserve safety boundaries.

## Integrity goals

- make checks repeatable
- make failures explainable
- avoid silent mutation
- preserve safe defaults
- keep release-facing claims evidence-bound
- block promotion when required evidence is missing
- ensure release metadata agrees across public surfaces

## Required checks

| Check group | Required evidence | First gate |
| --- | --- | --- |
| Documentation | required docs exist and link to current surfaces | docs guard tests |
| Scripts | required scripts exist and pass shell syntax where applicable | `sh -n` and script tests |
| Release metadata | release metadata agrees across public surfaces including README, website, Cargo, status JSON, and docs | `cargo test --test release_metadata` |
| Generated docs | managed blocks exist and preserve current base/edge paths | `cargo test --test docs_sync_guard` |
| Base1 | Base1 links and inventories are intact; docs, inventories, release archives, and link checks pass | `sh scripts/base1-doc-integrity.sh` |
| Security/crypto | security and crypto docs preserve guardrails; crypto policy, provider registry, profiles, and non-claims exist | `sh scripts/security-crypto-doc-integrity.sh` |
| Fyr | roadmap, safety docs, package behavior, and fixtures pass | Fyr focused tests |
| Analysis | analysis commands preserve no-execute boundaries; metadata-only load/list/inspect preserve no-execute rows | analysis focused tests |
| Website | homepage, status, assets, and release badges agree | website focused tests |
| CI | workflow presence and validation posture are documented | quality/security workflow tests |

## Test anchors

The integrity plan intentionally preserves these exact phrases for documentation guard tests:

```text
release metadata agrees across public surfaces
Base1 links and inventories are intact
security and crypto docs preserve guardrails
analysis commands preserve no-execute boundaries
```

## Report shape

White Arts integrity reports should use this shape:

```text
white-arts integrity report
status           : nominal | warning | blocked | unknown
scope            : <scope>
checks           : <count>
passed           : <count>
failed           : <count>
mutations        : none
host-execution   : disabled by default
repair-policy    : staged-candidate-only
blocked-claims   : <claims>
```

## Failure behavior

Failures should:

- identify the missing or inconsistent evidence
- name the affected system
- name the blocked claim or blocked promotion
- recommend a reviewable repair plan
- avoid printing secrets or host-specific sensitive paths
- avoid writing unless the operator explicitly chooses a staged repair command

## Promotion rule

A White Arts integrity check may support promotion only when:

- its test is present
- its docs are linked
- it preserves non-claims
- it has clear failure behavior
- its recovery path is documented
- it has been locally and CI validated

## Non-claims

Passing integrity checks does not make the project production hardened, certified, malware-safe, forensic-admissible, installer-ready, daily-driver ready, or cryptographically complete.
