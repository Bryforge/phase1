# Tutorials

![Tutorials](https://img.shields.io/badge/tutorials-TRY%20THIS-00d8ff) ![Edge](https://img.shields.io/badge/edge-v6.0.0-00d8ff) ![Stable](https://img.shields.io/badge/stable-v5.0.0-39ff88) ![Safe Defaults](https://img.shields.io/badge/safe%20defaults-on-39ff88)

These tutorials are designed for a new Phase1 user. Run them in order or jump to the workflow you need.

## Tutorial 1: First boot and orientation

Start Phase1:

```bash
sh phase1
```

Alternative Rust-native start:

```bash
cargo run
```

At the boot selector:

```text
1
```

> [!TIP]
> TRY THIS
>
> ```text
> help
> help ui
> help flows
> version
> version --compare
> cat readme.txt
> wiki
> wiki-quick
> security
> capabilities
> sysinfo
> ```

You should understand:

- what version is running
- whether safe mode is active
- whether SHIELD is on
- whether TRUST HOST is on
- where the built-in quick start lives
- which commands require host access

## Tutorial 2: Create a small virtual filesystem lab

> [!TIP]
> TRY THIS
>
> ```text
> mkdir lab
> cd lab
> echo Phase1 VFS Tutorial > README.txt
> echo alpha > data.txt
> echo beta >> data.txt
> echo alpha >> data.txt
> ls
> cat README.txt
> cat data.txt
> ```

Now inspect it:

```text
pwd
tree /home
cat data.txt | sort | uniq
cat data.txt | grep alpha | wc -l
```

Expected ideas:

- `README.txt` and `data.txt` exist inside the Phase1 VFS
- pipeline filters can process VFS content
- `/home` is the normal workspace

## Tutorial 3: Write and run Fyr

Create a simple Fyr script:

```text
echo 'fn main() -> i32 { print("hello from Fyr"); return 0; }' > hello.fyr
fyr run hello.fyr
```

Expected output:

```text
hello from Fyr
```

Use AVIM for a slightly larger script:

```text
avim math.fyr
```

In AVIM:

```text
i
fn main() -> i32 {
    let answer = 40 + 2;
    print(answer);
    return 0;
}
Esc
:wq
```

Run it:

```text
fyr run math.fyr
```

Expected output:

```text
42
```

## Tutorial 4: Use AVIM to write Python

Start Phase1 with host runtimes enabled:

```bash
./scripts/phase1-runtimes.sh
```

Inside Phase1:

```text
avim hello.py
```

In AVIM:

```text
i
print("hello from phase1")
Esc
:wq
```

Run the file:

```text
py hello.py
```

Alternative run path:

```text
lang run python hello.py
```

> [!IMPORTANT]
> Python execution is host-backed. Use it only after you understand SHIELD, TRUST HOST, and safe mode.

## Tutorial 5: Use AVIM to write Rust

Start runtime mode:

```bash
./scripts/phase1-runtimes.sh
```

Inside Phase1:

```text
avim main.rs
```

In AVIM:

```text
i
fn main() {
    println!("hello from phase1 rust");
}
Esc
:wq
```

Run it:

```text
lang run rust main.rs
```

## Tutorial 6: Inspect browser output

Start runtime mode:

```bash
./scripts/phase1-runtimes.sh
```

Inside Phase1:

```text
browser about
browser phase1
browser example.com
```

Then try the project page:

```text
browser https://github.com/Bryforge/phase1
```

Look for:

- page status
- final URL
- page title
- readable extracted text
- indexed link list

## Tutorial 7: Inspect network state

Start runtime mode:

```bash
./scripts/phase1-runtimes.sh
```

Inside Phase1:

```text
security
ifconfig
nmcli
iwconfig
ping example.com
```

Look for:

- host tool permission state
- interface names
- IPv4 addresses
- WiFi summary where supported
- safe denial messages when a command is intentionally blocked

## Tutorial 8: Use dashboard, audit, and nested context tools

> [!TIP]
> TRY THIS
>
> ```text
> dash
> dash --compact
> audit
> opslog status
> opslog tail
> nest status
> nest spawn lab
> nest list
> nest tree
> nest inspect lab
> nest destroy lab
> ```

You should see:

- active Phase1 version
- mode and channel
- VFS mount summary
- network safety summary
- latest audit event
- local operations log status
- nested metadata contexts and topology

## Tutorial 9: Simulate process and hardware commands

> [!TIP]
> TRY THIS
>
> ```text
> ps
> spawn worker --background
> jobs
> ps
> kill 2
> cr3
> loadcr3 0x2000
> cr4
> pcide on
> lspci
> pcie
> ```

This tutorial is educational. The process and hardware commands are simulated and audited.

## Tutorial 10: Enable persistence

At boot selector:

```text
p    VAULT on
1    BOOT
```

Inside Phase1:

```text
echo saved file > saved.txt
ls
exit
```

Start Phase1 again with VAULT on and verify:

```text
ls
cat saved.txt
```

Persistence files on the host:

```text
phase1.state
phase1.history
```

> [!CAUTION]
> Do not store secrets in persistent VFS files.

## Tutorial 11: Run Base1 read-only and dry-run checks

Base1 checks should stay read-only or dry-run until implementation, review, and validation support stronger action.

```bash
sh scripts/base1-x86_64-detect.sh --dry-run
sh scripts/base1-b2-assembly-dry-run.sh --dry-run --profile x86_64-vm-validation
sh scripts/base1-preflight.sh
sh scripts/base1-install-dry-run.sh --dry-run --target /dev/example
sh scripts/base1-recovery-dry-run.sh --dry-run
```

Expected idea:

```text
no host disk writes are required for these checks
```

## Tutorial 12: Run the full validation loop

From the host:

```bash
git fetch origin
git status
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
sh scripts/quality-check.sh quick
cargo run
```

Expected result:

```text
format passes
compile passes
clippy passes
tests pass
quality gate passes
Phase1 boots
shutdown reports the current package version
```

## Tutorial 13: Prepare release-facing documentation

From the host:

```bash
git status
git log -1 --oneline
cargo metadata --no-deps --format-version 1 | grep '"version"'
sh scripts/quality-check.sh quick
```

Then check that these surfaces agree:

```text
README.md
docs/wiki/Home.md
docs/wiki/02-Version-Guide.md
docs/wiki/08-Updates-Releases-and-Validation.md
site/site.js
plugins/wiki-version.wasi
plugins/wiki-updates.wasi
```

For stable release-facing docs, avoid edge-only claims. For Base1 docs, keep claims tied to evidence. For Fyr docs, only use examples supported by current language behavior.

## Tutorial 14: Publish manual pages to the native wiki

Only do this after GitHub Wiki support exists for the repository and the source docs have been reviewed.

```bash
scripts/publish-wiki.sh
```

Manual equivalent:

```bash
cd ..
git clone https://github.com/Bryforge/phase1.wiki.git phase1.wiki
rsync -av --delete phase1/docs/wiki/ phase1.wiki/
cd phase1.wiki
git add .
git commit -m "Update Phase1 user manual"
git push origin master
```

If the clone fails with `Repository not found`, enable Wiki support in the repository settings or create the first wiki page through the GitHub web UI.
