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
            "next_milestone": "wire safe Fyr black_arts staged runtime stubs while preserving non-live defaults",
        },
        {
            "name": "Fyr native language",
            "estimated_completion_percent": 58,
            "state": "seed language and toolchain surface exist with F3/F4/F5/F6 evidence, runtime-safety fixtures, standard-library contracts, and black_arts staged-candidate design/operator-visual/source-wiring handoff evidence",
            "next_milestone": "implement the first safe fyr staged runtime stub from issue #317 without candidate writes, host commands, network access, validation execution, promotion, discard, or live-system changes",
        },
        {
            "name": "Base1 secure host / OS track",
            "estimated_completion_percent": 40,
            "state": "B2 focused test-suite evidence passed, reviewed B3 VM evidence is present, and the B6 X200 marker chain is published through evidence, checkpoint, public status, and release note; claim remains not_claimed",
            "next_milestone": "continue B4 recovery validation and repeatable physical boot evidence while preserving installer, hardening, release-candidate, and daily-driver non-claims",
        },
        {
            "name": "X200 / Libreboot hardware path",
            "estimated_completion_percent": 44,
            "state": "X200 Linux-libre host generated reviewed B3 VM evidence and B6 marker evidence with phase1_marker_seen; the checkpoint and release note are published; repeatable physical boot validation remains separate",
            "next_milestone": "capture repeatable physical boot evidence and keep emulator, USB, recovery, installer, and hardware-readiness claims separated",
        },
        {
            "name": "Security and crypto policy",
            "estimated_completion_percent": 55,
            "state": "trust model, crypto policy roadmap, provider registry, profile docs, config schema, and integrity checks are present",
            "next_milestone": "move from documentation policy into scoped implementation only after tests and review evidence",
        },
        {
            "name": "Website and public docs",
            "estimated_completion_percent": 90,
            "state": "public site, status page, status JSON, badge endpoint, native GitHub Wiki, refreshed source wiki, organized docs, X200 evidence report, B6 checkpoint trail, Base1 B6 X200 release note, and Fyr black_arts public status trail are in place",
            "next_milestone": "keep the public status synchronized with implementation evidence and non-claims",
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
    b6_checkpoint = {
        "name": "B6 X200 marker checkpoint",
        "path": "docs/checkpoints/B6_X200_MARKER_CHECKPOINT.md",
        "checkpoint_commit": "d4cd1e13d429662f6713466f57a41233d8238416",
        "source_commit": "8eeca92294e8fc67437b46f4cb38917a4428e219",
        "final_evidence_anchor": "095786e808d3908d27c045f04f3de0b5cd538ab9",
        "artifact_sha256": "688518c1437003c7b8325b1d5d479bc97f77c3404c8fd27dace6d823d406b79b",
        "result": "phase1_marker_seen",
        "claim": "not_claimed",
        "release_note_path": "docs/base1/releases/RELEASE_BASE1_B6_X200_MARKER_CHECKPOINT_V1.md",
        "release_note_commit": "c7853f4b6f944b0e496d6a34ed49422fe6a090e7",
        "public_status_commit": "f23fcb822f9c0d11bcd0b07adf71c811fcfd99c1",
        "release_note_status": "published",
        "non_claims": [
            "not installer-ready",
            "not recovery-complete",
            "not hardened",
            "not release-candidate ready",
            "not daily-driver ready",
            "not broad hardware validation",
        ],
    }

    return {
        "name": "Phase1 public project status",
        "status_kind": "estimated roadmap progress",
        "last_updated_utc": now,
        "repository": "Bryforge/phase1",
        "branch": os.environ.get("GITHUB_REF_NAME", "edge/stable"),
        "commit": git_sha(),
        "overall_estimated_completion_percent": overall,
        "public_state": "active edge development with reviewed B3 VM evidence, B6 X200 marker evidence, public status promotion, Base1 B6 X200 marker checkpoint release note, and Fyr black_arts staged-candidate evidence published",
        "important_boundary": "Percentages are planning estimates, not production-readiness, security-hardening, live-update, or autonomous self-modification claims. Fyr black_arts has staged-candidate design, fixtures, operator visuals, source-wiring checklist, and runtime-stub handoff evidence, but the first safe runtime stub remains pending under issue #317. Installer readiness, recovery-complete status, hardening, release-candidate readiness, daily-driver readiness, broad hardware validation, live-system mutation, and autonomous promotion remain not claimed.",
        "projects": projects,
        "repository_organization_metrics": repo_metrics,
        "non_claims": [
            "not a production operating system",
            "not installer-ready",
            "not daily-driver ready",
            "not hardware-validated across targets",
            "not a hardened sandbox",
            "not cryptographically complete",
            "not a live self-updating system",
            "not autonomous promotion or mutation",
            "Fyr black_arts staged-candidate evidence is fixture-backed and design/contract oriented; first safe runtime wiring remains pending under issue #317",
            "B6 X200 marker checkpoint is present with phase1_marker_seen; non-claims remain in force",
            "B6 X200 release note is published; installer, recovery-complete, hardening, release-candidate, daily-driver, and broad hardware-validation claims remain out of scope",
        ],
        "evidence_checkpoints": [b6_checkpoint],
        "current_public_report": {
            "title": "Fyr black_arts staged-candidate evidence and Phase1/Base1 B6 X200 marker checkpoint report",
            "summary": "Fyr black_arts now has staged-candidate design, fixture, visual-mode, checklist, and runtime-stub handoff evidence, while the B6 X200 marker chain remains published through raw evidence, checkpoint, public status, and Base1 checkpoint release note.",
            "release_note_path": "docs/base1/releases/RELEASE_BASE1_B6_X200_MARKER_CHECKPOINT_V1.md",
            "checkpoint_path": "docs/checkpoints/B6_X200_MARKER_CHECKPOINT.md",
            "fyr_black_arts_paths": [
                "docs/fyr/STAGED_CANDIDATES.md",
                "docs/fyr/BLACK_ARTS_OPERATOR_VISUALS.md",
                "docs/fyr/STAGED_SOURCE_WIRING.md",
                "docs/fyr/STAGED_VISUAL_RUNTIME_CHECKLIST.md",
                "docs/fyr/STAGED_RUNTIME_PATCH_HANDOFF.md",
            ],
            "fyr_black_arts_runtime_issue": "https://github.com/Bryforge/phase1/issues/317",
            "marker_result": "phase1_marker_seen",
            "claim": "not_claimed",
            "release_note_commit": "c7853f4b6f944b0e496d6a34ed49422fe6a090e7",
            "public_status_commit": "f23fcb822f9c0d11bcd0b07adf71c811fcfd99c1",
            "checkpoint_commit": "d4cd1e13d429662f6713466f57a41233d8238416",
            "final_evidence_anchor": "095786e808d3908d27c045f04f3de0b5cd538ab9",
            "artifact_sha256": "688518c1437003c7b8325b1d5d479bc97f77c3404c8fd27dace6d823d406b79b",
        },
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
            "## Current public report",
            "",
            "Current report: Fyr black_arts staged-candidate evidence and [`docs/base1/releases/RELEASE_BASE1_B6_X200_MARKER_CHECKPOINT_V1.md`](../../docs/base1/releases/RELEASE_BASE1_B6_X200_MARKER_CHECKPOINT_V1.md)",
            "",
            "Fyr black_arts now has staged-candidate design, fixture, visual-mode, checklist, and runtime-stub handoff evidence. The first safe runtime wiring remains pending under issue #317.",
            "",
            "The B6 X200 marker chain is still published through raw evidence, checkpoint, public status, and Base1 checkpoint release note.",
            "",
            "| Item | Value |",
            "| --- | --- |",
            "| Fyr black_arts runtime issue | `#317` |",
            "| Fyr black_arts design | [`docs/fyr/STAGED_CANDIDATES.md`](../../docs/fyr/STAGED_CANDIDATES.md) |",
            "| Fyr black_arts operator visuals | [`docs/fyr/BLACK_ARTS_OPERATOR_VISUALS.md`](../../docs/fyr/BLACK_ARTS_OPERATOR_VISUALS.md) |",
            "| Fyr black_arts wiring handoff | [`docs/fyr/STAGED_RUNTIME_PATCH_HANDOFF.md`](../../docs/fyr/STAGED_RUNTIME_PATCH_HANDOFF.md) |",
            "| Marker result | `phase1_marker_seen` |",
            "| Claim state | `not_claimed` |",
            "| Checkpoint | [`docs/checkpoints/B6_X200_MARKER_CHECKPOINT.md`](../../docs/checkpoints/B6_X200_MARKER_CHECKPOINT.md) |",
            "| Release note | [`docs/base1/releases/RELEASE_BASE1_B6_X200_MARKER_CHECKPOINT_V1.md`](../../docs/base1/releases/RELEASE_BASE1_B6_X200_MARKER_CHECKPOINT_V1.md) |",
            "| Final evidence anchor | `095786e808d3908d27c045f04f3de0b5cd538ab9` |",
            "| Artifact SHA256 | `688518c1437003c7b8325b1d5d479bc97f77c3404c8fd27dace6d823d406b79b` |",
            "",
            "This report does not claim installer readiness, recovery completion, hardening, release-candidate readiness, daily-driver readiness, broad hardware validation, live-system mutation, or autonomous promotion.",
            "",
            "## Non-claims",
            "",
            "These percentages are planning estimates. They do not claim that Phase1, Base1, or Fyr are production-ready, installer-ready, daily-driver ready, hardware-validated across targets, hardened, cryptographically complete, live-self-updating, or capable of autonomous promotion/mutation.",
            "",
            "Fyr black_arts staged-candidate evidence is fixture-backed and design/contract oriented. The first safe runtime wiring remains pending under issue #317.",
            "",
            "B6 X200 marker evidence is a named marker observation only; it does not claim installer readiness, recovery completion, hardening, release-candidate readiness, daily-driver readiness, or broad hardware validation.",
            "",
        ]
    )
    text = "\n".join(line.rstrip() for line in text.splitlines()) + "\n"
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
