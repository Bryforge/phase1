#!/usr/bin/env sh
# Phase1 security crypto documentation integrity gate.
#
# This check is read-only. It verifies the cryptographic policy roadmap,
# registry, provider registry, provider template, algorithm template, operator
# command plan, config schema, implementation plan, profile index, profile
# drafts, and security documentation links before crypto policy implementation
# work continues.

set -eu

info() {
  printf 'security-crypto-doc-integrity: %s\n' "$1"
}

fail() {
  printf 'security-crypto-doc-integrity error: %s\n' "$1" >&2
  exit 1
}

check_file() {
  [ -f "$1" ] || fail "missing required file: $1"
  [ -s "$1" ] || fail "required file is empty: $1"
  info "file ok: $1"
}

check_contains() {
  file=$1
  needle=$2
  grep -F "$needle" "$file" >/dev/null 2>&1 || fail "$file does not contain required text: $needle"
  info "reference ok: $file -> $needle"
}

check_files() {
  for file in \
    SECURITY.md \
    docs/security/SECURITY_REVIEW.md \
    README.md \
    docs/security/README.md \
    docs/security/TRUST_MODEL.md \
    docs/security/CRYPTO_POLICY_ROADMAP.md \
    docs/security/CRYPTO_REGISTRY.md \
    docs/security/CRYPTO_PROVIDER_REGISTRY.md \
    docs/security/CRYPTO_PROVIDER_TEMPLATE.md \
    docs/security/CRYPTO_ALGORITHM_TEMPLATE.md \
    docs/security/CRYPTO_OPERATOR_COMMANDS.md \
    docs/security/CRYPTO_CONFIG_SCHEMA.md \
    docs/security/CRYPTO_IMPLEMENTATION_PLAN.md \
    docs/security/crypto-profiles/README.md \
    docs/security/crypto-profiles/SAFE_DEFAULT.md \
    docs/security/crypto-profiles/HIGH_SECURITY.md \
    docs/security/crypto-profiles/COMPATIBILITY.md \
    docs/security/crypto-profiles/POST_QUANTUM_PREVIEW.md \
    docs/security/crypto-profiles/LAB_ONLY.md
  do
    check_file "$file"
  done
}

check_links_and_goals() {
  check_contains SECURITY.md 'Security goal'
  check_contains SECURITY.md 'Cryptographic policy goal'
  check_contains SECURITY.md 'docs/security/CRYPTO_POLICY_ROADMAP.md'
  check_contains README.md 'docs/security/CRYPTO_POLICY_ROADMAP.md'
  check_contains docs/security/SECURITY_REVIEW.md 'Security and usability goal'
  check_contains docs/security/TRUST_MODEL.md 'Security and usability principle'
  check_contains docs/security/README.md 'CRYPTO_POLICY_ROADMAP.md'
  check_contains docs/security/README.md 'CRYPTO_REGISTRY.md'
  check_contains docs/security/README.md 'CRYPTO_PROVIDER_REGISTRY.md'
  check_contains docs/security/README.md 'CRYPTO_PROVIDER_TEMPLATE.md'
  check_contains docs/security/README.md 'CRYPTO_OPERATOR_COMMANDS.md'
  check_contains docs/security/README.md 'CRYPTO_CONFIG_SCHEMA.md'
  check_contains docs/security/README.md 'CRYPTO_IMPLEMENTATION_PLAN.md'
  check_contains docs/security/README.md 'crypto-profiles/README.md'
  check_contains docs/security/README.md 'CRYPTO_ALGORITHM_TEMPLATE.md'
  check_contains docs/security/CRYPTO_POLICY_ROADMAP.md 'CRYPTO_REGISTRY.md'
  check_contains docs/security/CRYPTO_POLICY_ROADMAP.md 'CRYPTO_PROVIDER_REGISTRY.md'
  check_contains docs/security/CRYPTO_POLICY_ROADMAP.md 'CRYPTO_ALGORITHM_TEMPLATE.md'
  check_contains docs/security/CRYPTO_POLICY_ROADMAP.md 'CRYPTO_OPERATOR_COMMANDS.md'
  check_contains docs/security/CRYPTO_POLICY_ROADMAP.md 'CRYPTO_CONFIG_SCHEMA.md'
  check_contains docs/security/CRYPTO_POLICY_ROADMAP.md 'CRYPTO_IMPLEMENTATION_PLAN.md'
  check_contains docs/security/CRYPTO_POLICY_ROADMAP.md 'crypto-profiles/README.md'
  check_contains docs/security/CRYPTO_IMPLEMENTATION_PLAN.md 'CRYPTO_PROVIDER_REGISTRY.md'
  check_contains docs/security/CRYPTO_REGISTRY.md 'CRYPTO_ALGORITHM_TEMPLATE.md'
  check_contains docs/security/CRYPTO_PROVIDER_REGISTRY.md 'CRYPTO_PROVIDER_TEMPLATE.md'
  check_contains docs/security/crypto-profiles/README.md 'SAFE_DEFAULT.md'
  check_contains docs/security/crypto-profiles/README.md 'HIGH_SECURITY.md'
  check_contains docs/security/crypto-profiles/README.md 'COMPATIBILITY.md'
  check_contains docs/security/crypto-profiles/README.md 'POST_QUANTUM_PREVIEW.md'
  check_contains docs/security/crypto-profiles/README.md 'LAB_ONLY.md'
}

check_profiles() {
  for file in \
    docs/security/crypto-profiles/SAFE_DEFAULT.md \
    docs/security/crypto-profiles/HIGH_SECURITY.md \
    docs/security/crypto-profiles/COMPATIBILITY.md \
    docs/security/crypto-profiles/POST_QUANTUM_PREVIEW.md \
    docs/security/crypto-profiles/LAB_ONLY.md
  do
    check_contains "$file" '../CRYPTO_REGISTRY.md'
    check_contains "$file" '../CRYPTO_ALGORITHM_TEMPLATE.md'
    check_contains "$file" 'Non-claims'
    check_contains "$file" 'does not make Phase1 or Base1 cryptographically complete'
  done
}

check_operator_commands() {
  check_contains docs/security/CRYPTO_OPERATOR_COMMANDS.md 'crypto status'
  check_contains docs/security/CRYPTO_OPERATOR_COMMANDS.md 'crypto profiles'
  check_contains docs/security/CRYPTO_OPERATOR_COMMANDS.md 'crypto explain <profile-or-algorithm>'
  check_contains docs/security/CRYPTO_OPERATOR_COMMANDS.md 'crypto select <profile> --scope <control-point> --confirm'
  check_contains docs/security/CRYPTO_OPERATOR_COMMANDS.md 'crypto policy export'
  check_contains docs/security/CRYPTO_OPERATOR_COMMANDS.md 'crypto policy verify'
  check_contains docs/security/CRYPTO_OPERATOR_COMMANDS.md 'Unknown scopes should fail closed.'
  check_contains docs/security/CRYPTO_OPERATOR_COMMANDS.md 'Unknown profiles should fail closed.'
  check_contains docs/security/CRYPTO_OPERATOR_COMMANDS.md 'Lab-only selections must fail outside `lab`, `docs`, or `tests` scopes.'
  check_contains docs/security/CRYPTO_OPERATOR_COMMANDS.md 'deprecated, rejected, or lab-only entries are not used in production scopes'
}

check_config_schema() {
  check_contains docs/security/CRYPTO_CONFIG_SCHEMA.md 'Phase1 crypto policy configuration schema'
  check_contains docs/security/CRYPTO_CONFIG_SCHEMA.md 'default_profile = "safe-default"'
  check_contains docs/security/CRYPTO_CONFIG_SCHEMA.md 'Unknown profiles must fail closed.'
  check_contains docs/security/CRYPTO_CONFIG_SCHEMA.md 'Unknown scopes must fail closed.'
  check_contains docs/security/CRYPTO_CONFIG_SCHEMA.md '`lab-only` outside `lab`, `docs`, or `tests`'
  check_contains docs/security/CRYPTO_CONFIG_SCHEMA.md '`compatibility` without a reason'
  check_contains docs/security/CRYPTO_CONFIG_SCHEMA.md '`post-quantum-preview` without a reason'
  check_contains docs/security/CRYPTO_CONFIG_SCHEMA.md 'production scopes using deprecated, rejected, or lab-only registry entries'
  check_contains docs/security/CRYPTO_CONFIG_SCHEMA.md 'logs-evidence'
  check_contains docs/security/CRYPTO_CONFIG_SCHEMA.md 'fyr-packages'
}

check_implementation_plan() {
  check_contains docs/security/CRYPTO_IMPLEMENTATION_PLAN.md 'Phase1 crypto implementation plan'
  check_contains docs/security/CRYPTO_IMPLEMENTATION_PLAN.md 'Documentation and registry surface'
  check_contains docs/security/CRYPTO_IMPLEMENTATION_PLAN.md 'Read-only command surface'
  check_contains docs/security/CRYPTO_IMPLEMENTATION_PLAN.md 'Config parser and validator'
  check_contains docs/security/CRYPTO_IMPLEMENTATION_PLAN.md 'Provider abstraction'
  check_contains docs/security/CRYPTO_IMPLEMENTATION_PLAN.md 'Test-vector harness'
  check_contains docs/security/CRYPTO_IMPLEMENTATION_PLAN.md 'Profile policy engine'
  check_contains docs/security/CRYPTO_IMPLEMENTATION_PLAN.md 'Scoped integration points'
  check_contains docs/security/CRYPTO_IMPLEMENTATION_PLAN.md 'Migration and rollback tooling'
  check_contains docs/security/CRYPTO_IMPLEMENTATION_PLAN.md 'No runtime control point should use cryptographic policy until the earlier phases are complete for that scope.'
  check_contains docs/security/CRYPTO_IMPLEMENTATION_PLAN.md 'Do not implement these until their prerequisites exist'
}

check_provider_registry() {
  check_contains docs/security/CRYPTO_PROVIDER_REGISTRY.md 'Phase1 crypto provider registry'
  check_contains docs/security/CRYPTO_PROVIDER_REGISTRY.md 'CRYPTO_PROVIDER_TEMPLATE.md'
  check_contains docs/security/CRYPTO_PROVIDER_REGISTRY.md 'No provider is currently approved by this registry for new production security claims.'
  check_contains docs/security/CRYPTO_PROVIDER_REGISTRY.md 'provider name'
  check_contains docs/security/CRYPTO_PROVIDER_REGISTRY.md 'library, crate, or system API'
  check_contains docs/security/CRYPTO_PROVIDER_REGISTRY.md 'version or source pinning plan'
  check_contains docs/security/CRYPTO_PROVIDER_REGISTRY.md 'supported algorithm families'
  check_contains docs/security/CRYPTO_PROVIDER_REGISTRY.md 'test-vector source'
  check_contains docs/security/CRYPTO_PROVIDER_REGISTRY.md 'reject unknown providers'
  check_contains docs/security/CRYPTO_PROVIDER_REGISTRY.md 'avoid silently falling back to a weaker provider'
  check_contains docs/security/CRYPTO_PROVIDER_REGISTRY.md 'failure behavior is fail-closed'
}

check_provider_template() {
  check_contains docs/security/CRYPTO_PROVIDER_TEMPLATE.md 'Crypto provider documentation template'
  check_contains docs/security/CRYPTO_PROVIDER_TEMPLATE.md 'Provider summary'
  check_contains docs/security/CRYPTO_PROVIDER_TEMPLATE.md 'Source and license'
  check_contains docs/security/CRYPTO_PROVIDER_TEMPLATE.md 'Supported capabilities'
  check_contains docs/security/CRYPTO_PROVIDER_TEMPLATE.md 'Supported platforms'
  check_contains docs/security/CRYPTO_PROVIDER_TEMPLATE.md 'Profile compatibility'
  check_contains docs/security/CRYPTO_PROVIDER_TEMPLATE.md 'Control-point compatibility'
  check_contains docs/security/CRYPTO_PROVIDER_TEMPLATE.md 'Test-vector coverage'
  check_contains docs/security/CRYPTO_PROVIDER_TEMPLATE.md 'Failure behavior'
  check_contains docs/security/CRYPTO_PROVIDER_TEMPLATE.md 'Fallback behavior'
  check_contains docs/security/CRYPTO_PROVIDER_TEMPLATE.md 'Failure behavior is fail-closed.'
  check_contains docs/security/CRYPTO_PROVIDER_TEMPLATE.md 'Fallback behavior is explicit.'
}

check_guardrails() {
  check_contains docs/security/CRYPTO_POLICY_ROADMAP.md 'Phase1 should not invent new cryptographic primitives for security-critical use.'
  check_contains docs/security/CRYPTO_REGISTRY.md 'No algorithm is currently approved by this registry for new production security claims.'
  check_contains docs/security/CRYPTO_PROVIDER_REGISTRY.md 'No provider is currently approved by this registry for new production security claims.'
  check_contains docs/security/CRYPTO_ALGORITHM_TEMPLATE.md 'Does not invent a custom security-critical primitive.'
  check_contains docs/security/crypto-profiles/SAFE_DEFAULT.md 'custom security-critical primitives'
  check_contains docs/security/crypto-profiles/HIGH_SECURITY.md 'silent downgrade behavior'
  check_contains docs/security/crypto-profiles/COMPATIBILITY.md 'explicitly confirms the compatibility path'
  check_contains docs/security/crypto-profiles/POST_QUANTUM_PREVIEW.md 'unsupported claims of quantum safety'
  check_contains docs/security/crypto-profiles/LAB_ONLY.md 'must not be used as a real security profile'
}

check_non_claims() {
  for file in \
    docs/security/CRYPTO_POLICY_ROADMAP.md \
    docs/security/CRYPTO_REGISTRY.md \
    docs/security/CRYPTO_PROVIDER_REGISTRY.md \
    docs/security/CRYPTO_PROVIDER_TEMPLATE.md \
    docs/security/CRYPTO_ALGORITHM_TEMPLATE.md \
    docs/security/CRYPTO_OPERATOR_COMMANDS.md \
    docs/security/CRYPTO_CONFIG_SCHEMA.md \
    docs/security/CRYPTO_IMPLEMENTATION_PLAN.md \
    docs/security/crypto-profiles/README.md \
    docs/security/crypto-profiles/SAFE_DEFAULT.md \
    docs/security/crypto-profiles/HIGH_SECURITY.md \
    docs/security/crypto-profiles/COMPATIBILITY.md \
    docs/security/crypto-profiles/POST_QUANTUM_PREVIEW.md \
    docs/security/crypto-profiles/LAB_ONLY.md
  do
    check_contains "$file" 'audited'
    check_contains "$file" 'quantum-safe'
    check_contains "$file" 'hardware-validated'
    check_contains "$file" 'daily-driver ready'
  done
}

check_files
check_links_and_goals
check_profiles
check_operator_commands
check_config_schema
check_implementation_plan
check_provider_registry
check_provider_template
check_guardrails
check_non_claims

info 'integrity complete; crypto policy roadmap, registry, provider registry, provider template, algorithm template, operator command plan, config schema, implementation plan, profile drafts, links, guardrails, and non-claims are present'
info 'writes: no'
