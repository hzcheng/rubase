# Development

## Toolchain

The repository pins its Rust version and required components in
`rust-toolchain.toml`. Do not override that version in individual crates.

## Repository commands

Use `cargo xtask` so local and CI behavior remain aligned:

```bash
cargo xtask check
cargo xtask test
cargo xtask ci
```

Third-party dependency versions belong in `[workspace.dependencies]` in the root
manifest. Workspace members should inherit package metadata and lint settings.

Generated files belong under `target/`, `dist/`, or `coverage/` and must not be
committed.
