# Releasing

Releases are created from signed or protected tags named `vX.Y.Z`.

Before tagging:

1. Update the workspace version and `CHANGELOG.md`.
2. Run `cargo xtask release-check`.
3. Confirm the working tree is clean.
4. Review the generated package file list.

The release workflow builds the CLI on supported platforms, publishes checksums,
and creates a hosting-platform release. Publishing crates to a registry is a
separate, manually approved operation.

Do not publish from an unverified local working tree. Release credentials must be
stored in the hosting platform's protected release environment.
