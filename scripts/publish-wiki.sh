#!/usr/bin/env bash
set -euo pipefail

REPO_SLUG="${PHASE1_REPO_SLUG:-Bryforge/phase1}"
WIKI_URL="https://github.com/${REPO_SLUG}.wiki.git"
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WIKI_SOURCE="${ROOT_DIR}/docs/wiki"
WORK_DIR="${PHASE1_WIKI_WORKDIR:-${ROOT_DIR}/../phase1.wiki}"

if [[ ! -d "${WIKI_SOURCE}" ]]; then
  echo "phase1 wiki publish: missing docs/wiki source at ${WIKI_SOURCE}" >&2
  exit 1
fi

if [[ -e "${WORK_DIR}" && ! -d "${WORK_DIR}/.git" ]]; then
  cat >&2 <<EOF
phase1 wiki publish: ${WORK_DIR} exists but is not a git checkout.

Move or remove that directory, then run this script again:

  rm -rf "${WORK_DIR}"
  bash scripts/publish-wiki.sh
EOF
  exit 1
fi

if [[ ! -d "${WORK_DIR}/.git" ]]; then
  echo "phase1 wiki publish: cloning ${WIKI_URL}"
  if ! git clone "${WIKI_URL}" "${WORK_DIR}"; then
    cat >&2 <<EOF
phase1 wiki publish: unable to clone ${WIKI_URL}

GitHub may not have native Wiki support initialized for this repository yet.
Enable Wiki support in repository settings or create the first page in the
GitHub web UI, then run this script again.
EOF
    exit 1
  fi
fi

rsync -av --delete --exclude='.git/' "${WIKI_SOURCE}/" "${WORK_DIR}/"
cd "${WORK_DIR}"

git status --short
if [[ -z "$(git status --short)" ]]; then
  echo "phase1 wiki publish: no wiki changes to publish"
  exit 0
fi

git add .
git commit -m "Update Phase1 user manual"
git push origin master

echo "phase1 wiki publish: published docs/wiki to ${WIKI_URL}"
