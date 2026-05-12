# B47 Mac build logic reconsideration

Status: black-phase1 architecture correction  
Scope: Mac development, X200 final media, QEMU framebuffer renderer lab

## Problem

A lot of Phase1 source and automation work has been produced from the Mac, but the target machine is the X200 running an x86_64 Linux boot path.

That means we must separate three jobs:

```text
Mac        -> source orchestration, GitHub pushes, documentation, design, optional cross-build lab
X200       -> final x86_64 Linux binary build, USB boot media preparation, hardware test
QEMU lab   -> fast renderer testing before X200 reboot cycles
```

## Important correction

A binary built normally on an Apple Silicon Mac is not the correct final binary for the X200.

The X200 final runtime expects:

```text
ELF 64-bit LSB executable, x86-64, Linux
```

A native Apple Silicon Mac build normally produces:

```text
Mach-O arm64 macOS binary
```

A Raspberry Pi build normally produces:

```text
ELF aarch64 Linux binary
```

Neither is valid as the final `/phase1/bin/phase1` binary for the X200.

## New rule

Do not trust a build until `file target/release/phase1` confirms the target.

For X200 boot media, required:

```text
target/release/phase1: ELF 64-bit ... x86-64 ... Linux
```

If not, stop before USB writing.

## Recommended workflow

### Mac

Use the Mac for fast source flow:

```sh
git checkout black-phase1
# edit
sh scripts/black-phase1.sh push "Describe rapid test"
```

### X200

Use the X200 for final media:

```sh
git checkout black-phase1
git pull --ff-only origin black-phase1
sh scripts/black-phase1.sh cycle /dev/sdb YES_WRITE_USB b47-test
```

### QEMU renderer lab

For QEMU renderer testing, use the machine that has the correct Linux kernel/initrd assets and a CJK font. Prefer the X200 Linux host or another x86_64 Linux box for now. Mac QEMU can be used later, but only after we define a reproducible Linux x86_64 build/cross-build path.

## Mac future options

Mac can become a full build orchestrator if we add one of these:

1. Linux x86_64 VM builder;
2. Docker/Podman Linux x86_64 builder;
3. Rust cross toolchain + Linux linker/sysroot;
4. CI artifact builder that publishes x86_64 Linux Phase1 binaries;
5. remote X200 builder invoked over SSH.

The cleanest long-term path is likely:

```text
Mac pushes -> GitHub Actions or X200 SSH builder builds x86_64 -> X200 pulls verified artifact -> USB prep
```

## B47 consequence

The framebuffer renderer lab should avoid depending on a native Mac binary. It should either:

- run on the X200 Linux host;
- run on an x86_64 Linux VM/container;
- or use a clearly verified cross-compiled x86_64 Linux artifact.

## Evidence fields to add/check

```text
BASE1_BUILD_HOST=<mac|x200|pi|linux-vm|ci>
BASE1_BUILD_HOST_ARCH=<uname -m>
BASE1_PHASE1_BINARY_FILE=<file output>
BASE1_PHASE1_BINARY_X200_COMPAT=yes|no
BASE1_MEDIA_BUILDER_X200_COMPAT=yes|no
```

## Updated stance

Mac is the command center. X200/Linux is the truth builder until we have a reproducible cross-build pipeline.
