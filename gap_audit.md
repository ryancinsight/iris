# Iris gap audit

## Provider boundary

Iris owns domain-neutral visualization contracts: normalized color values,
named color maps, fixed lookup tables, borrowed series and scalar-field views,
and the render-backend seam. It does not own domain interpretation, file
formats, GPU mechanics, or consumer-specific scalar-range semantics.

## IRIS-003 — Crates.io release automation

The registry package identity is `iris-viz`; the Rust library and import path
remain `iris`. The release artifact prescription is
`crate-iris-viz-v<version>`. `README.md`, `CHANGELOG.md`, `backlog.md`, and
`checklist.md` now use that split consistently.

The audited pre-change provider default was
`bc47ce37376127b65c064b93dac256555f44ff65`. Hosted provider CI passes at that
exact revision in run `31865323610`.

### External release blocker

The crates.io trusted-publishing configuration for `ryancinsight/iris` is not
present. Hosted release run `31462641512`, targeting the earlier release
revision `ab3eea284e41a9649941253a7b7a69c839907fb3`, passed package validation
but failed while requesting the short-lived OIDC token with HTTP 400:
`No Trusted Publishing config found for repository ryancinsight/iris`.

This is external registry configuration and remains open. The documentation
and PM synchronization does not configure the publisher, enforce
trusted-publishing-only mode, publish `iris-viz`, or verify the crates.io
index. Re-open this item after the exact trusted publisher is configured and a
new `crate-iris-viz-v<version>` release completes validation and publication.

## Local Atlas-overlay limitation

In the dirty Atlas checkout, `cargo fmt --all -- --check` and
`cargo metadata --locked --no-deps --format-version 1` pass. The locked
documentation build is blocked before compilation because the ambient Atlas
path-patch graph causes Cargo to attempt a `Cargo.lock` rewrite under
`--locked`. This is environment/stack resolution evidence, not a provider
source failure.
