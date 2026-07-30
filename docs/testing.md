# Testing

Tests are organized by responsibility:

- Unit tests stay next to the implementation.
- Public API tests stay in each crate's `tests/` directory.
- Cross-crate tests belong to `tests/integration`.
- Version and data compatibility tests belong to `tests/compatibility`.
- Cross-cutting performance tests belong to `benchmarks`.
- Fuzz targets belong to the excluded `fuzz` package.

Fixture files should be small, deterministic, and documented. Generated test
data must be written to a temporary directory and must not modify checked-in
fixtures.

Every pull request runs formatting, compilation, Clippy, tests, and documentation
generation. Slower coverage, dependency, and fuzz checks run separately.
