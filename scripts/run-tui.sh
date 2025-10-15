#!/usr/bin/env bash
set -euo pipefail

IMAGE="${1:-ghcr.io/realagiorganization/plastic:tui-latest}"
shift || true

docker run --rm -it "$IMAGE" "$@"
