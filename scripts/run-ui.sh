#!/usr/bin/env bash
set -euo pipefail

IMAGE="${1:-ghcr.io/realagiorganization/plastic:ui-latest}"
shift || true

docker run --rm "$IMAGE" "$@"
