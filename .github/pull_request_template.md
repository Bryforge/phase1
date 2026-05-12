# Pull request

## Summary

Describe what changed and why.

-

## Project area

Check all that apply:

- [ ] Phase1
- [ ] Base1
- [ ] Fyr
- [ ] Security
- [ ] Crypto policy
- [ ] Website / branding
- [ ] Community / support
- [ ] Documentation only
- [ ] Tests / quality gates

## Validation run

Paste the commands you ran and the result.

```bash
# examples
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
sh scripts/quality-check.sh quick
```

Focused gates, when relevant:

```bash
sh scripts/quality-check.sh base1-docs
sh scripts/quality-check.sh base1-reorg
sh scripts/quality-check.sh security-crypto-docs
```

## Safety checklist

- [ ] Safe defaults are preserved.
- [ ] Host-backed behavior remains explicit and gated.
- [ ] No credentials, tokens, private keys, recovery codes, private logs, or unrevised screenshots are included.
- [ ] Documentation does not overclaim security, OS readiness, hardware validation, cryptographic completeness, audit status, certification, or quantum safety.
- [ ] Tests or docs were updated for changed behavior.

## Compatibility checklist

- [ ] No compatibility paths were removed.
- [ ] Base1 release archive paths are preserved when relevant.
- [ ] Script/operator command paths remain stable or wrappers are documented.
- [ ] Migration or rollback guidance is documented when relevant.

## Crypto-specific checklist

Complete this section for crypto policy or implementation work.

- [ ] No custom security-critical primitive is added for real protection.
- [ ] Algorithm entries use `docs/security/CRYPTO_ALGORITHM_TEMPLATE.md`.
- [ ] Provider entries use `docs/security/CRYPTO_PROVIDER_TEMPLATE.md`.
- [ ] Unknown profiles, scopes, algorithms, and providers fail closed.
- [ ] Lab-only behavior is isolated from production control points.
- [ ] Non-claims are preserved.

## Risks and follow-ups

List known limitations, risks, or follow-up work.

-

## Related docs

- `CONTRIBUTING.md`
- `QUALITY.md`
- `SECURITY.md`
