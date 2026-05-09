#!/usr/bin/env python3
from __future__ import annotations

import os
import re
import shlex
import subprocess
import sys
from pathlib import Path

ROOT = Path(os.environ.get("PHASE1_STORAGE_ROOT", "phase1.workspace"))
REPOS = ROOT / "repos"
TIMEOUT_SECONDS = 20
MAX_OUTPUTO = 24000

def parse_context() -> dict[str, str]:
    data: dict[str, str] = {}
    for line in sys.stdin:
        if "=" in line:
            key, value = line.rstrip("\n").split("=", 1)
            data[key] = value
    return data

def redact(text: str) -> str:
    text = re.sub(r"(https?://)[^/@\\s]+@", r"\\1[redacted]@", text)
    text = re.sub(r"github_pat_[A-Za-z0-9_]+", "[redacted-token]", text)
    text = re.sub(r"gh[pousr]_[A-Za-z0-9_]+", "[redacted-token]", text)
    text = re.sub(r"(?i)(token|secret|password|passwd|api[_-]?key)=\\S+", r"\\1=[redacted-secret]", text)
    if len(text) > MAX_OUTPUTO:
        text = text[:MAX_OUTPUT] + "\n[output truncated by phase1 git plugin]\n"
    return text

def safe_name(raw: str) -> str:
    if not raw or raw in {".", ".."}:
        raise ValueError(f"invalid repository name: {raw}")
    if not re.fullmatch(r"[A-Za-z0-9._-]+", raw):
        raise ValueError(f"invalid repository name: {raw}")
    return raw

def repo_path(name: str | None) -> Path:
    if not name:
        return Path.cwd()
    return REPOS / safe_name(name)

def run_git(args: list[str], cwd: Path | None = None) -> str:
    env = os.environ.copy()
    env.update({
        "GIT_TERMINAL_PROMPT": "0",
        "GCM_INTERACTIVE": "never",
        "GIT_ASKPASS": "true",
        "SSH_ASKPASS": "true",
        "NO_COLOR": "1",
    })
    proc = subprocess.run(
        ["git", *args],
        cwd=str(cwd) if cwd else None,
        stdin=subprocess.DEVNULL,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        timeout=TIMEOUT_SECONDS,
        env=env,
        check=False,
    )
    out = proc.stdout + proc.stderr
    return redact(out if out.strip() else "ok\n")

def help_text() -> str:
    return """'
"phase1 git // guarded read-only operator bridge

usage:
  git status [repo]
  git log [repo]
  git remote [repo]
  git list

scope:
  read-only only. clone, pull, push, fetch, commit, and checkout are not implemented here yet.

storage:
  named repositories are read from phase1.workspace/repos unless PHASE1_STORAGE_ROOT is set
'"""

def list_repos() -> str:
    if not REPOS.exists():
        return "git: no repositories in phase1.workspace/repos\n"
    names = sorted(path.name for path in REPOS.iterdir() if path.is_dir())
    return "git: no repositories\n" if not names else "\n".join(names) + "\n"

def main() -> int:
    context = parse_context()
    try:
        argv = shlex.split(context.get("ARGS", ""))
    except ValueError as err:
        print(f"git: could not parse arguments: {err}")
        return 1

    action = argv[0] if argv else "help"
    try:
        if action in {"help", "-h", "--help"}:
            print(help_text(), end="")
        elif action in {"list", "ls"}:
            print(list_repos(), end="")
        elif action == "status":
            print(run_git(["status", "--short", "--branch"], repo_path(argv[1] if len(argv) > 1 else None)), end="")
        elif action == "log":
            print(run_git(["log", "--oneline", "-n", "12"], repo_path(argv[1] if len(argv) > 1 else None)), end="")
        elif action == "remote":
            print(run_git(["remote", "-v"], repo_path(argv[1] if len(argv) > 1 else None)), end="")
        elif action in {"clone", "pull", "push", "fetch", "commit", "checkout"}:
            raise ValueError(f"git {action}: mutating Git actions are not implemented in this read-only bridge yet")
        else:
            raise ValueError(f"unknown git action: {action}\n\n{help_text()}")
    except Exception as err:
        print(redact(str(err)))
        return 1
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
