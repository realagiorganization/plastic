#!/usr/bin/env bash
set -euo pipefail

WORKFLOW="${1:-rust.yml}"
BRANCH="${2:-master}"

if ! command -v gh >/dev/null 2>&1; then
  echo "GitHub CLI (gh) is required" >&2
  exit 1
fi

RUN_ID="$(gh run list --workflow "$WORKFLOW" --branch "$BRANCH" --limit 1 --json databaseId --jq '.[0].databaseId // ""')"

if [[ -z "$RUN_ID" ]]; then
  echo "No runs found for workflow $WORKFLOW on branch $BRANCH" >&2
  exit 1
fi

OUT_DIR="artifacts/${WORKFLOW%.*}-latest"
mkdir -p "$OUT_DIR"

gh run download "$RUN_ID" --dir "$OUT_DIR"
gh run view "$RUN_ID" --log > "$OUT_DIR/run-$RUN_ID.log"

env RUN_ID="$RUN_ID" WORKFLOW="$WORKFLOW" BRANCH="$BRANCH" gh run view "$RUN_ID" --json headSha,displayTitle,status,conclusion,createdAt,updatedAt,url --jq '
  [
    "Run ID: " + (env.RUN_ID // ""),
    "Workflow: " + (env.WORKFLOW // ""),
    "Branch: " + (env.BRANCH // ""),
    "Title: " + (.displayTitle // ""),
    "Head SHA: " + (.headSha // ""),
    "Status: " + (.status // ""),
    "Conclusion: " + (.conclusion // ""),
    "Created: " + (.createdAt // ""),
    "Updated: " + (.updatedAt // ""),
    "URL: " + (.url // "")
  ] | .[]
' > "$OUT_DIR/run-$RUN_ID.txt"
