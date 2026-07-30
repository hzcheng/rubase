# Contributing

## Development setup

Install `rustup`, clone the repository, and run:

```bash
cargo xtask ci
```

The pinned toolchain and required components are declared in
`rust-toolchain.toml`.

## Change expectations

- Keep changes focused and add tests for observable behavior.
- Run `cargo xtask ci` before opening a pull request.
- Update documentation when changing public APIs or repository conventions.
- Record significant engineering decisions in `docs/adr/`.
- Do not introduce `unsafe` code without first changing the workspace policy
  through an approved architecture decision record.

Commit messages should use the Conventional Commits form when practical, for
example `build: add release validation`.
