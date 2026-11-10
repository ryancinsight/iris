# Milestone ladder ([ADR 0003](../docs/adr/0003-release-scope-1-0.md))
<a id="M-correctness"></a> **M-correctness** (0.1.5) needs: (none) — correctness repairs.
- exit: IRIS-016 resolved with landed value-semantic evidence; `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets --all-features -- -D warnings`, `cargo nextest run --locked --all-features`, `cargo test --locked --doc --all-features`, and `cargo doc --locked --no-deps --all-features` pass; no open correctness items on M-correctness.
<a id="M-contract-verification"></a> **M-contract-verification** (0.2.0) needs: M-correctness — contract and verification.
- exit: IRIS-006, IRIS-010, IRIS-011, IRIS-014, and IRIS-015 resolved or explicitly re-scoped with ADR evidence; `cargo clippy --locked --all-targets --all-features -- -D warnings`, `cargo nextest run --locked --all-features`, and `cargo test --locked --doc --all-features` pass; no open correctness items.
<a id="M-1.0"></a> **M-1.0** (1.0.0) needs: M-contract-verification — release-ready (authority-gated).
- exit: IRIS-003 release automation complete and trusted-publishing release path verified; `cargo package --locked --no-verify` (or equivalent package gate) succeeds; `cargo semver-checks` release gate passes for the declared release; README/book and rustdoc are current; no open correctness items.
