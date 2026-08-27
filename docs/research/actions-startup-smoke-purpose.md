# Temporary startup smoke

This file accompanies `.github/workflows/startup-smoke.yml` while isolating a GitHub Actions `startup_failure` that occurs before any job is created.

The smoke workflow intentionally has no external Actions and only one shell step. It will be removed after the startup layer is characterized.
