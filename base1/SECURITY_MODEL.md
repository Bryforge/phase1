# Base1 Security Model

Base1 is the secure hardware host layer for Phase1. Its purpose is to make Phase1 useful on real hardware without making the host disposable, fragile, or easy to compromise.

## Security objective

Base1 should preserve host integrity when Phase1 fails.

A Phase1 failure includes:

- Accidental deletion of Phase1 workspace files.
- Corruption of Phase1 runtime state.
- A malicious command typed into Phase1.
- A bug in Phase1 runtime integration.
- A hostile project cloned into the Phase1 workspace.
- A compromised language runtime launched from Phase1.
- A crash or denial-of-service attempt inside the Phase1 layer.

Base1 should keep boot, recovery, logs, host secrets, and update authority outside Phase1 control.

## Non-goals for the first foundation

- Do not claim formal verification.
- Do not claim OpenBSD-equivalent maturity.
- Do not bypass the operator's existing OS security model.
- Do not build a destructive installer before preflight and dry-run tooling exists.
- Do not grant Phase1 direct root privileges.

## Threat model

### In scope

- Host command abuse from Phase1.
- Accidental host file deletion.
- Phase1 workspace compromise.
- Untrusted cloned repositories.
- Language runtime escape attempts.
- Network exposure from operator tools.
- Supply-chain drift in Phase1 builds.
- Log destruction inside Phase1.
- Misconfiguration on Raspberry Pi and X200 target hardware.

### Out of scope for foundation stage

- Physical attacks by a skilled adversary with unrestricted device access.
- CPU microcode or silicon attacks.
- Malicious firmware that already controls the machine.
- Nation-state level implants.
- Fully verified kernel correctness.

These may become future hardening topics, but Base1 must be honest about what it currently protects.

## Core isolation model

Base1 treats Phase1 as a contained workload.

Base1 should run Phase1 as:

- A dedicated unprivileged user.
- A process with a private runtime directory.
- A process with a bounded writable workspace.
- A process denied write access to host boot, package, service, key, and recovery paths.
- A process with network access disabled or restricted unless the operator chooses a network profile.

## Security rings

### Ring B0: firmware and boot

Owns hardware initialization, bootloader, kernel handoff, and recovery boot path.

Required direction:

- Document firmware state for each hardware target.
- Prefer libreboot/coreboot-friendly flows on X200-class hardware.
- Prefer signed or checksummed image workflows for Raspberry Pi.
- Keep recovery media independent of Phase1.

### Ring B1: Base1 host

Owns the installed OS, kernel, init, users, firewall, storage, logs, and update trust.

Required direction:

- Minimal package set.
- Default-deny inbound networking.
- Host logs outside Phase1 workspace.
- No host mutation from Phase1 except through explicit maintenance tools.
- Reproducible build and install manifests.

### Ring B2: Phase1 runtime account

Owns the Phase1 process and runtime account.

Required direction:

- Dedicated `phase1` user.
- No shell escalation by default.
- No passwordless sudo.
- No host package manager access.
- Controlled workspace path.

### Ring B3: Phase1 workspace

Owns user projects, clones, temporary build output, and VFS persistence.

Required direction:

- Clearly bounded directory.
- Easy reset and rebuild.
- No stored host secrets.
- Backups optional and explicit.

## Default-deny host mutation

Phase1 must not be able to directly change:

- `/boot`
- `/etc`
- `/usr`
- `/bin`
- `/sbin`
- `/lib`
- package manager databases
- service definitions
- firewall policy
- SSH host keys
- Base1 signing keys
- recovery partitions or media

When host mutation is needed, Base1 should require:

1. Maintenance mode.
2. Clear prompt.
3. Dry-run plan.
4. Explicit operator approval.
5. Log entry outside the Phase1 workspace.

## Filesystem policy

Recommended initial layout:

```text
/base1                 Base1 manifests and local policy
/opt/phase1            Phase1 executable and read-only assets
/var/lib/phase1        Phase1 persistent workspace
/var/log/base1         Base1 host logs
/var/log/phase1-host   Host-side Phase1 supervision logs
/run/phase1            Runtime files, deleted at reboot
```

Optional hardening profiles:

- Read-only root with separate writable `/var`.
- Snapshot/rollback capable root.
- Separate Phase1 data partition.
- External recovery image.
- Offline-only appliance mode.

## Network policy

Default network posture:

- Deny inbound connections.
- Permit outbound package/update traffic only in maintenance or update mode.
- Permit Phase1 network features only through an explicit Base1 network profile.
- Keep SSH disabled by default on single-user appliance deployments.
- If SSH is enabled, require key-only login and rate limiting.

## Update policy

Base1 updates and Phase1 updates are separate.

Base1 owns:

- OS updates.
- Kernel updates.
- Bootloader updates.
- Firewall and service updates.
- Recovery updates.

Phase1 owns:

- Phase1 application version.
- Phase1 docs/wiki assets.
- Phase1 test fixtures.

Base1 should be able to reinstall Phase1 from a known-good release without trusting a damaged Phase1 workspace.

## Logging and audit

Base1 logs must survive Phase1 destruction.

Minimum host logs:

- Phase1 start/stop events.
- Phase1 update attempts.
- Maintenance mode entry and exit.
- Host mutation dry-run and apply summaries.
- Network profile changes.
- Failed privilege attempts.

Logs should avoid storing secrets.

## Secret handling

Base1 and Phase1 should never request or store:

- GitHub passwords.
- Personal access tokens in repository files.
- SSH private keys in Phase1 workspace.
- Browser cookies.
- Apple ID or email passwords.
- Recovery codes.

If a credential is needed for an operator workflow, it must be stored outside Phase1 using host-controlled secret storage or provided interactively for the single operation.

## Compatibility contract

Base1 should expose a small stable interface to Phase1:

- `BASE1_PROFILE`
- `BASE1_HARDWARE_TARGET`
- `PHASE1_STORAGE_ROOT`
- `PHASE1_SAFE_MODE`
- `PHASE1_ALLOW_HOST_TOOLS`

The default should keep `PHASE1_SAFE_MODE=1` and `PHASE1_ALLOW_HOST_TOOLS=0`.

## Security quality bar

Base1 development must require:

- Secure defaults.
- Tests for security-critical scripts.
- Shell scripts using strict mode.
- No silent destructive behavior.
- No secrets in repository files.
- Clear rollback and recovery plan.
- Minimal dependencies.
- Repeatable builds.
- Documentation for every privilege boundary.

## OpenBSD-inspired discipline

Base1 should borrow principles from hardened operating systems:

- Small pieces.
- Clear privilege separation.
- Defense in depth.
- Conservative defaults.
- Code readability over cleverness.
- Man-page-level documentation.
- Disable what is not needed.
- Assume mistakes happen and contain them.

The project should not claim OpenBSD parity until it has years of audit, releases, exploit history, and operational maturity.
