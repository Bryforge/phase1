# Tutorials

![Tutorials](https://img.shields.io/badge/tutorials-TRY%20THIS-00d8ff) ![Stable](https://img.shields.io/badge/stable-v4.0.0-39ff88) ![Previous Stable](https://img.shields.io/badge/previous%20stable-v3.10.9-7f8cff)

These tutorials are designed for a new Phase1 user. Run them in order or jump to the workflow you need.

## Tutorial 1: First boot and orientation

Start Phase1:

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
> version
> version --compare
> cat readme.txt
> security
> sysinfo
> ```

You should understand:

- what version is running
- whether SHIELD is on
- whether TRUST HOST is on
- where the built-in quick start lives

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

## Tutorial 3: Use AVIM to write Python

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

## Tutorial 4: Use AVIM to write Rust

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

## Tutorial 5: Inspect browser output

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

## Tutorial 6: Inspect network state

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

## Tutorial 7: Use dashboard and audit tools

> [!TIP]
> TRY THIS
>
> ```text
> dash
> dash --compact
> audit
> opslog status
> opslog tail
> ```

You should see:

- active Phase1 version
- mode and channel
- VFS mount summary
- network safety summary
- latest audit event
- local operations log status

## Tutorial 8: Simulate process and hardware commands

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

## Tutorial 9: Enable persistence

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

## Tutorial 10: Run the full validation loop

From the host:

```bash
git pull origin master
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo audit
cargo deny check
cargo run
```

Expected result:

```text
format passes
compile passes
clippy passes
tests pass
audit passes
dependency policy passes
Phase1 boots
shutdown reports the current package version
```

## Tutorial 11: Prepare a stable release

From the host:

```bash
git status
git log -1 --oneline
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo audit
cargo deny check
```

For a stable build, remove `-dev`, update docs and in-system wiki fixtures, validate again, commit, tag, and push.

Current stable target:

```bash
git tag v4.0.0
git push origin v4.0.0
```

## Tutorial 12: Publish manual pages to the native wiki

Only do this after GitHub Wiki support exists for the repository.

```bash
cd ..
git clone https://github.com/Bryforge/phase1.wiki.git phase1.wiki
rsync -av --delete phase1/docs/wiki/ phase1.wiki/
cd phase1.wiki
git add .
git commit -m "Update Phase1 user manual for v4.0.0"
git push origin master
```

If the clone fails with `Repository not found`, enable Wiki support in the repository settings or create the first wiki page through the GitHub web UI.
