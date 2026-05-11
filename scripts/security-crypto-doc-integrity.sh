#!/usr/bin/env sh
# Phase1 security crypto documentation integrity gate.
#
# This check is read-only. It verifies the cryptographic policy roadmap,
# registry, algorithm template, operator command plan, profile index, profile
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
    SECURITY_REVIEW.md \
    README.md \
    docs/security/README.md \
    docs/security/TRUST_MODEL.md \
    docs/security/CRYPTO_POLICY_ROADMAP.md \
    docs/security/CRYPTO_REGISTRY.md \
    docs/security/CRYPTO_ALGORITHM_TEMPLATE.md \
    docs/security/CRYPTO_OPERATOR_COMMANDS.md \
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
  check_contains SECURITY_REVIEW.md 'Security and usability goal'
  check_contains docs/security/TRUST_MODEL.md 'Security and usability principle'
  check_contains docs/security/README.md 'CRYPTO_POLICY_ROADMAP.md'
  check_contains docs/security/README.md 'CRYPTO_REGISTRY.md'
  check_contains docs/security/README.md 'CRYPTO_OPERATOR_COMMANDS.md'
  check_contains docs/security/README.md 'crypto-profiles/README.md'
  check_contains docs/security/README.md 'CRYPTO_ALGORITHM_TEMPLATE.md'
  check_contains docs/security/CRYPTO_POLICY_ROADMAP.md 'CRYPTO_REGISTRY.md'
  check_contains docs/security/CRYPTO_POLICY_ROADMAP.md 'CRYPTO_ALGORITHM_TEMPLATE.md'
  check_contains docs/security/CRYPTO_POLICY_ROADMAP.md 'CRYPTO_OPERATOR_COMMANDS.md'
  check_contains docs/security/CRYPTO_POLICY_ROADMAP.md 'crypto-profiles/README.md'
  check_contains docs/security/CRYPTO_REGISTRY.md 'CRYPTO_ALGORITHM_TEMPLATE.md'
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

check_guardrails() {
  check_contains docs/security/CRYPTO_POLICY_ROADMAP.md 'Phase1 should not invent new cryptographic primitives for security-critical use.'
  check_contains docs/security/CRYPTO_REGISTRY.md 'No algorithm is currently approved by this registry for new production security claims.'
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
    docs/security/CRYPTO_ALGORITHM_TEMPLATE.md \
    docs/security/CRYPTO_OPERATOR_COMMANDS.md \
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
check_guardrails
check_non_claims

info 'integrity complete; crypto policy roadmap, registry, algorithm template, operator command plan, profile drafts, links, guardrails, and non-claims are present'
info 'writes: no'
