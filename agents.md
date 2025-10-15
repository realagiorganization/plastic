# Agents

This document records the automated processes and agents that keep the project
healthy. Update it when adding or retiring automation.

## Continuous Integration

- **Rust workflow** (`.github/workflows/rust.yml`): Builds and tests the project on
  Linux and Windows, gathers coverage, and publishes build artifacts for review.
- **Container workflow** (`.github/workflows/container.yml`): Builds Docker images
  for the GUI (`ui`) and TUI (`tui`) variants and publishes them to GHCR with
  release and commit tags.

## Release Automation

- **GitHub Releases**: Tagging a commit triggers the release job that publishes
  Debian packages and attaches the license to the release assets.

## Observability

- **Codecov**: Coverage reports generated in CI are uploaded to Codecov to track
  test coverage over time.
- Use `scripts/download-latest-artifacts.sh` to grab the most recent workflow
  artifacts and logs when triaging issues locally.

## Manual Tasks

- Add yourself here with contact details and responsibilities when you start
  looking after the project.
