#!/usr/bin/env python3
from pathlib import Path

ROOT = Path.cwd()


def replace_once(text, old, new, label):
    if old not in text:
        raise SystemExit("missing patch anchor: " + label)
    return text.replace(old, new, 1)


main_path = ROOT / "src/main.rs"
main = main_path.read_text()

main = replace_once(
    main,
    '        Some("spec") => fyr_spec(),\n        Some("run") => fyr_run(shell, &args[1..]),',
    '        Some("spec") => fyr_spec(),\n        Some("new") => fyr_new(shell, &args[1..]),\n        Some("cat") => fyr_cat(shell, &args[1..]),\n        Some("self") => fyr_self(),\n        Some("run") => fyr_run(shell, &args[1..]),',
    "fyr command dispatch",
)

main = main.replace(
    "status    : command stub active; interpreter seed supports print literals",
    "status    : authoring loop active; interpreter seed supports print literals",
)

insert = r'''
fn fyr_new(shell: &mut Phase1Shell, args: &[String]) -> String {
    let Some(path) = args.first().and_then(|raw| fyr_file_name(raw)) else {
        return "usage: fyr new <name>\n".to_string();
    };

    if shell.kernel.sys_read(&path).is_ok() {
        return format!("fyr new: {path} already exists\n");
    }

    let source = "fn main() -> i32 { print(\"Hello, hacker!\"); return 0; }\n";
    match shell.kernel.sys_write(&path, source, false) {
        Ok(()) => format!("fyr new: created {path}\n"),
        Err(err) => format!("fyr new: {err}\n"),
    }
}

fn fyr_cat(shell: &mut Phase1Shell, args: &[String]) -> String {
    let Some(path) = args.first().and_then(|raw| fyr_file_name(raw)) else {
        return "usage: fyr cat <file.fyr>\n".to_string();
    };

    match shell.kernel.sys_read(&path) {
        Ok(mut source) => {
            if !source.ends_with('\n') {
                source.push('\n');
            }
            source
        }
        Err(err) => format!("fyr cat: {err}\n"),
    }
}

fn fyr_self() -> String {
    "fyr self\nstatus : online\nvfs    : available\nrunner : print literal seed\nnext   : lexer, parser, VFS-safe standard library\n".to_string()
}

fn fyr_file_name(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }

    let base = trimmed.strip_suffix(".fyr").unwrap_or(trimmed);
    if base
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-'))
    {
        Some(format!("{base}.fyr"))
    } else {
        None
    }
}

'''

if "fn fyr_new(shell: &mut Phase1Shell" not in main:
    main = replace_once(
        main,
        "fn fyr_run(shell: &mut Phase1Shell, args: &[String]) -> String {",
        insert + "fn fyr_run(shell: &mut Phase1Shell, args: &[String]) -> String {",
        "fyr run function",
    )

main = main.replace(
    "usage:\\n  fyr status\\n  fyr spec\\n  fyr run <file.fyr>",
    "usage:\\n  fyr status\\n  fyr spec\\n  fyr new <name>\\n  fyr cat <file.fyr>\\n  fyr self\\n  fyr run <file.fyr>",
)

main_path.write_text(main)

registry_path = ROOT / "src/registry.rs"
registry = registry_path.read_text()
registry = registry.replace(
    "fyr [status|spec|run <file.fyr>]",
    "fyr [status|spec|new|cat|self|run <file.fyr>]",
)
registry_path.write_text(registry)

spec_path = ROOT / "PHASE1_NATIVE_LANGUAGE.md"
spec = spec_path.read_text()
if "## Authoring commands" not in spec:
    spec += """

## Authoring commands

```text
fyr new hello_hacker
fyr cat hello_hacker.fyr
fyr run hello_hacker.fyr
fyr self
```

These commands let Phase1 create, inspect, and run Fyr files from inside the Phase1 shell without manually echoing source code.
"""
spec_path.write_text(spec)

roadmap_path = ROOT / "docs/fyr/ROADMAP.md"
roadmap = roadmap_path.read_text()
roadmap = roadmap.replace(
    "| F2 — Authoring loop | Make Fyr usable without manually echoing source code. | `fyr new <name>`, `fyr cat <file>`, starter templates, safer VFS writes. | Next |",
    "| F2 — Authoring loop | Make Fyr usable without manually echoing source code. | `fyr new <name>`, `fyr cat <file>`, `fyr self`, starter templates, safer VFS writes. | Active |",
)
roadmap_path.write_text(roadmap)

test_path = ROOT / "tests/fyr_authoring_commands.rs"
test_path.write_text('''use std::fs;

#[test]
fn fyr_authoring_commands_are_wired() {
    let main = fs::read_to_string("src/main.rs").expect("main source exists");
    assert!(main.contains("Some(\"new\") => fyr_new(shell, &args[1..])"));
    assert!(main.contains("Some(\"cat\") => fyr_cat(shell, &args[1..])"));
    assert!(main.contains("Some(\"self\") => fyr_self()"));
    assert!(main.contains("fn fyr_new(shell: &mut Phase1Shell"));
    assert!(main.contains("fn fyr_cat(shell: &mut Phase1Shell"));
    assert!(main.contains("fn fyr_self()"));
    assert!(main.contains("fn fyr_file_name(raw: &str)"));
    assert!(main.contains("sys_write(&path, source, false)"));

    let registry = fs::read_to_string("src/registry.rs").expect("registry source exists");
    assert!(registry.contains("fyr [status|spec|new|cat|self|run <file.fyr>]"));

    let spec = fs::read_to_string("PHASE1_NATIVE_LANGUAGE.md").expect("Fyr spec exists");
    assert!(spec.contains("fyr new hello_hacker"));
    assert!(spec.contains("fyr cat hello_hacker.fyr"));
    assert!(spec.contains("fyr self"));
}
''')

print("fyr authoring patch applied")
