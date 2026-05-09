#!/usr/bin/env python3
from __future__ import annotations

import os
import shlex
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EXCLUDED = {
    "phase1.history",
    "phase1.state",
    "phase1.log",
    "phase1.learn",
}
EXCLUDED_PREFIXES = ("target/", ".git/")


def run(args: list[str], check: bool = True) -> int:
    print("+", shlex.join(args))
    proc = subprocess.run(args, cwd=ROOT)
    if check and proc.returncode != 0:
        raise SystemExit(proc.returncode)
    return proc.returncode


def output(args: list[str]) -> str:
    return subprocess.check_output(args, cwd=ROOT, text=True).strip()


def help_text() -> str:
    return """Phase1 Dev Dock

dev status
dev sync
dev branch <name>
dev quick
dev test
dev docs
dev checkpoint <title>
dev commit <message>
dev push
dev pr <title>
dev merge <number>
dev close <number>
dev doctor
"""


def assert_repo() -> None:
    if not (ROOT / "Cargo.toml").exists() or not (ROOT / "src").exists():
        raise SystemExit("dev: not running inside the Phase1 repository")


def current_branch() -> str:
    return output(["git", "branch", "--show-current"])


def safe_changed_paths() -> list[str]:
    raw = output(["git", "status", "--porcelain"])
    paths: list[str] = []
    for line in raw.splitlines():
        if not line.strip():
            continue
        path = line[3:].strip()
        if " -> " in path:
            path = path.split(" -> ", 1)[1].strip()
        if path in EXCLUDED or path.startswith(EXCLUDED_PREFIXES):
            continue
        paths.append(path)
    return paths


def cmd_status() -> int:
    run(["git", "status", "-sb"], check=False)
    run(["gh", "pr", "list"], check=False)
    run(["cargo", "--version"], check=False)
    run(["rustc", "--version"], check=False)
    run(["go", "version"], check=False)
    return 0


def cmd_sync() -> int:
    run(["git", "checkout", "edge/stable"])
    run(["git", "pull", "--ff-only"])
    return 0


def cmd_branch(args: list[str]) -> int:
    if not args:
        print("dev branch <name>")
        return 2
    run(["git", "checkout", "-B", args[0]])
    return 0


def cmd_quick() -> int:
    run(["cargo", "fmt", "--all", "--", "--check"])
    run(["cargo", "test", "-p", "phase1", "--bin", "phase1"])
    return 0


def cmd_test() -> int:
    run(["cargo", "fmt", "--all", "--", "--check"])
    run(["cargo", "test", "--workspace", "--all-targets"])
    return 0


def cmd_commit(args: list[str]) -> int:
    message = " ".join(args).strip()
    if not message:
        print("dev commit <message>")
        return 2

    paths = safe_changed_paths()
    if not paths:
        print("dev: no safe changed files to commit")
        return 0

    run(["git", "add", *paths])
    run(["git", "commit", "-m", message])
    return 0


def cmd_push() -> int:
    branch = current_branch()
    if not branch:
        print("dev: no current branch")
        return 2
    run(["git", "push", "-u", "origin", branch])
    return 0


def cmd_pr(args: list[str]) -> int:
    title = " ".join(args).strip()
    if not title:
        print("dev pr <title>")
        return 2
    branch = current_branch()
    run([
        "gh",
        "pr",
        "create",
        "--title",
        title,
        "--body",
        f"{title}. Created from inside Phase1 Dev Dock.",
        "--base",
        "edge/stable",
        "--head",
        branch,
    ])
    return 0


def cmd_merge(args: list[str]) -> int:
    if not args:
        print("dev merge <number>")
        return 2
    run(["gh", "pr", "merge", args[0], "--squash"])
    run(["git", "checkout", "edge/stable"])
    run(["git", "pull", "--ff-only"])
    return 0


def cmd_close(args: list[str]) -> int:
    if not args:
        print("dev close <number>")
        return 2
    run(["gh", "pr", "close", args[0]])
    return 0



def cmd_docs() -> int:
    run(["python3", "scripts/update-docs.py"])
    run(["cargo", "fmt", "--all", "--", "--check"])
    return 0


def cmd_checkpoint(args: list[str]) -> int:
    title = " ".join(args).strip() or "Checkpoint edge stable"
    slug = title.lower()
    for ch in " /_:.":
        slug = slug.replace(ch, "-")
    slug = "".join(ch for ch in slug if ch.isalnum() or ch == "-").strip("-") or "edge-stable"
    branch = f"checkpoint/{slug}"

    for cmd in [
        ["git", "fetch", "origin"],
        ["git", "checkout", "edge/stable"],
        ["git", "pull", "--ff-only"],
        ["git", "checkout", "-B", branch],
        ["python3", "scripts/update-docs.py"],
        ["cargo", "fmt", "--all", "--", "--check"],
        ["cargo", "test", "--workspace", "--all-targets"],
    ]:
        code = run(cmd)
        if code != 0:
            return code

    run(["git", "add", "README.md", "REPO_DOCTRINE.md", "EDGE.md", "EDGE_STABLE_CHECKPOINT.md", "FEATURE_STATUS.md", "WIKI_ROADMAP.md", "docs/wiki", "scripts/update-docs.py"])

    changed = subprocess.run(["git", "diff", "--cached", "--quiet"], cwd=ROOT).returncode != 0
    if not changed:
        print("dev checkpoint: no changes; edge/stable is already current, no PR needed")
        return 0

    code = run(["git", "commit", "-m", title])
    if code != 0:
        return code

    code = run(["git", "push", "-u", "origin", branch])
    if code != 0:
        return code

    return run(["gh", "pr", "create", "--title", title, "--body", "Automated Phase1 checkpoint created from inside Phase1.", "--base", "edge/stable", "--head", branch])



def cmd_doctor() -> int:
    assert_repo()
    for tool in ["git", "gh", "cargo", "rustc", "python3", "go"]:
        run([tool, "--version"] if tool != "go" else ["go", "version"], check=False)
    print("dev: doctor complete")
    return 0


def main() -> int:
    assert_repo()
    args = sys.argv[1:]
    if not args or args[0] in {"help", "-h", "--help"}:
        print(help_text())
        return 0

    action, rest = args[0], args[1:]

    commands = {
        "status": lambda: cmd_status(),
        "sync": lambda: cmd_sync(),
        "branch": lambda: cmd_branch(rest),
        "quick": lambda: cmd_quick(),
        "test": lambda: cmd_test(),
        "commit": lambda: cmd_commit(rest),
        "push": lambda: cmd_push(),
        "pr": lambda: cmd_pr(rest),
        "merge": lambda: cmd_merge(rest),
        "close": lambda: cmd_close(rest),
        "docs": lambda: cmd_docs(),
        "checkpoint": lambda: cmd_checkpoint(rest),
        "doctor": lambda: cmd_doctor(),
    }

    if action not in commands:
        print(f"dev: unknown action: {action}\n")
        print(help_text())
        return 2

    return commands[action]()


if __name__ == "__main__":
    raise SystemExit(main())
