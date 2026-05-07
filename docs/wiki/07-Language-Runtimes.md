# Language Runtimes

![Languages](https://img.shields.io/badge/languages-guarded%20host%20runtimes-00d8ff) ![Rust](https://img.shields.io/badge/Rust-supported-ff8a00) ![Python](https://img.shields.io/badge/Python-supported-39ff88)

Phase1 can manage and run language examples through guarded local host runtimes. Safe mode blocks these paths until the operator opts in.

## Runtime safety gate

Language runtime execution requires:

```text
SHIELD off
TRUST HOST on
```

Fast path:

```bash
./scripts/phase1-runtimes.sh
```

Manual boot path:

```text
4    SHIELD off
t    TRUST HOST on
1    BOOT
```

## Runtime commands

| Command | Purpose |
| --- | --- |
| `lang support` | Show supported language families |
| `lang security` | Show runtime safety model |
| `lang run python hello.py` | Run Python source through guarded host tools |
| `lang run rust main.rs` | Run Rust source through guarded host tools |
| `py hello.py` | Python shortcut |
| `gcc` | C compiler path when allowed |
| `plugins` | Host plugin workflow when allowed |
| `wasm list` | Show WASI-lite plugin/runtime entries |
| `wasm inspect` | Inspect WASI-lite plugin metadata |
| `wasm run` | Run a WASI-lite plugin path |

## Supported roadmap families

Phase1 is designed to support the major language families over time:

| Family | Examples |
| --- | --- |
| Systems | Rust, C, C++, Zig, Go |
| Scripting | Python, JavaScript, TypeScript, Ruby, Perl, Lua |
| JVM / VM | Java, Kotlin, Scala, C# |
| Web | HTML, CSS, JavaScript, TypeScript |
| Shell | Bash, Zsh, PowerShell |
| Data | SQL, R, Julia |
| Functional | Haskell, OCaml, F#, Elixir |
| WASM | WASI-lite plugins and sandboxed modules |

## Python tutorial

Start Phase1 with host runtimes:

```bash
./scripts/phase1-runtimes.sh
```

Inside Phase1:

> [!TIP]
> TRY THIS
>
> ```text
> avim hello.py
> ```

In AVIM:

```text
i
print("hello from phase1")
Esc
:wq
```

Run it:

```text
py hello.py
```

Alternative:

```text
lang run python hello.py
```

## Rust tutorial

Start Phase1 with host runtimes:

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

## C tutorial

Start Phase1 with host runtimes:

```bash
./scripts/phase1-runtimes.sh
```

Inside Phase1:

```text
avim hello.c
```

In AVIM:

```text
i
#include <stdio.h>

int main(void) {
    puts("hello from phase1 c");
    return 0;
}
Esc
:wq
```

Then run through the language manager when C support is available on your host:

```text
lang run c hello.c
```

## Security tutorial

> [!TIP]
> TRY THIS
>
> ```text
> security
> lang security
> capabilities
> ```

Confirm that runtime commands show host-exec capability metadata and require the correct boot gates.

## Troubleshooting language runtimes

| Symptom | Cause | Fix |
| --- | --- | --- |
| `disabled by safe boot profile` | SHIELD is still on | Use `./scripts/phase1-runtimes.sh` or press `4` at boot |
| `enable trusted host tools` | TRUST HOST is off | Use runtime launcher or press `t` at boot |
| Python not found | Host Python is missing | Install Python on the host |
| Rust not found | Host Rust toolchain is missing | Install Rust and Cargo on the host |
| C compiler not found | Host compiler is missing | Install Xcode tools, GCC, or Clang |
| Output redacted | Secret-like output detected | Remove tokens/passwords from output |

## Best practice

> [!IMPORTANT]
> Keep real credentials out of examples. Do not paste tokens, passwords, private keys, or recovery codes into Phase1 files or commands.
