#!/usr/bin/env sh
# Base1 B2 dry-run assembly preview.
#
# B2 boot-readiness slice: preview an assembly flow without writing images,
# boot entries, firmware settings, partitions, packages, initramfs files, or
# kernel command lines.

set -eu

usage() {
  cat <<'EOF'
usage: sh scripts/base1-b2-assembly-dry-run.sh --dry-run --profile <profile>

Read-only Base1 B2 dry-run assembly preview.

Required:
  --dry-run             prove this command is non-mutating
  --profile <profile>  one of:
                          x86_64-uefi-generic
                          x86_64-bios-generic
                          x86_64-libreboot-grub
                          x86_64-vm-validation
                          x86_64-recovery-usb

This script previews assembly status only and writes nothing.
EOF
}

fail() {
  printf 'base1-b2-assembly-dry-run error: %s\n' "$1" >&2
  exit 1
}

have() {
  command -v "$1" >/dev/null 2>&1
}

valid_profile() {
  case "$1" in
    x86_64-uefi-generic|x86_64-bios-generic|x86_64-libreboot-grub|x86_64-vm-validation|x86_64-recovery-usb)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

read_machine() {
  if have uname; then
    uname -m 2>/dev/null || printf 'unknown'
  elif have arch; then
    arch 2>/dev/null || printf 'unknown'
  else
    printf 'unknown'
  fi
}

DRY_RUN=no
PROFILE=""

while [ "$#" -gt 0 ]; do
  case "$1" in
    --dry-run)
      DRY_RUN=yes
      shift
      ;;
    --profile)
      shift
      [ "$#" -gt 0 ] || fail "--profile requires a value"
      PROFILE=$1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      usage >&2
      fail "unknown argument: $1"
      ;;
  esac
done

[ "$DRY_RUN" = "yes" ] || {
  usage >&2
  fail "--dry-run is required for B2 assembly preview"
}

[ -n "$PROFILE" ] || {
  usage >&2
  fail "--profile is required for B2 assembly preview"
}

valid_profile "$PROFILE" || fail "unsupported profile: $PROFILE"

MACHINE=$(read_machine)

printf 'base1_b2_assembly_dry_run: start\n'
printf 'status: B2 dry-run assembly preview\n'
printf 'boot_readiness_level: B2\n'
printf 'writes: no\n'
printf 'mutation: no\n'
printf 'network: no\n'
printf 'profile: %s\n' "$PROFILE"
printf '\n'

printf 'b1_detection_summary:\n'
printf '  detector: scripts/base1-x86_64-detect.sh --dry-run\n'
printf '  machine_hint: %s\n' "$MACHINE"
case "$MACHINE" in
  x86_64|amd64)
    printf '  x86_64_hint: yes\n'
    ;;
  unknown|'')
    printf '  x86_64_hint: unknown\n'
    ;;
  *)
    printf '  x86_64_hint: no\n'
    ;;
esac
if [ -d /sys/firmware/efi ]; then
  printf '  firmware_hint: uefi\n'
else
  printf '  firmware_hint: bios_or_unknown\n'
fi
printf '\n'

printf 'profile_assumptions:\n'
printf '  selected_profile: %s\n' "$PROFILE"
case "$PROFILE" in
  x86_64-vm-validation)
    printf '  target_class: virtual_machine_validation\n'
    printf '  claim_boundary: vm_preview_only\n'
    ;;
  x86_64-uefi-generic)
    printf '  target_class: generic_uefi_x86_64\n'
    printf '  claim_boundary: planning_only_without_validation_report\n'
    ;;
  x86_64-bios-generic)
    printf '  target_class: generic_legacy_bios_x86_64\n'
    printf '  claim_boundary: planning_only_without_validation_report\n'
    ;;
  x86_64-libreboot-grub)
    printf '  target_class: libreboot_grub_x86_64\n'
    printf '  claim_boundary: planning_only_without_validation_report\n'
    ;;
  x86_64-recovery-usb)
    printf '  target_class: recovery_usb_x86_64\n'
    printf '  claim_boundary: planning_only_without_validation_report\n'
    ;;
esac
printf '\n'

printf 'image_builder_preview:\n'
printf '  design_doc: docs/os/BASE1_IMAGE_BUILDER.md\n'
printf '  image_write: no\n'
printf '  required_inputs: base image recipe, Phase1 payload path, profile metadata, validation bundle path\n'
printf '  missing_inputs: implementation-specific image recipe and signed/verifiable metadata\n'
printf '\n'

printf 'boot_handoff_preview:\n'
printf '  kernel_handoff: preview_only\n'
printf '  initramfs_handoff: preview_only\n'
printf '  phase1_autostart: preview_only\n'
printf '  emergency_shell_fallback: required_before_claims_strengthen\n'
printf '\n'

printf 'installer_preview:\n'
printf '  design_doc: docs/os/INSTALLER_RECOVERY.md\n'
printf '  install_write: no\n'
printf '  partition_write: no\n'
printf '  boot_loader_write: no\n'
printf '  status: dry_run_preview_only\n'
printf '\n'

printf 'recovery_preview:\n'
printf '  recovery_path: preview_only\n'
printf '  recovery_media: unknown_until_validation\n'
printf '  emergency_shell: required\n'
printf '  status: not_validated\n'
printf '\n'

printf 'rollback_preview:\n'
printf '  design_doc: docs/os/BASE1_ROLLBACK_METADATA.md\n'
printf '  rollback_metadata_write: no\n'
printf '  rollback_available: not_claimed\n'
printf '  status: preview_only\n'
printf '\n'

printf 'validation_bundle:\n'
printf '  planned_path: docs/os/validation/b2-%s.md\n' "$PROFILE"
printf '  includes: detection summary, profile assumptions, image preview, boot handoff preview, recovery preview, rollback preview\n'
printf '  status: planned\n'
printf '\n'

printf 'known_limitations:\n'
printf '  bootable: not_claimed\n'
printf '  installer_ready: not_claimed\n'
printf '  recovery_complete: not_claimed\n'
printf '  hardware_validated: not_claimed\n'
printf '  hardened: not_claimed\n'
printf '  release_candidate: not_claimed\n'
printf '\n'

printf 'next_validation_step: create B2 limitations note and validation report; then run B2 script tests when implemented\n'
printf 'base1_b2_assembly_dry_run: complete\n'
