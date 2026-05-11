# Crypto provider documentation template

Status: template
Scope: future cryptographic implementation provider documentation

## Purpose

Use this template for every cryptographic provider, library, crate, or platform capability that Phase1 may consider for future crypto policy implementation.

This template is documentation-first. Completing it does not approve a provider for production security claims.

## Provider summary

- Provider name:
- Provider type: crate | library | system API | external tool | platform capability
- Registry ID:
- Status: candidate | documented | test-vector-covered | profile-eligible | default-eligible | compatibility-only | lab-only | rejected
- Intended profile(s): safe-default | high-security | compatibility | post-quantum-preview | lab-only
- Intended control point(s): storage | transport | identity | base1 | plugins | logs/evidence | fyr/packages | lab | docs | tests

## Source and license

- Upstream project:
- Package/crate name:
- Source repository:
- Documentation URL:
- License:
- License compatibility review:
- Version or source pinning plan:
- Maintenance status:

## Supported capabilities

| Capability | Supported | Notes |
| --- | --- | --- |
| Entropy/random generation |  |  |
| Hashing |  |  |
| Message authentication |  |  |
| Key derivation/password hashing |  |  |
| Symmetric encryption/AEAD |  |  |
| Public-key signatures |  |  |
| Public-key encryption/KEM |  |  |
| Key agreement |  |  |
| Post-quantum algorithms |  |  |
| Serialization/encoding |  |  |

## Supported platforms

- Linux:
- macOS:
- Windows:
- Raspberry Pi targets:
- ThinkPad/Base1 targets:
- Web/WASI/plugin targets:

## Allowed use cases

List exactly where this provider may be used.

- 

## Disallowed use cases

List where this provider must not be used.

- 

## Profile compatibility

| Profile | Provider status |
| --- | --- |
| safe-default |  |
| high-security |  |
| compatibility |  |
| post-quantum-preview |  |
| lab-only |  |

## Control-point compatibility

| Control point | Provider status |
| --- | --- |
| storage |  |
| transport |  |
| identity |  |
| base1 |  |
| plugins |  |
| logs/evidence |  |
| fyr/packages |  |
| lab/docs/tests |  |

## Test-vector coverage

- Test-vector source:
- Covered algorithms:
- Phase1 test file:
- Integration command:

```bash
cargo test --all-targets
```

## Failure behavior

Document how this provider behaves on:

- unsupported algorithm;
- unsupported platform;
- invalid key material;
- invalid signature or tag;
- invalid nonce/IV;
- invalid configuration;
- missing feature flag;
- dependency failure.

Expected behavior should be fail-closed with clear operator-visible errors.

## Fallback behavior

- Is fallback allowed: yes/no
- Fallback target:
- Required warning:
- Required audit event:
- Conditions where fallback is rejected:

Fallback must never silently change the intended security posture.

## Audit and logging expectations

Future runtime use should log redacted metadata only:

- provider ID;
- profile;
- control point;
- operation class;
- success/failure;
- reason for rejection, when applicable.

Logs must not include secrets, private keys, seed material, tokens, raw credentials, or sensitive plaintext.

## Review checklist

- [ ] Provider metadata is complete.
- [ ] License compatibility is reviewed.
- [ ] Version/source pinning is defined.
- [ ] Maintenance status is documented.
- [ ] Supported algorithms are documented in `CRYPTO_REGISTRY.md`.
- [ ] Algorithm docs use `CRYPTO_ALGORITHM_TEMPLATE.md`.
- [ ] Test vectors or equivalent tests exist.
- [ ] Failure behavior is fail-closed.
- [ ] Fallback behavior is explicit.
- [ ] Non-claims are preserved.

## Non-claims

This provider entry does not make Phase1 or Base1 cryptographically complete, audited, certified, quantum-safe, hardware-validated, installer-ready, or daily-driver ready.

It documents a provider candidate for review and future policy integration only.
