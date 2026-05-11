# Cryptographic algorithm documentation template

Status: template
Scope: Phase1 cryptographic capability registry entries

## Purpose

Use this template for every cryptographic algorithm, design, profile component, or implementation provider that Phase1 documents for future policy use.

The goal is to make every cryptographic choice visible, reviewable, testable, and configurable without overstating security claims.

## Algorithm summary

- Name:
- Family:
- Registry ID:
- Status: default | advanced | compatibility | deprecated | rejected | lab-only
- Intended profile(s): safe-default | high-security | compatibility | post-quantum-preview | lab-only
- Intended control point(s): storage | transport | identity | base1 | plugins | logs/evidence | fyr/packages

## Implementation source

- Provider/library:
- Upstream project:
- License:
- Version/source pinning plan:
- Maintenance status:
- Audit status:

## Allowed use cases

List exactly where this algorithm may be used.

- 

## Disallowed use cases

List where this algorithm must not be used.

- 

## Parameters and limits

Document all relevant parameters.

| Parameter | Value | Notes |
| --- | --- | --- |
| Key size |  |  |
| Nonce/IV size |  |  |
| Tag size |  |  |
| Salt size |  |  |
| Work factor |  |  |
| Message/data limit |  |  |
| Rotation guidance |  |  |

## Security notes

- Known risks:
- Misuse hazards:
- Side-channel considerations:
- Dependency concerns:
- Deprecation triggers:

## Usability notes

- Operator impact:
- Performance impact:
- Compatibility impact:
- Migration difficulty:
- Recovery implications:

## Test vectors

Every production-capable algorithm entry should reference standard test vectors or include a test-vector source.

- Test vector source:
- Phase1 test file:
- Integration command:

```bash
cargo test --all-targets
```

## Profile behavior

Describe behavior by profile.

| Profile | Behavior |
| --- | --- |
| safe-default |  |
| high-security |  |
| compatibility |  |
| post-quantum-preview |  |
| lab-only |  |

## Configuration example

```toml
[crypto.scope.example]
profile = "safe-default"
```

## Migration and rotation

- How to migrate into this algorithm:
- How to migrate away from this algorithm:
- How to rotate keys or parameters:
- Required backup/recovery step:

## Review checklist

- [ ] Uses a reviewed open-source implementation where possible.
- [ ] Does not invent a custom security-critical primitive.
- [ ] Status is clearly labeled.
- [ ] Allowed and disallowed use cases are documented.
- [ ] Parameters and limits are documented.
- [ ] Test vectors are identified.
- [ ] Operator usability impact is documented.
- [ ] Migration/rotation guidance is documented.
- [ ] Non-claims are preserved.

## Non-claims

This entry does not make Phase1 or Base1 cryptographically complete, audited, certified, quantum-safe, hardware-validated, installer-ready, or daily-driver ready.

This entry documents a cryptographic option for review and future policy integration only.
