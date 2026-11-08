# Alpha release process

The current version is `0.1.0-alpha.1` and has not yet been published. Merge stacked PRs from the bottom up, retargeting the next layer's base to main before each merge. Confirm that the final main includes every layer, then run release validation against that commit.

## Validate the candidate commit

1. Confirm that the working tree is clean, the version matches the CHANGELOG, and CI passes MSRV/stable, 32/64-bit differential, macOS/Windows, PostgreSQL, and documentation checks.
2. Run `python3 tools/check-release-matrix.py`, the full differential suite, and the independent oracle. Trigger the extended randomized workflow manually and save the seed and result artifacts.
3. Run `cargo doc --locked -p rs-decimal --all-features --no-deps` and `cargo test --locked -p rs-decimal --all-features --doc`. docs.rs uses all-features so that every optional entry point appears in the documentation.
4. Run `cargo package --locked --list` to inspect package contents, followed by `cargo publish --locked -p rs-decimal --dry-run --all-features --registry crates-io`. Test the unpacked artifact and examples/configuration.rs to catch dependencies on repository-relative paths.
5. Run `python3 tools/check-consumer.py` to validate dependency resolution and all features using a standalone edition 2024 consumer without a lockfile. MSRV CI runs the same command with Rust 1.85. The consumer uses Cargo resolver 3 with Rust-version-aware dependency resolution.

## Publish after a maintainer's decision

Recheck the rs-decimal name on crates.io and the publishing account's permissions. A dry run neither reserves the name nor proves upload permissions. After selecting the source commit and release notes, publish and create the `v0.1.0-alpha.1` tag/GitHub Release at the same commit. Do not commit credentials or publish from unvalidated stacked branches. Current CI validates only; it does not upload.

References: [Cargo publishing documentation](https://doc.rust-lang.org/cargo/reference/publishing.html), [docs.rs metadata](https://docs.rs/about/metadata).

Public name check on 2026-11-07: the crates.io API returned 404 for rs-decimal (unregistered at that time). This does not reserve the name or establish any account's upload permissions.
