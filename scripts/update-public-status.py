#!/usr/bin/env python3
"""Generate the public Phase1 project status marker from repository state.

This script keeps the public status page data honest by deriving repository
organization signals from the checked-out tree instead of requiring manual edits.
The percentages remain planning estimates and are not production-readiness claims.
"""

from __future__ import annotations

import datetime as _dt
import json
import os
import subprocess
from pathlib import Path
from typing import Iterable

ROOT = Path(__file__).resolve().parents[1]

EXPECTED_ROOT_FILES = {
    ".gitignore",
    ".nojekyll",
    "CONTRIBUTING.md",
    "Cargo.lock",
    "Cargo.toml",
    "LICENSE",
    "README.md",
    "SECURITY.md",
    "deny.toml",
    "phase1",
    "run-phase1-uefi.sh",
    "start_phase1",
}

ROOT_STATUS_DUPLICATES = {
    "status.html",
    "status.json",
    "status-badge.json",
}

GENERATED_ARTIFACT_SUFFIXES = (
    ".img",
    ".raw",
    ".tar",
    ".iso",
)

GENERATED_ARTIFACT_BASENAMES = {
    "BOOTX64.EFI",
    "initrd.img",
    "vmlinuz",
}


def git_lines(*args: str) -> list[str]:
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).splitlines()


def git_sha() -> str:
    return os.environ.get("GITHUB_SHA") or git_lines("rev-parse", "HEAD")[0]


def tracked_files() -> list[str]:
    return sorted(git_lines("ls-files"))


def generated_artifacts(files: Iterable[str]) -> list[str]:
    result: list[str] = []
    for path in files:
        name = Path(path).name
        if path.startswith("build/"):
            result.append(path)
            continue
        if path.startswith("docs/base1/evidence/build-artifacts/"):
            result.append(path)
            continue
        if name in GENERATED_ARTIFACT_BASENAMES:
            result.append(path)
            continue
        if path.endswith(GENERATED_ARTIFACT_SUFFIXES) and (
            path.startswith("build/") or path.startswith("docs/base1/evidence/")
        ):
            result.append(path)
    return sorted(result)


def repo_organization(files: list[str]) -> tuple[int, str, dict[str, object]]:
    root_files = sorted(path for path in files if "/" not in path)
    root_dirs = sorted({path.split("/", 1)[0] for path in files if "/" in path})
    unplanned_root_files = sorted(set(root_files) - EXPECTED_ROOT_FILES)
    missing_expected_root_files = sorted(EXPECTED_ROOT_FILES - set(root_files))
    root_status_duplicates = sorted(set(root_files) & ROOT_STATUS_DUPLICATES)
    tracked_build_files = sorted(path for path in files if path.startswith("build/"))
    tracked_root_site_files = sorted(path for path in files if path.startswith("root-site/"))
    tracked_generated_artifacts = generated_artifacts(files)

    penalty = 0
    penalty += 4 * len(unplanned_root_files)
    penalty += 2 * len(missing_expected_root_files)
    penalty += 8 * len(root_status_duplicates)
    penalty += min(30, 2 * len(tracked_build_files))
    penalty += min(12, 3 * len(tracked_root_site_files))
    penalty += min(20, len(tracked_generated_artifacts))
    percent = max(0, min(100, 100 - penalty))

    state = (
        f"minimal root has {len(root_files)} tracked files, "
        f"{len(root_dirs)} top-level folders, "
        f"{len(unplanned_root_files)} unplanned root files, "
        f"{len(tracked_build_files)} tracked build files, and "
        f"{len(root_status_duplicates)} root status duplicates"
    )

    metrics: dict[str, object] = {
        "root_files": root_files,
        "root_file_count": len(root_files),
        "root_directories": root_dirs,
        "root_directory_count": len(root_dirs),
        "expected_root_files": sorted(EXPECTED_ROOT_FILES),
        "unplanned_root_files": unplanned_root_files,
        "missing_expected_root_files": missing_expected_root_files,
        "tracked_build_file_count": len(tracked_build_files),
        "tracked_root_site_file_count": len(tracked_root_site_files),
        "root_status_duplicates": root_status_duplicates,
        "generated_artifact_count": len(tracked_generated_artifacts),
    }
    return percent, state, metrics


def project_rows(repo_percent: int, repo_state: str) -> list[dict[str, object]]:
    return [
        {
            "name": "Phase1 operator console",
            "estimated_completion_percent": 82,
            "state": "usable edge console with guarded host access, VFS, dashboards, help UI, themes, learning, and tests",
            "next_milestone": "polish release-facing flows and keep safe defaults simple",
        },
        {
            "name": "Fyr native language",
            "estimated_completion_percent": 44,
            "state": "seed language and toolchain surface exist with scripts, tests, assertions, package checks, and docs",
            "next_milestone": "expand language book, package workflow, and runtime integration",
        },
        {
            "name": "Base1 secure host / OS track",
            "estimated_completion_percent": 31,
            "state": "documentation, dry-run scripts, release archives, validation gates, recovery planning, and x86_64 boot planning exist",
            "next_milestone": "advance from dry-run/read-only evidence to VM and hardware validation",
        },
        {
            "name": "X200 / Libreboot hardware path",
            "estimated_completion_percent": 38,
            "state": "USB staging, framebuffer proof paths, recovery notes, and safety gates exist; hardware success remains evidence-bound",
            "next_milestone": "capture repeatable physical boot evidence without strengthening claims early",
        },
        {
            "name": "Security and crypto policy",
            "estimated_completion_percent": 55,
            "state": "trust model, crypto policy roadmap, provider registry, profile docs, config schema, and integrity checks are present",
            "next_milestone": "move from documentation policy into scoped implementation only after tests and review evidence",
        },
        {
            "name": "Website and public docs",
            "estimated_completion_percent": 78,
            "state": "public site, status page, status JSON, badge endpoint, organized docs, asset policy, and Pages routing are in place",
            "next_milestone": "keep the Pages workflow generating public status from repository state",
        },
        {
            "name": "Repository organization",
            "estimated_completion_percent": repo_percent,
            "state": repo_state,
            "next_milestone": "keep generated artifacts out of Git and keep compatibility links clean as work lands",
        },
    ]


def build_status() -> dict[str, object]:
    files = tracked_files()
    repo_percent, repo_state, repo_metrics = repo_organization(files)
    projects = project_rows(repo_percent, repo_state)
    overall = round(
        sum(int(project["estimated_completion_percent"]) for project in projects) / len(projects)
    )
    now = _dt.datetime.now(_dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")
    return {
        "name": "Phase1 public project status",
        "status_kind": "estimated roadmap progress",
        "last_updated_utc": now,
        "repository": "Bryforge/phase1",
        "branch": os.environ.get("GITHUB_REF_NAME", "edge/stable"),
        "commit": git_sha(),
        "overall_estimated_completion_percent": overall,
        "public_state": "active edge development",
        "important_boundary": "Percentages are planning estimates, not production-readiness or security-hardening claims.",
        "projects": projects,
        "repository_organization_metrics": repo_metrics,
        "non_claims": [
            "not a production operating system",
            "not installer-ready",
            "not daily-driver ready",
            "not hardware-validated across targets",
            "not a hardened sandbox",
            "not cryptographically complete",
        ],
    }


def write_json(path: Path, data: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2) + "\n")


def write_markdown(status: dict[str, object]) -> None:
    rows = []
    for project in status["projects"]:  # type: ignore[index]
        item = project  # type: ignore[assignment]
        rows.append(
            "| {name} | {pct}% | {state} | {next} |".format(
                name=item["name"],
                pct=item["estimated_completion_percent"],
                state=item["state"],
                next=item["next_milestone"],
            )
        )

    metrics = status["repository_organization_metrics"]  # type: ignore[index]
    text = "\n".join(
        [
            "# Public project status",
            "",
            "Status kind: estimated roadmap progress  ",
            "Source marker: [`site/status.json`](../../site/status.json)  ",
            "Badge marker: [`site/status-badge.json`](../../site/status-badge.json)  ",
            f"Generated from commit: `{status['commit']}`  ",
            f"Last updated UTC: `{status['last_updated_utc']}`",
            "",
            "## Current estimate",
            "",
            "| Project | Estimated completion | Current state | Next milestone |",
            "| --- | ---: | --- | --- |",
            *rows,
            "",
            f"Overall estimated roadmap completion: **{status['overall_estimated_completion_percent']}%**.",
            "",
            "## Repository organization inputs",
            "",
            f"- Root tracked files: {metrics['root_file_count']}",
            f"- Top-level tracked directories: {metrics['root_directory_count']}",
            f"- Unplanned root files: {len(metrics['unplanned_root_files'])}",
            f"- Tracked build files: {metrics['tracked_build_file_count']}",
            f"- Root status duplicates: {len(metrics['root_status_duplicates'])}",
            f"- Generated artifact count: {metrics['generated_artifact_count']}",
            "",
            "## How to check it publicly",
            "",
            "```text",
            "https://bryforge.github.io/phase1/status.html",
            "https://bryforge.github.io/phase1/status.json",
            "https://bryforge.github.io/phase1/status-badge.json",
            "```",
            "",
            "## Non-claims",
            "",
            "These percentages are planning estimates. They do not claim that Phase1, Base1, or Fyr are production-ready, installer-ready, daily-driver ready, hardware-validated across targets, hardened, or cryptographically complete.",
            "",
        ]
    )
    (ROOT / "docs/status/PROJECT_STATUS.md").write_text(text)


def main() -> None:
    status = build_status()
    overall = status["overall_estimated_completion_percent"]
    write_json(ROOT / "site/status.json", status)
    write_json(
        ROOT / "site/status-badge.json",
        {
            "schemaVersion": 1,
            "label": "Phase1 status",
            "message": f"{overall}% roadmap",
            "color": "00d8ff",
            "namedLogo": "github",
            "cacheSeconds": 300,
        },
    )
    write_markdown(status)
    print(f"public status updated: {overall}% roadmap")


if __name__ == "__main__":
    main()
