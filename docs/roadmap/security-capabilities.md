# R3 security and capabilities design

## Purpose

phase1 should expose a realistic security model without pretending to be a real security boundary for the host machine.

The simulator should model users, permissions, and capabilities internally, while host integrations remain guarded by allow flags, validation, and timeouts.

## Capability labels

Command metadata already includes a capability string. These labels should become policy inputs.

Initial labels:

```text
none
fs.read
fs.write
proc.read
proc.spawn
proc.manage
proc.kill
net.read
net.admin
host.net
host.exec
hw.read
hw.write
sys.read
sys.log
sys.audit
user.read
user.env
user.switch
```

## Policy model

Policy decision input:

```text
subject.user
subject.uid
command.name
command.capability
object.path_or_resource
context.interactive
context.host_mutation_allowed
```

Policy output:

```text
allow | deny
reason
```

Example decisions:

```text
root + fs.write       -> allow
user + fs.write /tmp  -> allow
user + hw.write       -> deny
any  + net.read       -> allow
any  + net.admin      -> deny unless explicitly enabled
any  + host.exec      -> allow only when command has guardrails
```

## Commands

Add:

```text
capabilities
policy
policy explain <command>
```

Example output:

```text
command       capability   status
wifi-connect  net.admin    guarded
python        host.exec    timeout
loadcr3       hw.write     root-only
```

## Host safety rules

Host mutation should be opt-in.

Examples:

```text
PHASE1_ALLOW_HOST_NETWORK_CHANGES=1
PHASE1_ALLOW_HOST_EXEC=1  # future stricter mode
```

Current host tools should remain bounded:

- curl timeout and URL scheme validation
- ping timeout and host validation
- compiler timeout
- python timeout
- Wi-Fi mutation dry-run by default

## Audit integration

Every policy decision should produce an audit event:

```text
policy.allow command=cat capability=fs.read user=root
policy.deny command=loadcr3 capability=hw.write user=user reason=root-only
```

## Implementation checklist

1. Add `policy.rs` with `PolicyDecision` and `check_command`.
2. Add `capabilities` command.
3. Add `policy explain <command>`.
4. Run policy checks before command execution.
5. Record policy events in audit.
6. Add tests for allowed and denied operations.

## Important note

This is an educational simulator policy model. It is not a host OS sandbox. Host safety still depends on careful command validation and using Rust process APIs safely.
