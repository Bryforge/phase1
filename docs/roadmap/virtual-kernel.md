# R2 virtual kernel design

## Purpose

The virtual kernel should be the architectural boundary between shell commands and simulated OS state.

Commands should not directly mutate major kernel state once the syscall layer exists. They should request operations through kernel APIs that can validate, audit, and test behavior.

## Kernel domains

### VFS

Responsibilities:

- path resolution
- file and directory mutation
- mount routing
- permissions later
- procfs/devfs/logfs generation

Current target mounts:

```text
/          rootfs
/home      user workspace
/proc      generated process/kernel state
/dev       generated device nodes
/tmp       scratch space
/var/log   generated and writable logs
/etc       config files
```

### Scheduler

Responsibilities:

- process table
- process lifecycle
- job foreground/background state
- simulated priority
- simulated CPU ticks
- signal/kill behavior

Future process states:

```text
ready
running
sleeping
stopped
zombie
terminated
```

### Syscalls

Initial syscall surface:

```text
sys_read(path)
sys_write(path, data, append)
sys_spawn(name, cmdline, background)
sys_kill(pid)
sys_stat(path)
sys_list(path)
sys_chdir(path)
sys_mount(source, target, kind)
sys_unmount(target)
```

Every syscall should record an audit event.

### Audit

Audit event shape:

```text
time=<uptime_ms> user=<user> action=<syscall> object=<path|pid|resource> result=<allow|deny|error>
```

Example:

```text
0004 user=root action=sys.write object=/home/demo.txt result=allow
```

## Design rules

- Shell commands should become thin wrappers.
- Kernel APIs should return typed results where possible.
- Error messages should be deterministic enough for tests.
- Generated files under `/proc` should reflect current kernel state.
- Host commands should never bypass audit.

## Implementation checklist

1. Move `cd`, `ls`, `mkdir`, `touch`, `rm`, `cp`, and `mv` through syscall-style methods.
2. Add typed syscall result wrappers.
3. Expand `/proc` to include process details.
4. Add `/var/log/audit.log` mirror from the audit buffer.
5. Add mount table data structure.
6. Add smoke tests for procfs changing after `spawn` and `kill`.

## Demo commands

```text
cat /proc/version
spawn worker --background
ps
cat /proc/uptime
audit
```
