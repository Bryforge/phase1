# B45 fast Mac to X200 automation

Status: active workflow plan  
Scope: Mac development machine -> GitHub -> X200 test machine -> Phase1 boot media

## Purpose

Reduce the manual loop between Mac editing, GitHub pushes, X200 pulls, USB prep, and hardware testing.

The desired loop is:

```text
Mac edits/pushes source
  -> GitHub edge/stable receives update
  -> X200 pulls latest
  -> X200 builds x86_64 Phase1
  -> X200 prepares and verifies USB
  -> only then hardware reboot/test happens
```

## Important architecture rule

The Mac can push source and docs quickly, but the final X200 bootable runtime should be built on the X200 or another x86_64 Linux builder.

Reasons:

- Apple Silicon Mac builds produce ARM binaries unless cross-compilation is explicitly configured.
- Raspberry Pi builds produce ARM binaries.
- X200 needs an x86_64 Phase1 binary.
- Libreboot/X200 USB boot media needs the x86 BIOS GRUB path installed from an x86-capable Linux builder.

## Fast workflow commands

### Mac: push a test update

```sh
cd ~/phase1_library/phase1
sh scripts/mac-b45-push-test-update.sh "describe the test update"
```

This script:

- verifies the current repo;
- warns if there are no changes;
- scans staged text for obvious private IP/token patterns;
- commits all local changes with the supplied message;
- rebases on `origin/edge/stable`;
- pushes `edge/stable`.

### X200: fetch latest and prepare test media

```sh
cd ~/phase1
sh scripts/x200-b45-fetch-latest-test.sh /dev/sdb YES_WRITE_USB
```

This script:

- fetches and pulls `edge/stable`;
- prints the latest commit;
- builds Phase1 on x86_64;
- runs B43 preflight;
- runs B45 next-test prep;
- refuses to continue unless the wrapper verifies the USB.

## Fast update from inside Phase1

The current native Phase1 booted initramfs should not silently mutate GitHub or the host. Direct update from inside Phase1 should be a future explicit command behind safe/trust gates.

Target future command:

```text
update github latest
```

Requirements before implementing inside Phase1:

- safe mode/trust gate policy;
- no embedded GitHub token;
- explicit confirmation before host/network writes;
- SSH transfer path or host-backed helper;
- evidence logging.

For now, use the X200 Linux shell script `x200-b45-fetch-latest-test.sh` after booting back into the host OS.

## Checkpointing

After a successful Mac push + X200 prepare cycle, create a checkpoint branch from the verified commit:

```sh
git fetch origin
git branch -f checkpoint/b45-fast-automation origin/edge/stable
git push -f origin checkpoint/b45-fast-automation
```

Do this only after the USB prep reports:

```text
RESULT: prepared_and_verified_for_next_test
```

## Safety notes

- Do not paste private tokens into the repository.
- Do not serve `$HOME` over HTTP while secrets such as `.ssh`, `.gitconfig`, or token files are visible.
- Prefer GitHub pull/push for source changes and a locked-down SSH transfer entry for future runtime transfers.
- Keep all claims evidence-bound.
