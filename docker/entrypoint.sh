#!/usr/bin/env bash
set -euo pipefail

variant="${PLASTIC_VARIANT:-ui}"

case "${variant}" in
  ui|plastic)
    exec plastic "$@"
    ;;
  tui|plastic_tui)
    exec plastic_tui "$@"
    ;;
  *)
    echo "Unknown PLASTIC_VARIANT '${variant}'. Use 'ui' or 'tui'." >&2
    exit 1
    ;;
esac
