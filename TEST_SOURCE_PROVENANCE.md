# Test source provenance

This repository is an isolated public canary owned by
`gha-indie-worker-test`. It is not a deployment source and must not receive
production credentials or production network authority.

- Canonical source: `gha-indie-worker/gha-clone-server.rs`
- Imported branch: `dev`
- Imported commit: `9f71a571dd38c098127ec366c98a9f721fabfd67`
- Test-only additions: the fail-closed test-organization manifest and its
  digest-bound isolation workflow, plus a narrow compiler canary that accepts
  only the empty `permissions: {}` mapping while keeping every broader
  workflow permission form fail-closed

Execution through the custom clone/worker lane must bind this repository to a
full immutable commit SHA, an exact allowlisted workflow path, and a fixed
reviewed profile.
