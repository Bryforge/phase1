# B40 native binary loader fix plan

Status: planning scaffold

Scope: X200 / B38 working protocol / B39 native Phase1 binary launch fix.

## Purpose

B39 reached the Phase1-owned runtime and attempted to execute `/phase1/bin/phase1`, but the shell reported:

`/phase1/bin/phase1: not found`

Because B39 copied the native binary into the initramfs, this usually means the ELF program interpreter or shared dynamic libraries were missing, not that the binary file itself was absent.

On glibc systems, the interpreter is commonly:

`/lib64/ld-linux-x86-64.so.2`

If that file is a symlink and only the symlink is copied, it can point to a missing target inside the initramfs and produce `not found`.

## B40 fix

B40 keeps the B38 working boot protocol and B39 native launch but improves packaging:

- copy Phase1 binary;
- copy ldd-discovered shared libraries;
- dereference symlinks with `cp -L`;
- explicitly copy the ELF interpreter when discoverable;
- explicitly copy common glibc loader paths;
- add a loader diagnostic before executing Phase1;
- keep a fallback shell.

## Preferred future fix

A static or musl-linked Phase1 binary should eventually replace glibc dynamic packaging for initramfs booting.

## Success states

`phase1_native_binary_started`

The Phase1 binary was actually executed.

`phase1_native_console_seen`

The designed Phase1 console appeared.

Fallback:

`phase1_full_system_load_seen`

The B38/B40 fallback shell remains usable even if native launch fails.
