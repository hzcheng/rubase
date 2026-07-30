# Repository architecture

The repository is a Cargo workspace with a deliberately small public surface.

- `crates/rubase` is the public library facade.
- `crates/rubase-cli` contains the command-line binary.
- `tests/` contains cross-crate and compatibility test packages.
- `benchmarks/` contains cross-cutting performance measurements.
- `tools/xtask` provides platform-independent repository automation.
- `fuzz/` and `packaging/` are reserved for tooling that is not part of the
  default workspace build.

New crates should be introduced only after their ownership, dependency direction,
and independent testing or build value are clear. Internal crates should set
`publish = false`; external users should normally depend only on `rubase`.
