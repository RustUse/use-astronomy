# Contributing

RustUse/use-astronomy is intentionally small. Contributions should favor durable vocabulary, clear naming, strong documentation, and minimal surface area over broad feature count.

For routing and organization-wide policy, use the RustUse defaults for [support](https://github.com/RustUse/.github/blob/main/SUPPORT.md), [security](https://github.com/RustUse/.github/blob/main/SECURITY.md), and the [code of conduct](https://github.com/RustUse/.github/blob/main/CODE_OF_CONDUCT.md), alongside `GOVERNANCE.md` and `MAINTAINERS.md`.

## Development Flow

1. Make the smallest useful change that improves one crate or one workflow.
2. Add or update tests for every public type or validation rule you change.
3. Keep dependencies lightweight unless the workspace would clearly be worse without them.
4. Preserve the primitive astronomy boundary: no orbital simulators, ephemeris engines, telescope-control systems, observatory runtimes, rendering stacks, astronomy database clients, network fetchers, or data-reduction pipelines.

## Local Validation

```sh
cargo fmt --all -- --check
cargo check --workspace --all-features
cargo check --workspace --all-features --examples
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
cargo audit
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo doc --workspace --all-features --no-deps
```

## Tooling Shortcuts

The repository ships cross-platform Cargo aliases in `.cargo/config.toml`:

```sh
cargo xcheck
cargo xlint
cargo xtest
cargo xtest-minimal
cargo xexamples
cargo xdoc
```

VS Code users also get checked-in task definitions in `.vscode/tasks.json` and extension recommendations in `.vscode/extensions.json`.

## Optional Dev Tool Bootstrap

Optional Cargo tooling used by local release and advisory flows can be installed with either bootstrap script:

```sh
bash scripts/bootstrap-dev-tools.sh
pwsh -File scripts/bootstrap-dev-tools.ps1
```

These scripts install `cargo-deny`, `cargo-audit`, `cargo-cyclonedx`, `release-plz`, and `cargo-machete`.

## Documentation

- Update the root README when the crate list or facade story changes.
- Keep crate README examples small and runnable.
- Keep docs aligned with the current astronomy scope and non-goals.
- Follow `CRATE_TEMPLATE.md` when introducing or expanding a focused crate.

## Release Policy

- The workspace-level default keeps `publish = false`, while the focused crate manifests and the `use-astronomy` facade manifest opt in with `publish = true`.
- Publish focused crates before the `use-astronomy` facade.
- Versions move in lockstep at `0.x.y` for now.
- Until `1.0`, breaking API changes should bump the minor version and compatible additive changes should bump the patch version.
- `Cargo.lock` is committed intentionally for reproducible CI, security checks, and release dry runs in this library workspace.

Use `docs/maintainer-release-flow.md` for the maintainer review sequence around release PRs, changelog cleanup, and the manual publish dispatch step.
