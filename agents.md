# Agents

This document records the automated processes and agents that keep the project
healthy. Update it when adding or retiring automation.

## Continuous Integration

- **Rust workflow** (`.github/workflows/rust.yml`): Builds and tests the project on
  Linux and Windows, gathers coverage, and publishes build artifacts for review.

## Release Automation

- **GitHub Releases**: Tagging a commit triggers the release job that publishes
  Debian packages and attaches the license to the release assets.

## Observability

- **Codecov**: Coverage reports generated in CI are uploaded to Codecov to track
  test coverage over time.

## Manual Tasks

- Add yourself here with contact details and responsibilities when you start
  looking after the project.
